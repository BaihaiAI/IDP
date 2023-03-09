const { name } = require('../package.json');

module.exports = {
    webpack: (config = {}) => {
        config.library = `${name}-[name]`;
        config.libraryTarget = 'umd';
        config.globalObject = 'window';
        config.chunkLoadingGlobal = `webpackJsonp_${name}`;
        return config;
    }
};
