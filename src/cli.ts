#!/usr/bin/env node
"use strict";

import { extensionReporter } from ".";
import mri from "mri";
import chalk from "chalk";
import { table, getBorderCharacters } from "table";

interface IArguments extends mri.Argv {
  _: string[];
  help: boolean;
  h: boolean;
}

const args = mri(process.argv.slice(2), { boolean: ["h", "help"] }) as IArguments;

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
    console.log("Results for: ", chalk.yellow(d.title), "\n");
    const tableData = d.rows.map(row => {
      return [
        chalk.blue(row.extension),
        chalk.blue.bold(row.count.toString()),
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
