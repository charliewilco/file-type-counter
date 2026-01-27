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

## Help output
```
$ extension-count --help
Count file extensions in one or more directories

Usage: extension-count [OPTIONS] <INPUTS>...

Arguments:
  <INPUTS>...  Folders or files to scan

Options:
      --ci           Disable ANSI colors
      --json         Emit JSON instead of a table
      --limit <N>    Limit the number of files listed per extension (0 = unlimited) [default: 10]
      --sort <SORT>  Sort rows by count, extension, or file path count [default: count] [possible values: count, ext, files]
      --reverse      Reverse row order
  -h, --help         Print help
  -V, --version      Print version
```

## JSON output
```
$ extension-count ./src --json
```

Produces an array of per-path objects:
- `title` scanned path
- `total_files` total files found
- `rows` list of `{ extension, label, count, files }`

## Labels

Extension labels live in `labels.json` at the repo root. Keys are extensions without
the leading dot. Update this file to control how labels appear in CLI output.

## Development
```
$ cargo test
```

## Legacy TypeScript implementation

The original TypeScript CLI is preserved in `legacy/` and uses Bun for tests.

```
$ cd legacy
$ bun install
$ bun test
```

Legacy CLI entrypoint: `legacy/src/cli.ts`
