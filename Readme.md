# Extension Count
![CI](https://github.com/charliewilco/file-type-counter/actions/workflows/ci.yml/badge.svg)

Fast, colorful CLI that counts file extensions across one or more paths.

```
Usage
    $ extension-count <input>...

Examples
    $ extension-count ./src
    $ extension-count ./src ./test
    $ extension-count ./src --limit 0
    $ extension-count ./src --json
    $ extension-count ./src --summary
    $ extension-count ./src --bordered
    $ extension-count ./src --sort ext --reverse
```

## Install

### From source (Rust)
```
$ cargo build --release
$ ./target/release/extension-count ./src
```

## Options
- `--ci` disable ANSI colors
- `--json` emit JSON instead of a table
- `--limit <n>` limit file list per extension (0 = unlimited)
- `--sort <count|ext|files>` sort rows (default: count)
- `--reverse` reverse sort order
- `--summary` hide file paths and only show extension counts
- `--bordered` render table output with an ASCII border
- `--labels <path>` load extension labels from a custom JSON file
- `--no-labels` disable extension labels
- `--no-ignore` include files ignored by `.gitignore`, hidden-directory, and common build-directory filters

## Help output
```
$ extension-count --help
Count file extensions in one or more directories

Usage: extension-count [OPTIONS] <INPUTS>...

Arguments:
  <INPUTS>...  Folders or files to scan

Options:
      --ci               Disable ANSI colors
      --json             Emit JSON instead of a table
      --limit <LIMIT>    Limit the number of files listed per extension (0 = unlimited) [default: 10]
      --sort <SORT>      Sort rows by count, extension, or file path count [default: count] [possible values: count, ext, files]
      --reverse          Reverse row order
      --no-labels        Disable extension labels
      --labels <LABELS>  Load extension labels from a custom JSON file
      --no-ignore        Disable .gitignore, hidden-directory, and common build-directory filtering
      --bordered         Render table output with an ASCII border
      --summary          Hide file paths and only show extension counts
  -h, --help             Print help
  -V, --version          Print version
```

## JSON output
```
$ extension-count ./src --json
```

Produces an array of per-path objects:
- `title` scanned path
- `total_files` total files found
- `rows` list of `{ extension, label, count, files }`

Use `--summary --json` to omit file paths from JSON rows.

## Ignore behavior

By default, `extension-count` respects `.gitignore` files, skips hidden paths,
and skips common dependency/build directories such as `node_modules` and `target`.
Use `--no-ignore` to include those files.

## Labels

Extension labels live in `labels.json` at the repo root. Keys are extensions without
the leading dot. Update this file to control how labels appear in CLI output.
Use `--labels <path>` to load labels from another file or `--no-labels` to show
raw extensions.

## Development
```
$ cargo test
```

## Legacy Node implementation

The original JavaScript CLI is preserved in `legacy/` and uses Node for tests.

```
$ cd legacy
$ npm install
$ npm test
$ npm run build
```

Legacy CLI entrypoint: `legacy/cli.js`
