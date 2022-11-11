const path = require('path')
const webpack = require('webpack')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const { getThemeVariables } = require("antd/dist/theme");
const idpPluginsConfig = require('../config/plugins-config');
const WebpackBar = require('webpackbar');
const webpackConfig = require('../config/webpack-config');

const entryPlugins = idpPluginsConfig.webpackEntry();

module.exports = {
    mode: 'production',
    entry: entryPlugins,
    devtool: false,
    resolve: {
        alias: webpackConfig.getAlias(),
        extensions: webpackConfig.getExtensions()
    },
    performance: webpackConfig.getPerformance(),
    module: {
        rules: [
            webpackConfig.loadJsxOrTsxRules(),
            webpackConfig.loadJsRules(),
            webpackConfig.loadCssRules(),
            webpackConfig.loadUrlRules(),
            webpackConfig.loadLessRules(),
            webpackConfig.loadModuleLessRules()
        ]
    },
    output: {
        filename: '[name]/index.js',
        path: path.join(__dirname, '../dist_plugins'),
        // 链接库输出方式 默认'var'形式赋给变量
        libraryTarget: 'umd',
        // 全局变量名称 导出库将被以var的形式赋给这个全局变量 通过这个变量获取到里面模块
        library: '[name]'
    },

    plugins: [
        new WebpackBar(),
        new CleanWebpackPlugin({
            cleanOnceBeforeBuildPatterns: [path.join(__dirname, '../dist_plugins/*')]
        })
    ]
}