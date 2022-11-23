const { merge } = require('webpack-merge');
const webpack = require('webpack'); // 用于访问内置插件
const webpackBaseConfig = require('./webpack.base.config');
const rescriptsrc = require('../config/rescriptsrc');
const { createHtmlWebpackPlugin } = require('../config/htmlWebpackPlugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const devConfig = {
    mode: 'development',
    devtool: "source-map",
    plugins: [
        createHtmlWebpackPlugin({
            options: {
                colorLinkUrl: `//localhost:${rescriptsrc.devServer().port}/static/color.less`,
                loadingGifUrl: `//localhost:${rescriptsrc.devServer().port}/static/loading.gif`,
            },
            env: process.env.NODE_ENV,
            template: 'open.ejs'
        }),
        new MiniCssExtractPlugin(),
        new webpack.HotModuleReplacementPlugin()
    ],
};

module.exports = merge(webpackBaseConfig, devConfig);
