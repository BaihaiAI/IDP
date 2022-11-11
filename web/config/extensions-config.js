const rescriptsrc = require('../config/rescriptsrc');

function loadExtensions() {
    var extensionsURl = process.env.NODE === 'dev' ? `//localhost:${rescriptsrc.devServer().port}` : '/child/idpStudio-idp';
    const path = `${extensionsURl}/extension`;
    var plugins = [
        `${path}/dataRoute/index.js`,
        `${path}/footerBarNetWorkStatus/index.js`,
        `${path}/footerBarSwitchEnvironment/index.js`,
        `${path}/footerBarUsage/index.js`,
        `${path}/headerTeam/index.js`,
        `${path}/modelwarenhouseRoute/index.js`,
        `${path}/monitor/index.js`,
        `${path}/tensorboardRoute/index.js`,
        `${path}/wandbRoute/index.js`,
        `${path}/workflowRoute/index.js`
    ];
    for (var pl = 0; pl < plugins.length; pl++) {
        var hm_p = document.createElement("script");
        hm_p.src = plugins[pl];
        var s_p = document.getElementsByTagName("script")[0];
        s_p.parentNode.insertBefore(hm_p, s_p);
    }
}

export default loadExtensions