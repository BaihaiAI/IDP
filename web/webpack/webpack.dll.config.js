const path = require('path')
const webpack = require('webpack')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')

module.exports = {
    mode: 'production',
    entry: {
        // 还有redux 之类的也可以放进来
        vendor: ['react', 'react-dom', 'react-router-dom', 'react-router', 'mobx', 'mobx-react']
    },

    devtool: false,

    output: {
        filename: '[name].dll.js',
        path: path.join(__dirname, '../dll'),
        // 链接库输出方式 默认'var'形式赋给变量
        libraryTarget: 'var',
        // 全局变量名称 导出库将被以var的形式赋给这个全局变量 通过这个变量获取到里面模块
        library: '_dll_[name]_[hash:8]'
    },

    plugins: [
        // 每次运行时清空之前的 dll 文件
        new CleanWebpackPlugin({
            cleanOnceBeforeBuildPatterns: [path.join(__dirname, '../dll/*')]
        }),
        new webpack.DllPlugin({
            // path 指定manifest文件的输出路径
            path: path.join(__dirname, '../dll/vendor.manifest.json'),
            // 和library 一致，输出的manifest.json中的name值
            name: '[name].dll.js'
        })
    ]
}