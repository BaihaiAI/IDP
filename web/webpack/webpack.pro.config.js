const { merge } = require('webpack-merge');
const webpack = require('webpack');
const path = require('path');
const webpackBaseConfig = require('./webpack.base.config');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const UglifyJsPlugin = require('uglifyjs-webpack-plugin');
const ScriptExtHtmlWebpackPlugin = require("script-ext-html-webpack-plugin");
const AddAssetHtmlWebpackPlugin = require('add-asset-html-webpack-plugin');
const AntdDayjsWebpackPlugin = require('antd-dayjs-webpack-plugin');
const { createHtmlWebpackPlugin } = require('../config/htmlWebpackPlugin');
const idpPluginsConfig = require('../config/plugins-config');



const dllReferencePlugin = process.env.NODE_PLUG === 'true' ? idpPluginsConfig.dllReferencePluginExtensions() : [];
const addAssetHtmlWebpackPlugin = process.env.NODE_PLUG === 'true' ? idpPluginsConfig.addAssetHtmlWebpackPluginExtensions() : [];

const proConfig = {
    mode: 'production',
    devtool: false, // source-map
    plugins: [
        new CleanWebpackPlugin({
            cleanOnceBeforeBuildPatterns: [path.join(__dirname, '!../dist/extension/**/*')]
        }),
        new UglifyJsPlugin(),
        new AntdDayjsWebpackPlugin(),
        new ScriptExtHtmlWebpackPlugin({
            inline: /runtime\..*\.js$/
        }),
        new webpack.DllReferencePlugin({
            context: path.join(__dirname),
            manifest: path.join(__dirname, `../dll/vendor.manifest.json`)
        }),
        new AddAssetHtmlWebpackPlugin({
            filepath: path.resolve(__dirname, '../dll/vendor.dll.js'),
            outputPath: '../dist/js',
            publicPath: `/child/idpStudio-idp/js`
        }),
        createHtmlWebpackPlugin({ env: process.env.NODE_ENV })
    ].concat(...dllReferencePlugin, ...addAssetHtmlWebpackPlugin)
};

module.exports = merge(webpackBaseConfig, proConfig);