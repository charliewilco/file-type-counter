#!/usr/bin/env node
"use strict";

import { extensionReporter } from ".";
import * as mri from "mri";
import * as chalk from "chalk";
import { table, getBorderCharacters } from "table";

interface IArguments extends mri.Argv {
  _: string[];
  help: boolean;
  h: boolean;
}

const args = <IArguments>mri(process.argv.slice(2), { boolean: ["h", "help"] });

let help = `
  Usage
    $ extension-count <input>
  Examples
    $ extension-count ./src
`;

function getFileList(files: string[], limit: number | null = 10): string {
  if (limit !== null && files.length > limit) {
    return files
      .slice(0, limit)
      .join("\n")
      .concat(`\n${files.length - limit} more files`);
  } else {
    return files.join("\n");
  }
}

function app() {
  if (args.help || args.h) {
    console.log(help);

    return;
  }

  const data = extensionReporter(args._);

  data.forEach(d => {
    console.log("Results for: ", chalk.default.yellow(d.title), "\n");
    const tableData = d.rows.map(row => {
      return [
        chalk.default.blue(row.extension),
        chalk.default.blue.bold(row.count.toString()),
        getFileList(row.files, null)
      ];
    });

    const header = ["Extension", "File Count", "Files"];
    const t: string = table([header, ...tableData], {
      border: getBorderCharacters("void"),
      drawHorizontalLine: () => {
        return true;
      }
    });

    console.log(t, "\n\n");
  });
}

app();
