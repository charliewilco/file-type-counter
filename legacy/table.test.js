import assert from "node:assert/strict";
import { describe, test } from "node:test";
import { formatTable } from "./table.js";

describe("formatTable", () => {
	test("renders a bordered table", () => {
		assert.equal(
			formatTable([
				["Extension", "File Count", "Files"],
				[".ts", "1", "../fixture/index.ts"],
			]),
			[
				"+-----------+------------+---------------------+",
				"| Extension | File Count | Files               |",
				"+-----------+------------+---------------------+",
				"| .ts       | 1          | ../fixture/index.ts |",
				"+-----------+------------+---------------------+",
			].join("\n"),
		);
	});

	test("sizes columns to the widest body cell", () => {
		assert.equal(
			formatTable([
				["Name", "Count"],
				["Short", "2"],
				["Much longer", "100"],
			]),
			[
				"+-------------+-------+",
				"| Name        | Count |",
				"+-------------+-------+",
				"| Short       | 2     |",
				"| Much longer | 100   |",
				"+-------------+-------+",
			].join("\n"),
		);
	});

	test("renders empty input as an empty string", () => {
		assert.equal(formatTable([]), "");
	});
});
