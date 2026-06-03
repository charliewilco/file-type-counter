import { readdirSync, statSync } from "node:fs";

/**
 * @typedef {object} FileReporterRow
 * @property {string} extension
 * @property {number} count
 * @property {string[]} files
 */

/**
 * @typedef {object} OutputTableData
 * @property {string} title
 * @property {FileReporterRow[]} rows
 */

export class ExtensionReporter {
	/** @type {OutputTableData[]} */
	#result = [];

	/**
	 * @param {string[]} folders
	 */
	constructor(folders) {
		this.#result = folders.map((folder) => {
			return {
				title: folder,
				rows: this.createTable(folder),
			};
		});
	}

	/**
	 * @returns {OutputTableData[]}
	 */
	get result() {
		return this.#result;
	}

	/**
	 * @param {string} folder
	 * @returns {FileReporterRow[]}
	 */
	createTable(folder) {
		const extensionPattern = /(?:\.([^.]+))?$/;
		const map = new Map();
		const files = this.getFiles(folder);

		for (const file of files) {
			const extension = extensionPattern.exec(file)?.[0];
			const existingFiles = map.get(extension);

			if (existingFiles) {
				existingFiles.add(file);
			} else {
				map.set(extension, new Set([file]));
			}
		}

		const data = this.fromEntries(map);

		return Object.keys(data).map((key) => {
			return {
				extension: key,
				count: data[key].size,
				files: Array.from(data[key]),
			};
		});
	}

	/**
	 * @template T
	 * @param {Map<string, T>} iterable
	 * @returns {Record<string, T>}
	 */
	fromEntries(iterable) {
		return Object.fromEntries(iterable);
	}

	/**
	 * @param {string} dir
	 * @param {string[]} [files]
	 * @returns {string[]}
	 */
	getFiles(dir, files = []) {
		const entries = readdirSync(dir);

		for (const entry of entries) {
			const name = `${dir}/${entry}`;

			if (statSync(name).isDirectory()) {
				this.getFiles(name, files);
			} else {
				files.push(name);
			}
		}

		return files;
	}

	/**
	 * @param {string[]} files
	 * @param {number | null} [limit]
	 * @returns {string}
	 */
	getFileList(files, limit = 10) {
		if (limit !== null && files.length > limit) {
			return files
				.slice(0, limit)
				.join("\n")
				.concat(`\n${files.length - limit} more files`);
		}

		return files.join("\n");
	}
}
