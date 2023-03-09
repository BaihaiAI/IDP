let path = require('path');
let { getThemeVariables } = require("antd/dist/theme");
const { version, name } = require('../package.json');

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
            "@components": path.resolve(__dirname, '../src/components')
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
    loadJsxOrTsxRules: function (options = {}, reset = false, useOpt = false) {
        let optRules = {};
        if (reset) {
            optRules = options;
        } else {
            optRules = { test: /\.(j|t)sx?$/, use: useOpt ? useOpt : ['cache-loader', 'thread-loader', 'babel-loader'], exclude: /node_modules/ }
        };
        return optRules;
    },
    loadJsRules: function (options = {}, reset = false, useOpt = false) {
        let optRules = {};
        if (reset) {
            optRules = options;
        } else {
            optRules = { test: /\.js$/, use: useOpt ? useOpt : ['babel-loader'], exclude: /node_modules/ }
        };
        return optRules;
    },
    loadUrlRules: function (options = {}, reset = false, useOpt = false) {
        let optRules = {};
        if (reset) {
            optRules = options;
        } else {
            optRules = { 
                test: /\.(png|jpe?g|gif|svg)(\?.*)?$/, 
                use: useOpt ? useOpt : {
                    loader: 'url-loader',
                }
            }
        };
        return optRules;
    },
    loadCssRules: function (options = {}, reset = false, useOpt = false) {
        let optRules = {};
        if (reset) {
            optRules = options;
        } else {
            optRules = { test: /\.css$/, use: useOpt ? useOpt : ['style-loader', 'css-loader'] }
        };
        return optRules;
    },
    loadLessRules: function (options = {}, reset = false, useOpt = false) {
        let optRules = {};
        if (reset) {
            optRules = options;
        } else {
            optRules = {
                test: /\.less$/,
                exclude: /\.module\.less$/,
                use: [
                    'style-loader',
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
    loadModuleLessRules: function (options = {}, reset = false, useOpt = false) {
        let optRules = {};
        if (reset) {
            optRules = options;
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