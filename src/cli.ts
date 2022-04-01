#!/usr/bin/env node
"use strict";

import { ExtensionReporter } from ".";
import arg from "arg";
import pc from "picocolors";
import { table, getBorderCharacters } from "table";

const args = arg({
  "--help": Boolean,
  "-h": "--help",
  "--ci": Boolean,
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
    const t: string = table([header, ...tableData], {
      border: getBorderCharacters("void"),
      drawHorizontalLine: () => {
        return true;
      },
    });

    console.log(t, "\n\n");
  }
}

main();
