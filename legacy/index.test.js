import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, test } from "node:test";
import { ExtensionReporter } from "./index.js";

describe("Files", () => {
	test("From Entries", () => {
		const reporter = new ExtensionReporter(["../fixture"]);
		const obj = reporter.fromEntries(
			new Map([
				["one", 1],
				["two", 2],
			]),
		);

		assert.deepEqual(obj, { one: 1, two: 2 });
		assert.notDeepEqual(obj, { one: 1, two: 2, three: 3 });
	});

	test("makes a table", () => {
		const reporter = new ExtensionReporter(["../fixture"]);
		const firstRow = reporter.result[0].rows[0];

		assert.ok(reporter.result.length > 0);
		assert.ok(firstRow.files.length >= 1);
		assert.ok(firstRow.files.includes("../fixture/index.ts"));
	});

	test("recurses through nested directories and groups matching extensions", () => {
		const dir = mkdtempSync(join(tmpdir(), "file-type-counter-"));

		try {
			mkdirSync(join(dir, "nested"));
			writeFileSync(join(dir, "one.js"), "");
			writeFileSync(join(dir, "nested", "two.js"), "");
			writeFileSync(join(dir, "nested", "three.md"), "");

			const reporter = new ExtensionReporter([dir]);
			const rowsByExtension = Object.fromEntries(
				reporter.result[0].rows.map((row) => [row.extension, row]),
			);

			assert.equal(rowsByExtension[".js"].count, 2);
			assert.equal(rowsByExtension[".md"].count, 1);
			assert.ok(rowsByExtension[".js"].files.includes(`${dir}/one.js`));
			assert.ok(rowsByExtension[".js"].files.includes(`${dir}/nested/two.js`));
		} finally {
			rmSync(dir, { recursive: true, force: true });
		}
	});

	test("truncates long file lists by default", () => {
		const reporter = new ExtensionReporter(["../fixture"]);

		assert.equal(
			reporter.getFileList(["one.js", "two.js", "three.js"], 2),
			"one.js\ntwo.js\n1 more files",
		);
	});

	test("can render file lists without a limit", () => {
		const reporter = new ExtensionReporter(["../fixture"]);

		assert.equal(reporter.getFileList(["one.js", "two.js"], null), "one.js\ntwo.js");
	});
});
