import globalData from '@/idp/global';
import LoadPlugins from '@/idp/global/plugins/load';
import { developApi } from '@/services';
import { noteApiPath2 } from '../src/services/httpClient';
import { devLoadPlugins } from './unEnabledLoadDevPlugins';
import { userId, teamId } from '../src/store/cookie';
import axios from 'axios';

const localPlugins = require('./local_plugins')
let pluginsSize = 0;
let updateSize = 0;

async function updatePluginVersionApi(res, callback) {
    // 系统默认的插件若是存在最新的版本，则系统默认更新到最新, 更新规则,如下：
    // 1、[v].[n].[m]，v版本不自动更新，n以及m需要自动更新
    // 2、系统默认安装的全部更新
    res = devLoadPlugins(res);
    const isp = res.filter(it => it.optionalVersion.length > 0);
    if (isp.length > 0) {
        Object.values(res).forEach(async (it) => {
            if (it.optionalVersion.length > 0) {
                if (!it.visible) {
                    // 判断visible
                    await updateSizeAction(it);
                } else {
                    // 判断version
                    const oldVersion = it.version.split('.'); // 获取旧版本
                    const newVersion = it.optionalVersion[0].split('.'); // 获取新版本
                    if (oldVersion.length > 2 && newVersion.length > 2) { // 必须版本正常
                        if (oldVersion[0] == newVersion[0]) { // 必须保证v版本一致，不一致不触发自动更新, 
                            if (Number(newVersion[1]) > Number(oldVersion[1]) || Number(newVersion[2]) > Number(oldVersion[2])) {
                                await updateSizeAction(it);
                            }
                        } else {
                            updateSize = updateSize + 1;
                        }
                    }
                }
                if (updateSize === isp.length) {
                    callback(res);
                }
            }
        })
    } else {
        callback(res);
    }
}

async function loadLocalSystemExtensions(callback) {
    localSystemPlugin(callback);
}

async function localSystemPlugin(callback) {
    const pluginsLocalPaths = localPlugins.map(it => developApi.loadLocalSystemScript(it));
    pluginsAction(pluginsLocalPaths, localPlugins, callback);
}

async function updateSizeAction(it) {
    updateSize = updateSize + 1;
    try {
        const updateRes = await developApi.updatePluginVersion(it.optionalVersion[0], it.name);
        if (updateRes.data.code >= 20000000 && updateRes.data.code <= 30000000) {
            it.url = updateRes.data?.data || '';
            it.version = it.optionalVersion[0];
        }
    } catch (error) { }
}

async function loadExtensions(callback) {
    try {
        // let data = await developApi.installedList();
        const { data } = await axios.get(`${noteApiPath2}/extensions/installedList`, { params: { teamId, userId } });
        data.data = devLoadPlugins(data.data);
        if (data.code === 21000000) {
            // 判断逻辑：以localPlugins为主，为辅install接口为辅
            Object.values(localPlugins).forEach(it => {
                const flgObjArray = data.data.filter(its => it.name == its.name);
                if (flgObjArray.length > 0) {
                    flgObjArray[0].local = true; // 置为空，则过滤不加载
                    flgObjArray[0].url = it.name;
                    flgObjArray[0].optionalVersion = [];
                    flgObjArray[0].visible = false;
                } else {
                    data.data.push({
                        name: it.name,
                        optionalVersion: [],
                        url: it.name,
                        title: it.name,
                        visible: false,
                        local: true
                    });
                }
            });
            if (data.data.length > 0) {
                const res = data.data.filter(it => it.url);
                updatePluginVersionApi(res, (result) => {
                    loadScriptApi(result, callback);
                    LoadPlugins.updatePluginSize(result.length);
                })
            } else {
                callback()
            }
        } else {
            callback();
        }
    } catch (error) {
        callback();
    }
}

export function loadScriptApi(data, callback) {
    const pluginsPaths = data.map(it => {
        if (it.local) {
            return developApi.loadLocalSystemScript(it)
        } else {
            return developApi.loadScript(it)
        }
    });
    pluginsAction(pluginsPaths, data, callback);
}

export function pluginsAction(pluginsPaths, data, callback) {
    loadScript(pluginsPaths, data, function (status) {
        if (status === 'rejected') {
            callback(status);
        };
        for (let i = 0; i < data.length; i++) {
            if (window.hasOwnProperty(data[i]?.name)) {
                window[data[i]?.name].activate(globalData);
                callback('pending', data[i]);
            }
            if (data.length === (i + 1)) {
                callback('resolved')
            }
        }
    })
}

function loadScript(src, data, callback) {
    arraySync(function (one, i, c) {
        var cur_script = document.createElement("script");
        cur_script.type = 'text/javascript';
        cur_script.src = one;
        cur_script.async = true;
        cur_script.addEventListener('load', function () {
            c(0, {
                i: i,
                v: data[i]
            });
            pluginsSize = pluginsSize + 1;
            LoadPlugins.updateCurrentLoadPluginSize(pluginsSize);
            LoadPlugins.updateCurrentLoadPluginRecord(data[i]);
        }, false);
        document.head.appendChild(cur_script);
    }, src, data, function (err, r) {
        //全部加载完成后执行的回调函数
        if (err) {
            callback('rejected');
        } else {
            callback()
        }
    });
}

//处理异步，不用promise的方案
function arraySync(bsFunc, ar, data) {
    var callback = arguments[arguments.length - 1];
    if (ar.length == 0) {
        callback(0, []);
        return;
    }
    var sendErr = false;
    var finishNum = ar.length;
    var result = [];
    var args = [0, 0];
    for (var index = 3; index < arguments.length - 2; ++index) {
        args.push(arguments[index]);
    }
    args.push(function (err, r) {
        if (err) {
            if (!sendErr) {
                sendErr = true;
                callback(err);
            }
            return;
        }
        --finishNum;
        result[r.i] = r.v;
        if (result.length == data.length) {
            let unLoadPlugins = [];
            for (let i = 0; i < result.length; i++) {
                if (!result[i]) {
                    unLoadPlugins.push(data[i]);
                }
            };
            if (unLoadPlugins.length > 0) {
                console.log('@未执行的插件:', unLoadPlugins);
            }
        }
        callback(0, result); // 不必等待插件全部加载完成
    });

    for (var i = 0; i < ar.length; ++i) {
        args[0] = ar[i];
        args[1] = i;
        bsFunc.apply(null, args);
    }
};

export {
    loadLocalSystemExtensions,
    loadExtensions
}
