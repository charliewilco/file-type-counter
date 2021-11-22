import * as fs from "fs";

interface IRow {
  extension: string;
  count: number;
  files: string[];
}

interface ITableData {
  title: string;
  rows: IRow[];
}

export class ExtensionReporter {
  #result: ITableData[];
  constructor(folders: string[]) {
    this.#result = folders.map((folder) => {
      return {
        title: folder,
        rows: this.#createTable(folder),
      };
    });
  }

  get result() {
    return this.#result;
  }

  #createTable(folder: string): IRow[] {
    const re: RegExp = /(?:\.([^.]+))?$/;
    const map = new Map<string, Set<string>>();
    const files = this.getFiles(folder);

    for (let file of files) {
      const extension = re.exec(file)[0];

      if (map.has(extension)) {
        const set = map.get(extension);
        set.add(file);
      } else {
        map.set(extension, new Set([file]));
      }
    }

    const data = this.fromEntries(map);

    return Object.keys(data).map<IRow>((key) => {
      return {
        extension: key,
        count: data[key].size,
        files: Array.from(data[key]),
      };
    });
  }

  fromEntries<U>(iterable: Map<string, U>): Record<string, U> {
    return [...iterable].reduce((obj: Record<string, U>, [key, val]) => {
      obj[key] = val;
      return obj;
    }, {});
  }

  getFiles(dir: string, files_?: any[]) {
    files_ = files_ || [];
    var files = fs.readdirSync(dir);
    for (var i in files) {
      var name = dir + "/" + files[i];
      if (fs.statSync(name).isDirectory()) {
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
