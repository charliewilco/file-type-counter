// @bun
// index.ts
import { readdirSync, statSync } from "fs";

class ExtensionReporter {
  _result = [];
  constructor(folders) {
    this._result = folders.map((folder) => {
      return {
        title: folder,
        rows: this.createTable(folder)
      };
    });
  }
  get result() {
    return this._result;
  }
  createTable(folder) {
    const re = /(?:\.([^.]+))?$/;
    const map = new Map;
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
        files: Array.from(data[key])
      };
    });
  }
  fromEntries(iterable) {
    return [...iterable].reduce((obj, [key, val]) => {
      obj[key] = val;
      return obj;
    }, {});
  }
  getFiles(dir, files_ = []) {
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
  getFileList(files, limit = 10) {
    if (limit !== null && files.length > limit) {
      return files.slice(0, limit).join(`
`).concat(`
${files.length - limit} more files`);
    } else {
      return files.join(`
`);
    }
  }
}
export {
  ExtensionReporter
};
