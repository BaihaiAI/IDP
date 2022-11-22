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
const CompressionPlugin = require('compression-webpack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const zlib = require("zlib");

const proConfig = {
    mode: 'production',
    devtool: false, // source-map
    plugins: [
        new CleanWebpackPlugin(),
        new UglifyJsPlugin(),
        new MiniCssExtractPlugin({
            filename: 'css/[name].[hash:8].css'
        }),
        new AntdDayjsWebpackPlugin(),
        new CompressionPlugin({
            filename: "[path][name].gz",
            algorithm: "gzip",
            test: /\.(js|jsx|ts|tsx)$/,
            compressionOptions: {
                params: {
                    [zlib.constants.BROTLI_PARAM_QUALITY]: 11,
                },
            },
            threshold: 10240,
            minRatio: 0.8,
            deleteOriginalAssets: false,
        }),
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
    ]
};

module.exports = merge(webpackBaseConfig, proConfig);