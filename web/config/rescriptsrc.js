const { name } = require('../package.json');

let target = "";

if (process.env.NODE_OPEN === 'true') {
    target = 'http://127.0.0.1:3000'; // http://192.168.12.12:3033
} else {
    target = require('../../config/global.js').originTargetUrl;
}

console.log('@当前加载的proxy代理地址是:', target);

module.exports = {
    webpack: (config = {}) => {
        config.library = `${name}-[name]`;
        config.libraryTarget = 'umd';
        config.globalObject = 'window';
        config.chunkLoadingGlobal = `webpackJsonp_${name}`;
        return config;
    },
    devServer: (_ = {}) => {
        const config = Object.assign({}, _);
        config.headers = {
            'Access-Control-Allow-Origin': '*',
        };
        config.historyApiFallback = true;
        config.hot = true;
        config.liveReload = true;
        config.port = 8090;
        config.compress = true;
        config.proxy = {  //进行代理转发
            '/**/api/**': {
                target: target,
                changeOrigin: true,
                ws: true,
            }
        }
        return config;
    },
};
