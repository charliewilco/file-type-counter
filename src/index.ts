import { readdirSync, statSync } from "node:fs";

interface FileReporterRow {
	extension: string;
	count: number;
	files: string[];
}

interface OutputTableData {
	title: string;
	rows: FileReporterRow[];
}

export class ExtensionReporter {
	private _result: OutputTableData[] = [];
	constructor(folders: string[]) {
		this._result = folders.map((folder) => {
			return {
				title: folder,
				rows: this.createTable(folder),
			};
		});
	}

	get result() {
		return this._result;
	}

	private createTable(folder: string): FileReporterRow[] {
		const re = /(?:\.([^.]+))?$/;
		const map = new Map();
		const files = this.getFiles(folder);

		for (let file of files) {
			const extension = re.exec(file)?.[0];

			if (map.has(extension)) {
				const set = map.get(extension);
				set.add(file);
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

	fromEntries(iterable: Map<string, Set<string>>): Record<string, Set<string>> {
		return [...iterable].reduce<Record<string, Set<string>>>((obj, [key, val]) => {
			obj[key] = val;
			return obj;
		}, {});
	}

	getFiles(dir: string, files_: any[] = []): string[] {
		files_ = files_ || [];
		var files = readdirSync(dir);
		for (var i in files) {
			var name = dir + "/" + files[i];
			if (statSync(name).isDirectory()) {
				this.getFiles(name, files_);
			} else {
				files_.push(name);
			}
		}
		return files_;
	}

	getFileList(files: string[], limit: number | null = 10): string {
		if (limit !== null && files.length > limit) {
			return files
				.slice(0, limit)
				.join("\n")
				.concat(`\n${files.length - limit} more files`);
		} else {
			return files.join("\n");
		}
	}
}
