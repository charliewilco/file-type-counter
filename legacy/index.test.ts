import { describe, expect, test } from "bun:test";
import { ExtensionReporter } from ".";

describe("Files", () => {
	test("From Entries", () => {
		const reporter = new ExtensionReporter(["../fixture"]);
		const obj = reporter.fromEntries(
			// @ts-ignore
			new Map([
				["one", 1],
				["two", 2],
			])
		);

		expect(obj).toMatchObject({ one: 1, two: 2 });
		expect(obj).not.toMatchObject({ one: 1, two: 2, three: 3 });
	});

	test("makes a table", () => {
		// NOTE: this is different in than the import statement
		// because this is running in the root directory

		const reporter = new ExtensionReporter(["../fixture"]);

		expect(reporter.result.length).toBeGreaterThan(0);
		expect(reporter.result[0].rows[0].files.length).toBeGreaterThanOrEqual(1);
		expect(reporter.result[0].rows[0].files).toContain("../fixture/index.ts");
	});
});
