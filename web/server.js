const Webpack = require('webpack');
const WebpackDevServer = require('webpack-dev-server');

console.log('关键环境变量：', Boolean(process.env.NODE_PLUGIN));
let webpackConfig = '';
if (Boolean(process.env.NODE_PLUGIN)) {
    webpackConfig = require('./webpack/webpack.plugin.config');
} else {
    webpackConfig = require('./webpack/webpack.dev.config');
}

const rescriptsrc = require('./config/rescriptsrc');

const compiler = Webpack(webpackConfig);
const devServerOptions = {
    ...webpackConfig.devServer,
    ...rescriptsrc.devServer()
};
const server = new WebpackDevServer(devServerOptions, compiler);

const runServer = async () => {
    await server.start();
};

const stopServer = async () => {
    await server.stop();
};

runServer();