// dev开发环境，屏蔽请求加载的index.js插件
export const unEnabledLoadDevPlugins = [
    // 'centre',
    'contact',
    'dataSet',
    'environment',
    'feedback',
    'monitor',
    'networkstatus',
    'notification',
    'teams',
    'tensorboard',
    'usage',
    'workflow',
    'modelwarenhouse',
    'optuna',
    'colony',
    'datalabel'
];

export function devLoadPlugins(res) {
    let newRes = res;
    if (process.env.NODE == 'dev' && !Boolean(process.env.NODE_PLUGIN) && !Boolean(process.env.NODE_OPEN)) {
        newRes = [];
        Object.keys(res).forEach(index => {
            if ( res[index]?.name ) {
                if (!unEnabledLoadDevPlugins.includes(res[index].name)) {
                    newRes.push(res[index]);
                }
            } else {
                newRes.push(res[index]);
            }
        })
    }
    return newRes;
}