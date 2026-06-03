/**
 * @param {string[][]} rows
 * @returns {string}
 */
export function formatTable(rows) {
	if (rows.length === 0) {
		return "";
	}

	const widths = rows[0].map((_, columnIndex) => {
		return Math.max(...rows.map((row) => row[columnIndex].length));
	});

	const border = formatBorder(widths);
	const formattedRows = rows.map((row) => formatRow(row, widths));
	const [header, ...body] = formattedRows;

	return [border, header, border, ...body, border].join("\n");
}

/**
 * @param {number[]} widths
 * @returns {string}
 */
function formatBorder(widths) {
	return `+${widths.map((width) => "-".repeat(width + 2)).join("+")}+`;
}

/**
 * @param {string[]} row
 * @param {number[]} widths
 * @returns {string}
 */
function formatRow(row, widths) {
	const cells = row.map((cell, columnIndex) => {
		return ` ${cell.padEnd(widths[columnIndex])} `;
	});

	return `|${cells.join("|")}|`;
}
