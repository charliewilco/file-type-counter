import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { describe, test } from "node:test";

describe("CLI", () => {
	test("prints a bordered table for a directory", () => {
		const output = execFileSync(
			process.execPath,
			["./cli.js", "../fixture", "--ci"],
			{
				encoding: "utf8",
			},
		);

		assert.match(output, /Results for:\s+..\/fixture/);
		assert.match(output, /\+-----------\+------------\+---------------------\+/);
		assert.match(output, /\| Extension \| File Count \| Files\s+\|/);
		assert.match(output, /\| \.ts\s+\| 1\s+\| ..\/fixture\/index\.ts \|/);
	});

	test("prints help", () => {
		const output = execFileSync(process.execPath, ["./cli.js", "--help"], {
			encoding: "utf8",
		});

		assert.match(output, /Usage/);
		assert.match(output, /extension-count <input>/);
	});
});
