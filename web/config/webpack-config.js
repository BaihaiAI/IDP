let path = require('path');
let { getThemeVariables } = require("antd/dist/theme");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const rescriptsrc = require('../config/rescriptsrc');

module.exports = {
    /**
     * 设置webpack alias参数
     * @param {*} options  "@": path.resolve(__dirname, xxx),
     * @param {*} reset 是否重置alias
     * @returns 
     */
    getAlias: function (options, reset = false) {
        let opt = {
            "@": path.resolve(__dirname, '../src'),
            "@assets": path.resolve(__dirname, '../src/assets'),
            "@components": path.resolve(__dirname, '../src/components'),
            '@idp': path.resolve(__dirname, '../src/idp'),
            "idpStudio": path.resolve(__dirname, '../src'),
            "idp": path.resolve(__dirname, '../src/idp'),
            'idpServices': path.resolve(__dirname, '../src/services'),
            'idpUtils': path.resolve(__dirname, '../src/utils'),
            'idpStore': path.resolve(__dirname, '../src/store')
        };
        return Object.assign(reset ? {} : { ...opt }, { ...options })
    },
    /**
     * 设置webpack Extensions参数
     * @param {*} options [".ts", ".js", ".tsx", ".jsx", ".json", '.css', '.less']
     * @returns 
     */
    getExtensions: function (options = []) {
        if (options.length === 0) {
            return [".ts", ".js", ".tsx", ".jsx", ".json", '.css', '.less']
        } else {
            return options
        }
    },
    /**
     * 设置webpack Performance参数
     * @param {*} hints 
     * @param {*} options 
     * @returns 
     */
    getPerformance: function (hints = false, options = {}) {
        return { hints }
    },
    /**
     * webpage加载 module-rules
     * @param {*} options
     * @param {*} reset 是否重置 /\.(j|t)sx?$/
     * @param {*} useOpt
     * @returns 
     */
    loadJsxOrTsxRules: function (option = {}, reset = false, useOpt = false, exclude = [/node_modules/, /extension/]) {
        let optRules = {};
        if (reset) {
            optRules = option;
        } else {
            optRules = {
                test: /\.(j|t)sx?$/, use: useOpt ? useOpt : ['cache-loader', 'thread-loader', {
                    loader: 'babel-loader',
                    options: {
                        plugins: [
                            '@babel/plugin-syntax-dynamic-import'
                        ]
                    }
                }], exclude: exclude
            }
        };
        return optRules;
    },
    loadJsRules: function (option = {}, reset = false, useOpt = false, exclude = [/node_modules/, /extension/]) {
        let optRules = {};
        if (reset) {
            optRules = option;
        } else {
            optRules = {
                test: /\.js$/, use: useOpt ? useOpt : [{
                    loader: 'babel-loader',
                    options: {
                        plugins: [
                            '@babel/plugin-syntax-dynamic-import'
                        ]
                    }
                }], exclude: exclude
            }
        };
        return optRules;
    },
    loadUrlRules: function (option = {}, reset = false, useOpt = false) {
        let optRules = {};
        if (reset) {
            optRules = option;
        } else {
            optRules = { test: /\.(png|jpe?g|gif|svg)(\?.*)?$/, use: useOpt ? useOpt : ['url-loader'] }
        };
        return optRules;
    },
    loadCssRules: function (option = {}, reset = false, useOpt = false) {
        let optRules = {};
        if (reset) {
            optRules = option;
        } else {
            optRules = {
                test: /\.css$/, use: useOpt ? useOpt : [{
                    loader: MiniCssExtractPlugin.loader,
                    options: {
                        publicPath: process.env.NODE_ENV === 'dev' ? `//localhost:${rescriptsrc.devServer().port}` : `/child/idpStudio-idp`,
                    }
                }, 'css-loader']
            }
        };
        return optRules;
    },
    loadLessRules: function (option = {}, reset = false, useOpt = false) {
        let optRules = {};
        if (reset) {
            optRules = option;
        } else {
            optRules = {
                test: /\.less$/,
                exclude: /\.module\.less$/,
                use: useOpt ? useOpt : [
                    MiniCssExtractPlugin.loader,
                    'css-loader',
                    {
                        loader: "less-loader",
                        options: {
                            lessOptions: {
                                modifyVars: {
                                    ...getThemeVariables({
                                        compact: false,
                                    }),
                                },
                                javascriptEnabled: true,
                            },
                        },
                    }
                ]
            }
        };
        return optRules;
    },
    loadModuleLessRules: function (option = {}, reset = false, useOpt = false) {
        let optRules = {};
        if (reset) {
            optRules = option;
        } else {
            optRules = {
                test: /\.module\.(less)$/,
                use: [
                    'style-loader',
                    {
                        loader: 'css-loader',
                        options: {
                            modules: {
                                localIdentName: '[name]_[local]-[hash:6]'
                            }
                        }
                    },
                    {
                        loader: "less-loader",
                        options: {
                            lessOptions: {
                                modifyVars: {
                                    ...getThemeVariables({
                                        compact: false,
                                    }),
                                },
                                javascriptEnabled: true,
                            },
                        },
                    }
                ]
            }
        };
        return optRules;
    }
}