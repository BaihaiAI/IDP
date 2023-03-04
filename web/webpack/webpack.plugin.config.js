const webpack = require('webpack'); // 用于访问内置插件
const rescriptsrc = require('../config/rescriptsrc');
const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WebpackBar = require('webpackbar');
const AddAssetHtmlPlugin = require('add-asset-html-webpack-plugin');
const { createHtmlWebpackPlugin } = require('../config/htmlWebpackPlugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const HotModuleReplacementPlugin = require('webpack/lib/HotModuleReplacementPlugin')
const webpackConfig = require('../config/webpack-config');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const publicPath = '/';
const entry = './src/plugin.jsx';
const output = {
    path: path.join(__dirname, "../dist"),
    filename: `js/[name].[hash].js`,
    publicPath,
    ...rescriptsrc.webpack()
}

let REACT_APP_VERSION = '';
function loadEntry() {
    if (process.env.NODE_OPEN === 'true') {
        REACT_APP_VERSION = require('../config/global').REACT_APP_VERSION; // 设置成open
    } else {
        REACT_APP_VERSION = require('../../config/global').REACT_APP_VERSION; // 设置成open
    }
}

console.log('--------webpack dev【studio】启动信息--------');
console.log('@当前启动的版本是: 微应用本地开发需安装插件版');
console.log('@当前启动的js静态引入的path是:', publicPath);
console.log('@当前启动webpack插件列表:', entry);
console.log('@当前启动webpack打包输出模式:', output);
console.log('@当前启动的接口代理信息:', rescriptsrc.devServer().proxy);

const devConfig = {
    mode: 'development',
    devtool: false,
    entry,
    output,
    resolve: {
        alias: webpackConfig.getAlias(),
        extensions: webpackConfig.getExtensions()
    },
    performance: webpackConfig.getPerformance(),
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
    module: {
        rules: [
            webpackConfig.loadJsxOrTsxRules(),
            webpackConfig.loadJsRules(),
            webpackConfig.loadCssRules(),
            webpackConfig.loadLessRules(),
            webpackConfig.loadModuleLessRules(),
            {
                test: /\.(png|jpe?g|gif|svg)(\?.*)?$/,
                use: [
                    {
                        loader: 'url-loader',
                        options: {
                            name: path.join('../dist/img/[name].[hash:7].[ext]'),
                            publicPath: `//localhost:${rescriptsrc.devServer().port}`,
                        }
                    }]
            }
        ]
    },
    plugins: [
        new WebpackBar(),
        new MiniCssExtractPlugin(),
        new webpack.HotModuleReplacementPlugin(),
        new webpack.DefinePlugin({
            'process.env': {
                'NODE': JSON.stringify(process.env.NODE_ENV),
                'REACT_APP_VERSION': JSON.stringify(REACT_APP_VERSION),
                'NODE_OPEN': JSON.stringify(process.env.NODE_OPEN),
                'proxy_target': JSON.stringify(rescriptsrc.target),
                'NODE_PLUGIN': JSON.stringify(process.env.NODE_PLUGIN),
            }
        }),
        new CopyWebpackPlugin({
            patterns: [
                { from: path.resolve(__dirname, '../static'), to: path.resolve(__dirname, '../dist/static') },
                { from: path.resolve(__dirname, '../dist_extension'), to: path.resolve(__dirname, '../dist/extension') }
            ],
        }),
        new AddAssetHtmlPlugin({
            filepath: require.resolve(`../static/less.min.js`),
            publicPath
        }),
        new HotModuleReplacementPlugin(),
        new HtmlWebpackPlugin({
            template: 'index.ejs',
            filename: 'index.html',
            ...createHtmlWebpackPlugin({
                options: {
                    colorLinkUrl: `//localhost:${rescriptsrc.devServer().port}/static/color.less`,
                    loadingGifUrl: `//localhost:${rescriptsrc.devServer().port}/static/loading.gif`,
                    reactDom: `//localhost:${rescriptsrc.devServer().port}/static/react-dom.production.min.js`,
                    react: `//localhost:${rescriptsrc.devServer().port}/static/react.production.min.js`,
                },
                env: process.env.NODE_ENV,
                react_app_version: JSON.stringify(REACT_APP_VERSION)
            })
        })
    ],
};

module.exports = devConfig;
