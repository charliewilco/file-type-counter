use serde::Serialize;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct FileRow {
    pub extension: String,
    pub label: Option<String>,
    pub count: usize,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OutputTable {
    pub title: String,
    pub total_files: usize,
    pub rows: Vec<FileRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExtensionReporter {
    pub results: Vec<OutputTable>,
}

impl ExtensionReporter {
    pub fn new(paths: Vec<PathBuf>) -> io::Result<Self> {
        let labels = load_labels()?;
        let mut results = Vec::with_capacity(paths.len());
        for path in paths {
            let files = collect_files(&path)?;
            let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for file in &files {
                let ext = extension_string(file);
                grouped.entry(ext).or_default().push(file.display().to_string());
            }

            let rows: Vec<FileRow> = grouped
                .into_iter()
                .map(|(extension, files)| FileRow {
                    label: label_for_extension(&labels, &extension),
                    extension,
                    count: files.len(),
                    files,
                })
                .collect();

            let total_files = files.len();
            results.push(OutputTable {
                title: path.display().to_string(),
                total_files,
                rows,
            });
        }

        Ok(Self { results })
    }
}

fn load_labels() -> io::Result<BTreeMap<String, String>> {
    let path = match std::env::current_dir() {
        Ok(dir) => dir.join("labels.json"),
        Err(_) => return Ok(BTreeMap::new()),
    };

    if !path.exists() {
        return Ok(BTreeMap::new());
    }

    let contents = fs::read_to_string(&path)?;
    let labels = serde_json::from_str(&contents)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    Ok(labels)
}

fn label_for_extension(labels: &BTreeMap<String, String>, extension: &str) -> Option<String> {
    if extension.is_empty() {
        return None;
    }

    let key = extension.trim_start_matches('.').to_ascii_lowercase();
    labels.get(&key).cloned()
}

pub fn collect_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_files_inner(root, &mut files)?;
    Ok(files)
}

fn collect_files_inner(root: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    let metadata = fs::metadata(root)?;
    if metadata.is_file() {
        files.push(root.to_path_buf());
        return Ok(());
    }

    if metadata.is_dir() {
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            collect_files_inner(&entry.path(), files)?;
        }
    }

    Ok(())
}

fn extension_string(path: &Path) -> String {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) if !ext.is_empty() => format!(".{ext}"),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::sync::{Mutex, OnceLock};
    use tempfile::TempDir;

    fn cwd_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().expect("cwd lock")
    }

    struct CwdGuard {
        original: PathBuf,
    }

    impl CwdGuard {
        fn set(temp: &TempDir) -> Self {
            let original = std::env::current_dir().expect("current dir");
            std::env::set_current_dir(temp.path()).expect("set current dir");
            Self { original }
        }
    }

    impl Drop for CwdGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.original);
        }
    }

    fn write_file(path: &Path, contents: &str) {
        let mut file = File::create(path).expect("create file");
        file.write_all(contents.as_bytes()).expect("write file");
    }

    #[test]
    fn reports_fixture() {
        let _lock = cwd_lock();
        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixture");
        let reporter = ExtensionReporter::new(vec![fixture]).unwrap();
        assert_eq!(reporter.results.len(), 1);
        let table = &reporter.results[0];
        assert!(table.total_files > 0);
        let mut found = false;
        for row in &table.rows {
            if row.files.iter().any(|file| file.contains("fixture/index.ts")) {
                found = true;
            }
        }
        assert!(found);
    }

    #[test]
    fn handles_single_file_input() {
        let temp = TempDir::new().expect("temp dir");
        let _lock = cwd_lock();
        let file_path = temp.path().join("single.rs");
        write_file(&file_path, "fn main() {}");

        let reporter = ExtensionReporter::new(vec![file_path.clone()]).unwrap();
        assert_eq!(reporter.results.len(), 1);
        let table = &reporter.results[0];
        assert_eq!(table.total_files, 1);
        assert_eq!(table.rows.len(), 1);
        assert_eq!(table.rows[0].extension, ".rs");
        assert_eq!(table.rows[0].count, 1);
        assert_eq!(table.rows[0].files[0], file_path.display().to_string());
    }

    #[test]
    fn groups_by_extension_and_counts() {
        let temp = TempDir::new().expect("temp dir");
        let _lock = cwd_lock();
        let dir = temp.path();
        write_file(&dir.join("a.ts"), "const a = 1;");
        write_file(&dir.join("b.ts"), "const b = 2;");
        write_file(&dir.join("c.rs"), "fn main() {}");

        let reporter = ExtensionReporter::new(vec![dir.to_path_buf()]).unwrap();
        let table = &reporter.results[0];

        let mut counts = BTreeMap::new();
        let mut label = None;
        for row in &table.rows {
            counts.insert(row.extension.clone(), row.count);
            if row.extension == ".ts" {
                label = row.label.clone();
            }
        }

        assert_eq!(counts.get(".ts"), Some(&2));
        assert_eq!(counts.get(".rs"), Some(&1));
        assert_eq!(label.as_deref(), Some("TypeScript"));
    }

    #[test]
    fn handles_files_without_extension() {
        let temp = TempDir::new().expect("temp dir");
        let _lock = cwd_lock();
        let dir = temp.path();
        write_file(&dir.join("LICENSE"), "license text");
        write_file(&dir.join("README"), "readme text");

        let reporter = ExtensionReporter::new(vec![dir.to_path_buf()]).unwrap();
        let table = &reporter.results[0];

        let mut no_ext_count = 0;
        for row in &table.rows {
            if row.extension.is_empty() {
                no_ext_count += row.count;
            }
        }

        assert_eq!(no_ext_count, 2);
    }

    #[test]
    fn recurses_nested_directories() {
        let temp = TempDir::new().expect("temp dir");
        let _lock = cwd_lock();
        let nested = temp.path().join("nested/inner");
        fs::create_dir_all(&nested).expect("create nested");
        write_file(&nested.join("deep.txt"), "deep");

        let reporter = ExtensionReporter::new(vec![temp.path().to_path_buf()]).unwrap();
        let table = &reporter.results[0];
        let mut found = false;
        for row in &table.rows {
            if row.files.iter().any(|file| file.contains("deep.txt")) {
                found = true;
            }
        }
        assert!(found);
    }

    #[test]
    fn load_labels_returns_empty_when_missing() {
        let temp = TempDir::new().expect("temp dir");
        let _lock = cwd_lock();
        let _guard = CwdGuard::set(&temp);

        let labels = load_labels().expect("labels");
        assert!(labels.is_empty());
    }

    #[test]
    fn load_labels_reads_json() {
        let temp = TempDir::new().expect("temp dir");
        let _lock = cwd_lock();
        let _guard = CwdGuard::set(&temp);
        write_file(temp.path().join("labels.json").as_path(), r#"{"rs":"Rust","ts":"TypeScript"}"#);

        let labels = load_labels().expect("labels");
        assert_eq!(labels.get("rs").map(String::as_str), Some("Rust"));
        assert_eq!(labels.get("ts").map(String::as_str), Some("TypeScript"));
    }

    #[test]
    fn load_labels_invalid_json_returns_error() {
        let temp = TempDir::new().expect("temp dir");
        let _lock = cwd_lock();
        let _guard = CwdGuard::set(&temp);
        write_file(temp.path().join("labels.json").as_path(), "{");

        let err = load_labels().expect_err("invalid json should error");
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn label_for_extension_normalizes_dot_and_case() {
        let mut labels = BTreeMap::new();
        labels.insert("rs".to_string(), "Rust".to_string());
        labels.insert("ts".to_string(), "TypeScript".to_string());

        assert_eq!(label_for_extension(&labels, ".RS").as_deref(), Some("Rust"));
        assert_eq!(label_for_extension(&labels, "tS").as_deref(), Some("TypeScript"));
        assert_eq!(label_for_extension(&labels, "").as_deref(), None);
    }

    #[test]
    fn extension_string_handles_no_extension() {
        assert_eq!(extension_string(Path::new("README")), "");
        assert_eq!(extension_string(Path::new("main.rs")), ".rs");
    }

    #[test]
    fn collect_files_returns_error_for_missing_path() {
        let missing = Path::new("no_such_path_123");
        assert!(collect_files(missing).is_err());
    }

    #[test]
    fn rows_are_sorted_by_extension() {
        let temp = TempDir::new().expect("temp dir");
        let _lock = cwd_lock();
        let dir = temp.path();
        write_file(&dir.join("b.zzz"), "b");
        write_file(&dir.join("a.aaa"), "a");

        let reporter = ExtensionReporter::new(vec![dir.to_path_buf()]).unwrap();
        let table = &reporter.results[0];
        let extensions: Vec<&str> = table.rows.iter().map(|row| row.extension.as_str()).collect();
        assert_eq!(extensions, vec![".aaa", ".zzz"]);
    }
}
