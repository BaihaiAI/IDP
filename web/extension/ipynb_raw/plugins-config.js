const fs = require("fs");
const path = require('path');
const webpack = require('webpack');

const extensionsEntryPath = './src';
const extensionsConfigFileName = 'config.json';

function initEntry(_extensionsEntryPath, _extensionsConfigFileName) {
    let entry = {};
    const files = fs.readdirSync(path.resolve(__dirname, _extensionsEntryPath), { withFileTypes: true });
    for (let file of files) {
        if (file.name == _extensionsConfigFileName && file.isFile()) {
            const fileConfigPath = `${_extensionsEntryPath}/${_extensionsConfigFileName}`;
            const filesConfig = fs.readFileSync(path.join(__dirname, fileConfigPath), 'utf-8');
            const fileName = JSON.parse(filesConfig).fileName;
            console.log(path.join(__dirname, `${_extensionsEntryPath}/${JSON.parse(filesConfig).entry}`));
            Object.assign(entry, { [fileName]: path.join(__dirname, `${_extensionsEntryPath}/${JSON.parse(filesConfig).entry}`) })
        }
    }
    return entry;
}

/**
 * 读取配置文件
 */
module.exports = {
    webpackEntry: function () {
        return initEntry(extensionsEntryPath, extensionsConfigFileName);
    }
}