# Legacy Node CLI

This folder contains the original JavaScript implementation of the extension counter.
It is maintained for reference and compatibility only.

## Usage

```
$ npm install
$ npm test
$ npm run build
$ node ./cli.js ../fixture
```

## Notes

- CLI parsing uses `node:util.parseArgs`.
- Styling uses `node:util.styleText`.
- Tests use Node's built-in `node:test` module.
