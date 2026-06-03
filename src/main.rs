use clap::{Parser, ValueEnum};
use extension_count::{ExtensionReporter, FileRow, OutputTable, ReporterOptions};
use serde::Serialize;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "extension-count",
    version,
    about = "Count file extensions in one or more directories"
)]
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

    /// Disable extension labels
    #[arg(long, conflicts_with = "labels")]
    no_labels: bool,

    /// Load extension labels from a custom JSON file
    #[arg(long)]
    labels: Option<PathBuf>,

    /// Disable .gitignore, hidden-directory, and common build-directory filtering
    #[arg(long)]
    no_ignore: bool,

    /// Render table output with an ASCII border
    #[arg(long)]
    bordered: bool,

    /// Hide file paths and only show extension counts
    #[arg(long)]
    summary: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum SortKey {
    Count,
    Ext,
    Files,
}

#[derive(Debug, Clone, Copy)]
enum Align {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Cell {
    text: String,
    width: usize,
}

#[derive(Debug, Clone)]
struct RenderableTable {
    headers: Vec<Cell>,
    rows: Vec<Vec<Cell>>,
    alignments: Vec<Align>,
}

#[derive(Debug, Serialize)]
struct SummaryOutputTable {
    title: String,
    total_files: usize,
    rows: Vec<SummaryFileRow>,
}

#[derive(Debug, Serialize)]
struct SummaryFileRow {
    extension: String,
    label: Option<String>,
    count: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let reporter = ExtensionReporter::with_options(
        args.inputs.clone(),
        ReporterOptions {
            labels_path: args.labels.clone(),
            use_labels: !args.no_labels,
            respect_ignore: !args.no_ignore,
        },
    )?;

    if args.json {
        let mut results = reporter.results;
        sort_output_tables(&mut results, &args);
        let json = if args.summary {
            serde_json::to_string_pretty(&summarize_results(&results))
        } else {
            serde_json::to_string_pretty(&results)
        }
        .map_err(io::Error::other)?;
        println!("{json}");
        return Ok(());
    }

    for table in reporter.results {
        render_table(table, &args);
        println!();
    }

    Ok(())
}

fn sort_output_tables(results: &mut [OutputTable], args: &Args) {
    for table in results {
        sort_rows(&mut table.rows, args.sort, args.reverse);
    }
}

fn render_table(table: OutputTable, args: &Args) {
    let color = !args.ci;
    let title = yellow(&table.title, color);
    println!("Results for: {title}");
    println!(
        "Total files: {}",
        bold(&table.total_files.to_string(), color)
    );

    let mut rows = table.rows;
    sort_rows(&mut rows, args.sort, args.reverse);

    let table = renderable_table(&rows, args, color);
    if args.bordered {
        println!("{}", format_bordered_table(&table));
    } else {
        println!("{}", format_plain_table(&table));
    }
}

fn sort_rows(rows: &mut [FileRow], key: SortKey, reverse: bool) {
    rows.sort_by(|a, b| {
        let ordering = match key {
            SortKey::Count => b
                .count
                .cmp(&a.count)
                .then_with(|| a.extension.cmp(&b.extension)),
            SortKey::Ext => a
                .extension
                .cmp(&b.extension)
                .then_with(|| b.count.cmp(&a.count)),
            SortKey::Files => b
                .files
                .len()
                .cmp(&a.files.len())
                .then_with(|| a.extension.cmp(&b.extension)),
        };
        if reverse {
            ordering.reverse()
        } else {
            ordering
        }
    });
}

fn renderable_table(rows: &[FileRow], args: &Args, color: bool) -> RenderableTable {
    let mut headers = vec![plain_cell("Extension"), plain_cell("Count")];
    let mut alignments = vec![Align::Left, Align::Right];
    if !args.summary {
        headers.push(plain_cell("Files"));
        alignments.push(Align::Left);
    }

    let mut rendered_rows = Vec::new();
    for row in rows {
        let extension = row_label(row);
        let count = row.count.to_string();

        if args.summary {
            rendered_rows.push(vec![
                styled_cell(extension, blue(extension, color)),
                styled_cell(&count, bold(&count, color)),
            ]);
            continue;
        }

        let files_lines = display_files(row, args);
        if files_lines.is_empty() {
            rendered_rows.push(vec![
                styled_cell(extension, blue(extension, color)),
                styled_cell(&count, bold(&count, color)),
                plain_cell(""),
            ]);
            continue;
        }

        for (index, file) in files_lines.iter().enumerate() {
            if index == 0 {
                rendered_rows.push(vec![
                    styled_cell(extension, blue(extension, color)),
                    styled_cell(&count, bold(&count, color)),
                    plain_cell(file),
                ]);
            } else {
                rendered_rows.push(vec![plain_cell(""), plain_cell(""), plain_cell(file)]);
            }
        }
    }

    RenderableTable {
        headers,
        rows: rendered_rows,
        alignments,
    }
}

fn format_plain_table(table: &RenderableTable) -> String {
    let widths = column_widths(table);
    let mut lines = Vec::new();
    lines.push(format_plain_row(&table.headers, &widths, &table.alignments));
    lines.push(
        widths
            .iter()
            .map(|width| "-".repeat(*width))
            .collect::<Vec<_>>()
            .join("  "),
    );

    for row in &table.rows {
        lines.push(format_plain_row(row, &widths, &table.alignments));
    }

    lines.join("\n")
}

fn format_bordered_table(table: &RenderableTable) -> String {
    let widths = column_widths(table);
    let border = format_border(&widths);
    let mut lines = Vec::new();
    lines.push(border.clone());
    lines.push(format_bordered_row(
        &table.headers,
        &widths,
        &table.alignments,
    ));
    lines.push(border.clone());

    for row in &table.rows {
        lines.push(format_bordered_row(row, &widths, &table.alignments));
    }

    lines.push(border);
    lines.join("\n")
}

fn column_widths(table: &RenderableTable) -> Vec<usize> {
    let mut widths: Vec<usize> = table.headers.iter().map(|cell| cell.width).collect();
    for row in &table.rows {
        for (index, cell) in row.iter().enumerate() {
            widths[index] = widths[index].max(cell.width);
        }
    }
    widths
}

fn format_plain_row(row: &[Cell], widths: &[usize], alignments: &[Align]) -> String {
    row.iter()
        .enumerate()
        .map(|(index, cell)| format_cell(cell, widths[index], alignments[index]))
        .collect::<Vec<_>>()
        .join("  ")
}

fn format_bordered_row(row: &[Cell], widths: &[usize], alignments: &[Align]) -> String {
    let cells = row
        .iter()
        .enumerate()
        .map(|(index, cell)| format!(" {} ", format_cell(cell, widths[index], alignments[index])))
        .collect::<Vec<_>>();
    format!("|{}|", cells.join("|"))
}

fn format_border(widths: &[usize]) -> String {
    format!(
        "+{}+",
        widths
            .iter()
            .map(|width| "-".repeat(width + 2))
            .collect::<Vec<_>>()
            .join("+")
    )
}

fn format_cell(cell: &Cell, width: usize, alignment: Align) -> String {
    let padding = width.saturating_sub(cell.width);
    match alignment {
        Align::Left => format!("{}{}", cell.text, " ".repeat(padding)),
        Align::Right => format!("{}{}", " ".repeat(padding), cell.text),
    }
}

fn display_files(row: &FileRow, args: &Args) -> Vec<String> {
    if row.files.is_empty() {
        return Vec::new();
    }

    let limit = if args.limit == 0 {
        None
    } else {
        Some(args.limit)
    };
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

    lines
}

fn row_label(row: &FileRow) -> &str {
    match row.label.as_deref() {
        Some(label) => label,
        None if row.extension.is_empty() => "(none)",
        None => &row.extension,
    }
}

fn plain_cell(input: &str) -> Cell {
    Cell {
        text: input.to_string(),
        width: input.len(),
    }
}

fn styled_cell(raw: &str, styled: String) -> Cell {
    Cell {
        text: styled,
        width: raw.len(),
    }
}

fn summarize_results(results: &[OutputTable]) -> Vec<SummaryOutputTable> {
    results
        .iter()
        .map(|table| SummaryOutputTable {
            title: table.title.clone(),
            total_files: table.total_files,
            rows: table
                .rows
                .iter()
                .map(|row| SummaryFileRow {
                    extension: row.extension.clone(),
                    label: row.label.clone(),
                    count: row.count,
                })
                .collect(),
        })
        .collect()
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
            no_labels: false,
            labels: None,
            no_ignore: false,
            bordered: false,
            summary: false,
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
        let lines = display_files(&row, &args);

        assert_eq!(lines, vec!["a.rs", "b.rs", "2 more files"]);
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
        let lines = display_files(&row, &args);
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
    fn row_label_displays_no_extension() {
        let row = FileRow {
            extension: String::new(),
            label: None,
            count: 1,
            files: vec!["README".to_string()],
        };

        assert_eq!(row_label(&row), "(none)");
    }

    #[test]
    fn format_bordered_table_renders_ascii_border() {
        let table = RenderableTable {
            headers: vec![plain_cell("Extension"), plain_cell("Count")],
            rows: vec![vec![plain_cell(".rs"), plain_cell("2")]],
            alignments: vec![Align::Left, Align::Right],
        };

        assert_eq!(
            format_bordered_table(&table),
            [
                "+-----------+-------+",
                "| Extension | Count |",
                "+-----------+-------+",
                "| .rs       |     2 |",
                "+-----------+-------+",
            ]
            .join("\n")
        );
    }

    #[test]
    fn summary_json_omits_files() {
        let results = vec![OutputTable {
            title: "fixture".to_string(),
            total_files: 1,
            rows: vec![FileRow {
                extension: ".rs".to_string(),
                label: Some("Rust".to_string()),
                count: 1,
                files: vec!["fixture/main.rs".to_string()],
            }],
        }];

        let json = serde_json::to_string(&summarize_results(&results)).expect("json");
        assert!(json.contains("\"count\":1"));
        assert!(!json.contains("fixture/main.rs"));
        assert!(!json.contains("\"files\""));
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
