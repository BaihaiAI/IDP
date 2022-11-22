const webpack = require('webpack'); // 用于访问内置插件
const path = require('path');
const rescriptsrc = require('../config/rescriptsrc');
const { getThemeVariables } = require("antd/dist/theme");
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WebpackBar = require('webpackbar');
const AddAssetHtmlPlugin = require('add-asset-html-webpack-plugin');
const webpackConfig = require('../config/webpack-config');

console.log('@插件化代码提示:', process.env.NODE_PLUG === 'true' ? '正在打包extension文件夹中的插件化代码' : '插件化代码和闭源代码将应用到环境中');

// index: process.env.NODE_ENV !== 'dev' ? './src/index_dev.tsx' : './src/index_pro.tsx'

let REACT_APP_VERSION = '';

function loadEntry() {
    if (process.env.NODE_OPEN === 'true') {
        REACT_APP_VERSION = require('../config/global').REACT_APP_VERSION; // 设置成open
        console.log('@正在启动的环境是Idp:', process.env.NODE_ENV, '版本：', `,[${REACT_APP_VERSION}]`, `注意：当前启动的【${REACT_APP_VERSION}】是idp项目作用域的值`);
    } else {
        REACT_APP_VERSION = require('../../config/global').REACT_APP_VERSION; // 设置成open
        console.log('@正在启动的环境是Idp:', process.env.NODE_ENV, '版本：', `,[${REACT_APP_VERSION}]`, `注意：当前启动的【${REACT_APP_VERSION}】是全局作用域的值`);
    }
}

loadEntry();

let publicPathflg = false;
function getPublicPath() {
    let publicPath = '/';
    if (process.env.NODE_OPEN === 'true') {
        publicPath = '/';
    } else {
        publicPath = process.env.NODE_ENV === 'dev' ? `/` : '/child/idpStudio-idp/';
    }
    !publicPathflg && console.log('@当前启动的publicPath路径是:', publicPath);
    publicPathflg = true;
    return publicPath;
}

function loadRescriptsrc() {
    let _rescriptsrc = {};
    if (process.env.NODE_OPEN === 'true') {
        _rescriptsrc = {}
    } else {
        Object.assign(_rescriptsrc, {
            ...rescriptsrc.webpack()
        })
    }
    return _rescriptsrc
}

const baseConfig = {
    entry: {
        index: process.env.NODE_OPEN === 'true' ? './src/index_open.jsx' : ['./src/index.jsx']
    },
    output: {
        publicPath: getPublicPath(),
        path: path.join(__dirname, "../dist"),
        filename: `js/[name].[hash].js`,
        ...loadRescriptsrc()
    },
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
            webpackConfig.loadLessRules(),
            webpackConfig.loadModuleLessRules(),
            {
                test: /\.(png|jpe?g|gif|svg)(\?.*)?$/,
                use: [
                    {
                        loader: 'url-loader',
                        options: {
                            name: 'img/[name].[hash:7].[ext]',
                            limit: 1024,
                            publicPath: process.env.NODE_ENV === 'dev' ? `//localhost:${rescriptsrc.devServer().port}` : `/child/idpStudio-idp`,
                        }
                    }]
            }
        ]
    },
    plugins: [
        new WebpackBar(),
        new webpack.DefinePlugin({
            'process.env': {
                'NODE': JSON.stringify(process.env.NODE_ENV),
                'REACT_APP_VERSION': JSON.stringify(REACT_APP_VERSION),
                'NODE_OPEN': JSON.stringify(process.env.NODE_OPEN),
            }
        }),
        new AddAssetHtmlPlugin({
            filepath: require.resolve(`../static/less.min.js`),
            publicPath: getPublicPath()
        })
    ].concat(
        process.env.NODE_PLUG === 'true' ? [] : new CopyWebpackPlugin({
            patterns: [
                { from: path.resolve(__dirname, '../static'), to: path.resolve(__dirname, '../dist/static') }
            ],
        }),
    )
};

module.exports = baseConfig;
