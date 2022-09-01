const fs = require("fs");
const path = require('path');
const webpack = require('webpack');

const extensionsEntryPath = '../extension';
const extensionsOutPutPath = '../dist/extension';
const extensionsConfigFileName = 'config.json';
const AddAssetHtmlWebpackPlugin = require('add-asset-html-webpack-plugin');

function initEntry(_extensionsEntryPath, _extensionsConfigFileName) {
    let entry = {};
    const files = fs.readdirSync(path.resolve(__dirname, _extensionsEntryPath), { withFileTypes: true });
    for (let file of files) {
        if (file.isDirectory()) {
            const dirS = path.join(`${_extensionsEntryPath}`, file.name);
            const entryFileName = file.name;
            const configFiles = fs.readdirSync(path.resolve(__dirname, dirS), { withFileTypes: true });
            for (const it of configFiles) {
                if (it.name == _extensionsConfigFileName && it.isFile()) {
                    const fileConfigPath = `${dirS}/${_extensionsConfigFileName}`;
                    console.log(fileConfigPath);
                    const filesConfig = fs.readFileSync(path.join(__dirname, fileConfigPath), 'utf-8');
                    Object.assign(entry, { [entryFileName]: [path.join(__dirname, `${_extensionsEntryPath}/${JSON.parse(filesConfig).fileName}/${JSON.parse(filesConfig).entry}`)] })
                }
            }
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
    },
    // pro 执行加载插件
    dllReferencePluginExtensions: function () {
        let plugins = [];
        const files = fs.readdirSync(path.resolve(__dirname, '../dll_plugins'), { withFileTypes: true });
        for (let file of files) {
            if (file.isDirectory()) {
                const dirS = path.join(`../dll_plugins`, file.name);
                const configFiles = fs.readdirSync(path.resolve(__dirname, dirS), { withFileTypes: true });
                for (const it of configFiles) {
                    if (it.name == `${file.name}.manifest.json` && it.isFile()) {
                        const fileConfigPath = `${dirS}/${file.name}.manifest.json`;
                        console.log(fileConfigPath);
                        plugins.push(
                            new webpack.DllReferencePlugin({
                                context: path.join(__dirname),
                                manifest: path.join(__dirname, fileConfigPath)
                            })
                        )
                    }
                }
            }
        }
        return plugins;
    },
    // pro 执行加载插件
    addAssetHtmlWebpackPluginExtensions: function () {
        let plugins = [];
        const files = fs.readdirSync(path.resolve(__dirname, '../dll_plugins'), { withFileTypes: true });
        for (let file of files) {
            if (file.isDirectory()) {
                const dirS = path.join(`../dll_plugins`, file.name);
                console.log(dirS);
                const configFiles = fs.readdirSync(path.resolve(__dirname, dirS), { withFileTypes: true });
                for (const it of configFiles) {
                    if (it.name == `${file.name}.dll_plugins.js` && it.isFile()) {
                        const fileConfigPath = `${dirS}/${file.name}.dll_plugins.js`;
                        console.log(fileConfigPath);
                        plugins.push(
                            new AddAssetHtmlWebpackPlugin({
                                filepath: path.resolve(__dirname, fileConfigPath),
                                outputPath: `../dist/extension/${file.name}`,
                                publicPath: `/child/idpStudio-idp/extension/${file.name}`
                            })
                        )
                    }
                }
            }
        }
        return plugins;
    }
}