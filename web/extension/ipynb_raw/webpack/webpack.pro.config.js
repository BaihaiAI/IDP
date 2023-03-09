const webpack = require('webpack'); // 用于访问内置插件
const path = require('path');
const { version, name } = require('../package.json');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const UglifyJsPlugin = require('uglifyjs-webpack-plugin');
const WebpackBar = require('webpackbar');
const ParallelUglifyPlugin = require('webpack-parallel-uglify-plugin');
const CompressionPlugin = require('compression-webpack-plugin');
const OptimizeCssAssetsPlugin = require('optimize-css-assets-webpack-plugin');
const zlib = require("zlib");

const idpPluginsConfig = require('../plugins-config');
const webpackConfig = require('../config/webpack-config');
const CssNano = require('cssnano');

let pluginVersion = version;

if (Boolean(process.env.LOCAL)) {
  pluginVersion = '';
}

const output = {
    publicPath: '/',
    path: path.join(__dirname, `../dist`),
    filename: `[name]/${pluginVersion}/index.js`,
    library: "[name]",
    libraryTarget: "umd",
    libraryExport: 'default'
}

const entry = idpPluginsConfig.webpackEntry();

const proConfig = {
    mode: 'production',
    entry,
    output,
    devtool: false,
    resolve: {
        extensions: webpackConfig.getExtensions()
    },
    performance: webpackConfig.getPerformance(),
    optimization: {
        minimizer: [
            new UglifyJsPlugin({
                test: /\.js(\?.*)?$/i,  //测试匹配文件
                cache: true,
                parallel: true,
                uglifyOptions: {
                    warnings: false,
                    parse: {},
                    compress: {},
                    mangle: true, // Note `mangle.properties` is `false` by default.
                    output: null,
                    toplevel: false,
                    nameCache: null,
                    ie8: false,
                    keep_fnames: false
                }
            }),
            new OptimizeCssAssetsPlugin({
                assetNameRegExp: /\.(sa|sc|c)ss$/g,
                cssProcessor: CssNano,
                cssProcessorOptions: {
                    safe: true,
                    discardComments: { removeAll: true }, //对CSS文件中注释的处理：移除注释
                    normalizeUnicode: false // 建议false,否则在使用unicode-range的时候会产生乱码
                },
                canPrint: true
            }),
        ],
    },
    module: {
        rules: [
            webpackConfig.loadJsxOrTsxRules(),
            webpackConfig.loadJsRules(),
            webpackConfig.loadCssRules(),
            webpackConfig.loadLessRules(),
            webpackConfig.loadUrlRules(),
            webpackConfig.loadModuleLessRules(),
        ]
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
        }
    },
    plugins: [
        new WebpackBar(),
        new CleanWebpackPlugin(),
        new webpack.DefinePlugin({
            'process.env': {
                'NODE': JSON.stringify(process.env.NODE_ENV)
            }
        }),
        // new AntdDayjsWebpackPlugin(),
        new ParallelUglifyPlugin({
            uglifyJS: {
                output: {
                    beautify: false, // 最紧凑的输出
                    comments: false, // 删除所有的注释
                },
                compress: {
                    drop_console: true,
                    collapse_vars: true,
                    reduce_vars: true,
                }
            }
        }),
    ].concat(Boolean(process.env.LOCAL) || Boolean(process.env.NODE_JS) || Boolean(process.env.NODE_JS) ? [] : new CompressionPlugin({
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
    }))
};

module.exports = proConfig;