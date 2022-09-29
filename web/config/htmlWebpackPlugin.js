const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');
/**
 * HtmlWebpackPlugin 配置
 * @param {*} param0 
 * @returns 
 */
function createHtmlWebpackPlugin({ template, filename, options = {}, minifyOptions = {}, env = 'dev' }) {

    const minify = env === 'dev' ? {} : Object.assign({
        removeComments: true, // 移除HTML中的注释
        collapseWhitespace: true, // 删除空白符与换行符
        minifyCSS: true,// 压缩内联css
        removeAttributeQuotes: true, // 去掉一些属性的引号，例如id="moo" => id=moo
        removeRedundantAttributes: true,
        useShortDoctype: true,
        removeEmptyAttributes: true,
        removeStyleLinkTypeAttributes: true,
        keepClosingSlash: true,
        minifyJS: true,
        minifyCSS: true,
        minifyURLs: true
    }, { ...minifyOptions });

    const htmlOptions = Object.assign({
        inject: true,
        colorLinkUrl: '/child/idpStudio-idp/static/color.less',
        loadingGifUrl: '/child/idpStudio-idp/static/loading.gif',
        loadPlugins: true,
        minify,
        chunks: ['index']
    }, { ...options });

    return new HtmlWebpackPlugin({
        template: template || 'index.ejs',
        filename: filename || 'index.html',
        ...htmlOptions,
        faviconUrl: (process.env.NODE === 'dev' || process.env.NODE_OPEN === 'true') ? './static/favicon.ico' : '/child/idpStudio-idp/static/favicon.ico',
    })
}

module.exports = {
    createHtmlWebpackPlugin
}