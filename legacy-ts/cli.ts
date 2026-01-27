#!/usr/bin/env node

import { parseArgs, styleText } from "node:util";
import { table, getBorderCharacters } from "table";
import { ExtensionReporter } from ".";

const rawArgs = typeof Bun !== "undefined" ? Bun.argv : process.argv;
const { values, positionals } = parseArgs({
	args: rawArgs,
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
	let help = `
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
	const yellow = (input: string) =>
		colorEnabled ? styleText("yellow", input) : input;
	const blue = (input: string) => (colorEnabled ? styleText("blue", input) : input);
	const bold = (input: string) => (colorEnabled ? styleText("bold", input) : input);
	const inputs = positionals.slice(2);
	const data = new ExtensionReporter(inputs);

	for (let { rows, title } of data.result) {
		console.log("Results for: ", yellow(title), "\n");
		const tableData = rows.map((row) => {
			return [
				blue(row.extension),
				blue(bold(row.count.toString())),
				data.getFileList(row.files, null),
			];
		});

		const header = ["Extension", "File Count", "Files"];
		const t = table([header, ...tableData], {
			border: getBorderCharacters("void"),
			drawHorizontalLine: () => {
				return true;
			},
		});

		console.log(t, "\n\n");
	}
}

main();
