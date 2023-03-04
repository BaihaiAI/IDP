const path = require('path');
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const WebpackBar = require('webpackbar');
const webpackConfig = require('./config/webpack-config');
const ParallelUglifyPlugin = require('webpack-parallel-uglify-plugin');
const TerserPlugin = require('terser-webpack-plugin') // 引入压缩插件

const entry = {
    idpStudio: './src/index.js'
};
const output = {
    filename: 'index.js',
    path: path.join(__dirname, './dist'),
    library: 'idpStudio',
    libraryTarget: 'umd',
    libraryExport: "default"
}

module.exports = {
    entry,
    output,
    devtool: false,
    resolve: {
        alias: webpackConfig.getAlias(),
        extensions: webpackConfig.getExtensions()
    },
    mode: 'none',
    optimization: {
        minimize: true,
        minimizer: [
            new TerserPlugin({ // 使用压缩插件
                include: /\.index\.js$/
            })
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
                },
                toplevel: false,
                warnings: false
            }
        }),
        new CleanWebpackPlugin()
    ]
}