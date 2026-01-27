use clap::{Parser, ValueEnum};
use extension_count::{ExtensionReporter, FileRow, OutputTable};
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "extension-count", version, about = "Count file extensions in one or more directories")]
struct Args {
    /// Folders or files to scan
    #[arg(required = true)]
    inputs: Vec<PathBuf>,

    /// Disable ANSI colors
    #[arg(long)]
    ci: bool,

    /// Emit JSON instead of a table
    #[arg(long)]
    json: bool,

    /// Limit the number of files listed per extension (0 = unlimited)
    #[arg(long, default_value = "10")]
    limit: usize,

    /// Sort rows by count, extension, or file path count
    #[arg(long, value_enum, default_value = "count")]
    sort: SortKey,

    /// Reverse row order
    #[arg(long)]
    reverse: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum SortKey {
    Count,
    Ext,
    Files,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let reporter = ExtensionReporter::new(args.inputs.clone())?;

    if args.json {
        let json = serde_json::to_string_pretty(&reporter.results)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        println!("{json}");
        return Ok(());
    }

    for table in reporter.results {
        render_table(table, &args);
        println!();
    }

    Ok(())
}

fn render_table(table: OutputTable, args: &Args) {
    let color = !args.ci;
    let title = yellow(&table.title, color);
    println!("Results for: {title}");
    println!("Total files: {}", bold(&table.total_files.to_string(), color));

    let mut rows = table.rows;
    sort_rows(&mut rows, args.sort, args.reverse);

    let header = ["Extension", "Count", "Files"];
    let mut col_widths = [header[0].len(), header[1].len(), header[2].len()];
    for row in &rows {
        col_widths[0] = col_widths[0].max(row_label(row).len());
        col_widths[1] = col_widths[1].max(row.count.to_string().len());
        col_widths[2] = col_widths[2].max(display_files(row, args).0);
    }

    println!(
        "{:<w0$}  {:>w1$}  {:<w2$}",
        header[0],
        header[1],
        header[2],
        w0 = col_widths[0],
        w1 = col_widths[1],
        w2 = col_widths[2]
    );
    println!(
        "{:-<w0$}  {:-<w1$}  {:-<w2$}",
        "",
        "",
        "",
        w0 = col_widths[0],
        w1 = col_widths[1],
        w2 = col_widths[2]
    );

    for row in rows {
        print_row(&row, col_widths, args, color);
    }
}

fn sort_rows(rows: &mut [FileRow], key: SortKey, reverse: bool) {
    rows.sort_by(|a, b| {
        let ordering = match key {
            SortKey::Count => b.count.cmp(&a.count).then_with(|| a.extension.cmp(&b.extension)),
            SortKey::Ext => a.extension.cmp(&b.extension).then_with(|| b.count.cmp(&a.count)),
            SortKey::Files => b.files.len().cmp(&a.files.len()).then_with(|| a.extension.cmp(&b.extension)),
        };
        if reverse {
            ordering.reverse()
        } else {
            ordering
        }
    });
}

fn print_row(row: &FileRow, widths: [usize; 3], args: &Args, color: bool) {
    let (files_width, files_lines) = display_files(row, args);
    let count = bold(&row.count.to_string(), color);
    let extension = blue(row_label(row), color);

    if files_lines.is_empty() {
        println!(
            "{:<w0$}  {:>w1$}  {:<w2$}",
            extension,
            count,
            "",
            w0 = widths[0],
            w1 = widths[1],
            w2 = widths[2]
        );
        return;
    }

    for (index, line) in files_lines.iter().enumerate() {
        if index == 0 {
            println!(
                "{:<w0$}  {:>w1$}  {:<w2$}",
                extension,
                count,
                line,
                w0 = widths[0],
                w1 = widths[1],
                w2 = widths[2].max(files_width)
            );
        } else {
            println!(
                "{:<w0$}  {:>w1$}  {:<w2$}",
                "",
                "",
                line,
                w0 = widths[0],
                w1 = widths[1],
                w2 = widths[2].max(files_width)
            );
        }
    }
}

fn display_files(row: &FileRow, args: &Args) -> (usize, Vec<String>) {
    if row.files.is_empty() {
        return (0, Vec::new());
    }

    let limit = if args.limit == 0 { None } else { Some(args.limit) };
    let mut lines = Vec::new();

    match limit {
        Some(limit) if row.files.len() > limit => {
            for file in row.files.iter().take(limit) {
                lines.push(file.clone());
            }
            let remaining = row.files.len() - limit;
            lines.push(format!("{remaining} more files"));
        }
        _ => {
            lines.extend(row.files.iter().cloned());
        }
    }

    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    (width, lines)
}

fn row_label(row: &FileRow) -> &str {
    row.label.as_deref().unwrap_or(&row.extension)
}

fn blue(input: &str, enabled: bool) -> String {
    if enabled {
        format!("\x1b[34m{input}\x1b[0m")
    } else {
        input.to_string()
    }
}

fn yellow(input: &str, enabled: bool) -> String {
    if enabled {
        format!("\x1b[33m{input}\x1b[0m")
    } else {
        input.to_string()
    }
}

fn bold(input: &str, enabled: bool) -> String {
    if enabled {
        format!("\x1b[1m{input}\x1b[0m")
    } else {
        input.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args_with_limit(limit: usize) -> Args {
        Args {
            inputs: Vec::new(),
            ci: true,
            json: false,
            limit,
            sort: SortKey::Count,
            reverse: false,
        }
    }

    #[test]
    fn sort_rows_by_count_then_extension() {
        let mut rows = vec![
            FileRow {
                extension: ".rs".to_string(),
                label: None,
                count: 2,
                files: vec!["a".to_string(), "b".to_string()],
            },
            FileRow {
                extension: ".ts".to_string(),
                label: None,
                count: 2,
                files: vec!["c".to_string(), "d".to_string()],
            },
            FileRow {
                extension: ".md".to_string(),
                label: None,
                count: 3,
                files: vec!["e".to_string(), "f".to_string(), "g".to_string()],
            },
        ];

        sort_rows(&mut rows, SortKey::Count, false);
        let ordered: Vec<&str> = rows.iter().map(|row| row.extension.as_str()).collect();
        assert_eq!(ordered, vec![".md", ".rs", ".ts"]);
    }

    #[test]
    fn sort_rows_by_extension_with_reverse() {
        let mut rows = vec![
            FileRow {
                extension: ".b".to_string(),
                label: None,
                count: 1,
                files: vec!["b".to_string()],
            },
            FileRow {
                extension: ".a".to_string(),
                label: None,
                count: 5,
                files: vec!["a".to_string()],
            },
        ];

        sort_rows(&mut rows, SortKey::Ext, true);
        let ordered: Vec<&str> = rows.iter().map(|row| row.extension.as_str()).collect();
        assert_eq!(ordered, vec![".b", ".a"]);
    }

    #[test]
    fn sort_rows_by_files_len() {
        let mut rows = vec![
            FileRow {
                extension: ".a".to_string(),
                label: None,
                count: 1,
                files: vec!["a".to_string()],
            },
            FileRow {
                extension: ".b".to_string(),
                label: None,
                count: 5,
                files: vec!["b".to_string(), "c".to_string()],
            },
        ];

        sort_rows(&mut rows, SortKey::Files, false);
        assert_eq!(rows[0].extension, ".b");
        assert_eq!(rows[1].extension, ".a");
    }

    #[test]
    fn display_files_respects_limit() {
        let row = FileRow {
            extension: ".rs".to_string(),
            label: None,
            count: 4,
            files: vec![
                "a.rs".to_string(),
                "b.rs".to_string(),
                "c.rs".to_string(),
                "d.rs".to_string(),
            ],
        };
        let args = args_with_limit(2);
        let (width, lines) = display_files(&row, &args);

        assert_eq!(lines, vec!["a.rs", "b.rs", "2 more files"]);
        assert_eq!(width, "2 more files".len());
    }

    #[test]
    fn display_files_unlimited_when_limit_zero() {
        let row = FileRow {
            extension: ".rs".to_string(),
            label: None,
            count: 2,
            files: vec!["a.rs".to_string(), "b.rs".to_string()],
        };
        let args = args_with_limit(0);
        let (_, lines) = display_files(&row, &args);
        assert_eq!(lines, vec!["a.rs", "b.rs"]);
    }

    #[test]
    fn row_label_prefers_label() {
        let row = FileRow {
            extension: ".rs".to_string(),
            label: Some("Rust".to_string()),
            count: 1,
            files: vec!["main.rs".to_string()],
        };
        assert_eq!(row_label(&row), "Rust");
    }

    #[test]
    fn color_helpers_add_escape_sequences() {
        assert_eq!(blue("text", false), "text");
        assert!(blue("text", true).contains("\x1b[34m"));
        assert_eq!(yellow("text", false), "text");
        assert!(yellow("text", true).contains("\x1b[33m"));
        assert_eq!(bold("text", false), "text");
        assert!(bold("text", true).contains("\x1b[1m"));
    }
}
