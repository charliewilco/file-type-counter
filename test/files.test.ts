import { ExtensionReporter } from "../src";

describe("Files", () => {
  it("From Entries", () => {
    const reporter = new ExtensionReporter(["./src"]);
    const obj = reporter.fromEntries(
      new Map([
        ["one", 1],
        ["two", 2],
      ])
    );

    expect(obj).toMatchObject({ one: 1, two: 2 });
    expect(obj).not.toMatchObject({ one: 1, two: 2, three: 3 });
  });

  it("makes a table", () => {
    // NOTE: this is different in than the import statement
    // because this is running in the root directory

    const reporter = new ExtensionReporter(["./src"]);

    expect(reporter.result.length).toBeGreaterThan(0);
    expect(reporter.result[0].rows[0].files.length).toBeGreaterThan(1);
    expect(reporter.result[0].rows[0].files).toContain("./src/cli.ts");
  });
});
