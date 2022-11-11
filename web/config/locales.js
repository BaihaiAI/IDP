import idpStudio from 'idp-studio';
require('intl/locale-data/jsonp/en.js');
require('intl/locale-data/jsonp/zh.js');

(function () {

    const locales = {
        "en-US": require('../../src/locales/en-US.json'),
        "zh-CN": require('../../src/locales/zh-CN.json')
    }

    function loadLocales() {
        idpStudio.StudioIntl.init({ currentLocale: 'zh-CN', locales }).then(() => { });
    }

    loadLocales();

})();