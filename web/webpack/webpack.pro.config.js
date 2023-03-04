const webpack = require('webpack');
const path = require('path');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const UglifyJsPlugin = require('uglifyjs-webpack-plugin');
const ScriptExtHtmlWebpackPlugin = require("script-ext-html-webpack-plugin");
const AddAssetHtmlWebpackPlugin = require('add-asset-html-webpack-plugin');
const AntdDayjsWebpackPlugin = require('antd-dayjs-webpack-plugin');
const { createHtmlWebpackPlugin } = require('../config/htmlWebpackPlugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WebpackBar = require('webpackbar');
const CopyWebpackPlugin = require('copy-webpack-plugin');
// const BundleAnalyzerPlugin = require("webpack-bundle-analyzer").BundleAnalyzerPlugin;
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CompressionPlugin = require('compression-webpack-plugin');
const zlib = require("zlib");

const rescriptsrc = require('../config/rescriptsrc');
const webpackConfig = require('../config/webpack-config');

const publicPath = Boolean(process.env.NODE_OPEN) ? '/' : '/child/idpStudio-idp/';
const entry = {
    index: Boolean(process.env.NODE_OPEN) ? `./src/open.jsx` : `./src/plugin.jsx`
};
const output = {
    path: path.join(__dirname, "../dist"),
    filename: `js/[name].[hash].js`,
    publicPath: publicPath,
    chunkFilename: 'js/[name].[id].js',
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

loadEntry();

console.log('--------webpack pro【studio】启动信息--------');
console.log('@当前启动的版本是:', Boolean(process.env.NODE_OPEN) ? '单机studio版' : '微应用studio版');
console.log('@当前打包插件环境:', Boolean(process.env.NODE_PLUGIN) ? '应用本地插件环境' : '应用s3库插件环境');;
console.log('@当前启动的js静态引入的path是:', publicPath);
console.log('@当前启动webpack插件列表:', entry);
console.log('@REACT_APP_VERSION:', REACT_APP_VERSION);
console.log('@当前启动webpack打包输出模式:', output);
console.log('@当前环境：', REACT_APP_VERSION);
console.log('@当前启动的接口代理信息:', rescriptsrc.devServer().proxy);

const proConfig = {
    mode: 'production',
    devtool: false, // source-map
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
            webpackConfig.loadUrlRules(),
            webpackConfig.loadModuleLessRules()
        ]
    },
    plugins: [
        new WebpackBar(),
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
        new webpack.DefinePlugin({
            'process.env': {
                'NODE': JSON.stringify(process.env.NODE_ENV),
                'REACT_APP_VERSION': JSON.stringify(REACT_APP_VERSION),
                'NODE_OPEN': JSON.stringify(process.env.NODE_OPEN),
                'NODE_TARGET': JSON.stringify(rescriptsrc.devServer().proxy.target),
                'NODE_PLUGIN': JSON.stringify(process.env.NODE_PLUGIN)
            }
        }),
        new webpack.DllReferencePlugin({
            context: path.join(__dirname),
            manifest: path.join(__dirname, `../dll/vendor.manifest.json`)
        }),
        new AddAssetHtmlWebpackPlugin({
            filepath: path.resolve(__dirname, '../dll/vendor.dll.js'),
            outputPath: '../dist/js',
            publicPath: `${Boolean(process.env.NODE_OPEN) ? './js' : '/child/idpStudio-idp/js'}`
        }),
        new CopyWebpackPlugin({
            patterns: [
                { from: path.resolve(__dirname, '../static'), to: path.resolve(__dirname, '../dist/static') },
                { from: path.resolve(__dirname, '../dist_extension'), to: path.resolve(__dirname, '../dist/extension') }
            ]
        }),
        // 打包体积分析
        // new BundleAnalyzerPlugin(),
        new HtmlWebpackPlugin({
            template: 'index.ejs',
            filename: 'index.html',
            ...createHtmlWebpackPlugin({
                options: {
                    colorLinkUrl: Boolean(process.env.NODE_OPEN) ? `/static/color.less` : '/child/idpStudio-idp/static/color.less',
                    reactDom: Boolean(process.env.NODE_OPEN) ? `/static/react-dom.production.min.js` : `/child/idpStudio-idp/static/react-dom.production.min.js`,
                    react: Boolean(process.env.NODE_OPEN) ? `/static/react.production.min.js` : `/child/idpStudio-idp/static/react.production.min.js`,
                    loadingGifUrl: Boolean(process.env.NODE_OPEN) ? `/static/loading.gif` : `/child/idpStudio-idp/static/loading.gif`,
                    lessmin: Boolean(process.env.NODE_OPEN) ? `/static/less.min.js` : `/child/idpStudio-idp/static/less.min.js`
                },
                env: process.env.NODE_ENV
            })
        })
    ]
};

module.exports = proConfig;