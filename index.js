// @ts-check

import * as fs from "node:fs";

/**
 * @typedef Row
 * @type {object}
 * @property {string} extension - file extension
 * @property {number} count
 * @property {string[]} files
 */

/**
 * @typedef TableData
 * @type {object}
 * @property {string} title
 * @property {Row[]} rows
 */

export class ExtensionReporter {
  /**
   * @type {TableData[]}
   */
  #result = [];
  /**
   *
   * @param {string[]} folders
   */
  constructor(folders) {
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

  /**
   *
   * @param folder string
   * @returns Row[]
   */
  #createTable(folder) {
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

  fromEntries(iterable) {
    return [...iterable].reduce((obj, [key, val]) => {
      obj[key] = val;
      return obj;
    }, {});
  }

  /**
   *
   * @param {string} dir
   * @param {any[]} files_
   * @returns
   */
  getFiles(dir, files_ = []) {
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

  /**
   *
   * @param {string[]} files
   * @param {number | null} limit
   * @returns string
   */

  getFileList(files, limit = 10) {
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
