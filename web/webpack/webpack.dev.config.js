const { merge } = require('webpack-merge');
const webpack = require('webpack'); // 用于访问内置插件
const webpackBaseConfig = require('./webpack.base.config');
const rescriptsrc = require('../config/rescriptsrc');
const { createHtmlWebpackPlugin } = require('../config/htmlWebpackPlugin');

const devConfig = {
    mode: 'development',
    devtool: "eval",
    plugins: [
        createHtmlWebpackPlugin({
            options: {
                colorLinkUrl: `//localhost:${rescriptsrc.devServer().port}/static/color.less`,
                loadingGifUrl: `//localhost:${rescriptsrc.devServer().port}/static/loading.gif`,
            },
            env: process.env.NODE_ENV
        }),
        new webpack.HotModuleReplacementPlugin()
    ],
};

module.exports = merge(webpackBaseConfig, devConfig);
