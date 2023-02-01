#!/usr/bin/env node

import arg from "arg";
import pc from "picocolors";
import { table, getBorderCharacters } from "table";
import { ExtensionReporter } from ".";

const args = arg({
	"--help": Boolean,
	"--ci": Boolean,
	"-h": "--help",
});

function main() {
	let help = `
  Usage
    $ extension-count <input>
    $ extension-count <input> --ci # No colors

  Examples
    $ extension-count ./src
`;

	if (args["--help"] || args["-h"]) {
		console.log(help);

		return;
	}
	const { yellow, blue, bold } = pc.createColors(!args["--ci"]);
	const data = new ExtensionReporter(args._);

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
