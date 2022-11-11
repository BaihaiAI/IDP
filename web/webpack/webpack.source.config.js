const path = require('path')
const webpack = require('webpack')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const { getThemeVariables } = require("antd/dist/theme");
const WebpackBar = require('webpackbar');
const webpackConfig = require('../config/webpack-config');
const rescriptsrc = require('../config/rescriptsrc');

module.exports = {
    mode: 'production',
    entry: {
        idpStudio: './src/public-source.jsx'
    },
    output: {
        filename: 'index.js',
        path: path.join(__dirname, '../dist_source'),
        ...rescriptsrc.webpack(),
        libraryExport: "default"
    },
    devtool: false,
    resolve: {
        alias: webpackConfig.getAlias(),
        extensions: webpackConfig.getExtensions()
    },
    externals: {
        'react': {
            commonjs: 'react',
            commonjs2: 'react',
            amd: 'react',
            root: 'React',
        },
        'react-dom': {
            commonjs: 'react-dom',
            commonjs2: 'react-dom',
            amd: 'react-dom',
            root: 'ReactDOM',
        },
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
    plugins: [
        new WebpackBar(),
        new CleanWebpackPlugin({
            cleanOnceBeforeBuildPatterns: [path.join(__dirname, '../dist_source/*')]
        })
    ]
}