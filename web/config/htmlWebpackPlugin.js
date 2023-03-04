const path = require('path');
/**
 * HtmlWebpackPlugin 配置
 * @param {*} param0 
 * @returns 
 */
function createHtmlWebpackPlugin({ options = {}, minifyOptions = {}, env = 'dev', react_app_version}) {

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
        reactDom: '/child/idpStudio-idp/static/react-dom.production.min.js',
        react: '/child/idpStudio-idp/static/react.production.min.js',
        lessmin: '/child/idpStudio-idp/static/less.min.js',
        react_app_version,
        loadPlugins: true,
        minify
    }, { ...options });

    return htmlOptions
}

module.exports = {
    createHtmlWebpackPlugin
}