const Webpack = require('webpack');
const WebpackDevServer = require('webpack-dev-server');
const webpackConfig = require('./webpack/webpack.dev.config');
const rescriptsrc = require('./config/rescriptsrc');

const compiler = Webpack(webpackConfig);
const devServerOptions = {
    ...webpackConfig.devServer,
    ...rescriptsrc.devServer()
};
const server = new WebpackDevServer(devServerOptions, compiler);

const runServer = async () => {
    console.log('正在启动Idp服务');
    await server.start();
};

const stopServer = async () => {
    console.log('Stopping server...');
    await server.stop();
};

runServer();