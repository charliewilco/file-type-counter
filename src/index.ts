import * as fs from "fs";

interface IRow {
  extension: string;
  count: number;
  files: string[];
}

interface StringTMap<T> {
  [key: string]: T;
}

export function fromEntries<U>(iterable: Map<string, U>): StringTMap<U> {
  return [...iterable].reduce((obj: StringTMap<U>, [key, val]) => {
    obj[key] = val;
    return obj;
  }, {});
}

export function getFiles(dir: string, files_?: any[]) {
  files_ = files_ || [];
  var files = fs.readdirSync(dir);
  for (var i in files) {
    var name = dir + "/" + files[i];
    if (fs.statSync(name).isDirectory()) {
      getFiles(name, files_);
    } else {
      files_.push(name);
    }
  }
  return files_;
}

export function createTable(folder: string): IRow[] {
  const re: RegExp = /(?:\.([^.]+))?$/;
  const map = new Map<string, Set<string>>();
  const files = getFiles(folder);

  files.forEach(file => {
    const extension = re.exec(file)[0];

    if (map.has(extension)) {
      const set = map.get(extension);
      set.add(file);
    } else {
      map.set(extension, new Set([file]));
    }
  });

  const data = fromEntries(map);

  return Object.keys(data).map<IRow>(key => {
    return {
      extension: key,
      count: data[key].size,
      files: Array.from(data[key])
    };
  });
}

interface ITableData {
  title: string;
  rows: IRow[];
}

export function extensionReporter(folders: string[]): ITableData[] {
  return folders.map((folder: string) => {
    return {
      title: folder,
      rows: createTable(folder)
    };
  });
}
