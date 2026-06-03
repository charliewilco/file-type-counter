#!/usr/bin/env node

import { parseArgs, styleText } from "node:util";
import { ExtensionReporter } from "./index.js";
import { formatTable } from "./table.js";

const { values, positionals } = parseArgs({
	args: process.argv.slice(2),
	options: {
		help: {
			type: "boolean",
			short: "h",
		},
		ci: {
			type: "boolean",
		},
	},
	strict: false,
	allowPositionals: true,
});

function main() {
	const help = `
  Usage
    $ extension-count <input>
    $ extension-count <input> --ci # No colors

  Examples
    $ extension-count ./src
`;

	if (values.help) {
		console.log(help);

		return;
	}

	const colorEnabled = !values.ci;
	const yellow = (input) => (colorEnabled ? styleText("yellow", input) : input);
	const blue = (input) => (colorEnabled ? styleText("blue", input) : input);
	const bold = (input) => (colorEnabled ? styleText("bold", input) : input);
	const data = new ExtensionReporter(positionals);

	for (const { rows, title } of data.result) {
		console.log("Results for: ", yellow(title), "\n");

		const tableData = rows.map((row) => {
			return [
				blue(row.extension),
				blue(bold(row.count.toString())),
				data.getFileList(row.files, null),
			];
		});

		const output = formatTable([["Extension", "File Count", "Files"], ...tableData]);

		console.log(output, "\n\n");
	}
}

main();
