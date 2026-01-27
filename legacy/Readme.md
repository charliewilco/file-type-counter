# Legacy TypeScript CLI

This folder contains the original TypeScript implementation of the extension counter.
It is maintained for reference and compatibility only.

## Usage
```
$ bun install
$ bun test
$ bun run build
$ node ./dist/cli.js ./fixture
```

## Notes
- CLI parsing uses `Bun.argv` with `node:util.parseArgs`.
- Styling uses `node:util.styleText`.
