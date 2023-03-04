import { noteApiPath2 } from './httpClient';
import { userId, teamId } from '../store/cookie';
const rescriptsrc = require('../../config/rescriptsrc');
import request from "./request";
import axios from 'axios';

function installedList() {
    const url = `${noteApiPath2}/extensions/installedList`;
    return request.get(url);
}

function unInstalledList(params) {
    const url = `${noteApiPath2}/extensions/uninstall?name=${params.name}&version=${params.version}`;
    return request.get(url);
}

function recommendedList() {
    const url = `${noteApiPath2}/extensions/recommendedList`;
    return request.get(url);
}

function loadPath(path) {
    const url = `${noteApiPath2}/extensions/load${path}`;
    return request.get(url);
}

function install(params) {
    let url = `${noteApiPath2}/extensions/install?name=${params.name}&entry=${params.entry}`;
    if (params.version) {
        url += `&version=${params.version}`;
    }
    return request.get(url);
}

function laodImgs(params) {
    return `${Boolean(process.env.NODE_OPEN) ? process.env.proxy_target : ''}${noteApiPath2}/extensions/load${params.url}${params.icon}`;
}

function loadJavaScriptPlugins(params) {
    const url = `${Boolean(process.env.NODE_OPEN) ? process.env.proxy_target : ''}${noteApiPath2}/extensions/load${params.url}${params.entry}`;
    return request.get(url);
}

function loadScript(params) {
    return `${Boolean(process.env.NODE_OPEN) ? process.env.proxy_target : ''}${noteApiPath2}/extensions/load${params.url}${params.entry}`;
}

function loadLocalSystemScript(params) {
    return process.env.NODE === 'pro' ? `/child/idpStudio-idp/extension/${params.url}/${params.entry}` : `//localhost:${rescriptsrc.devServer().port}/extension/${params.url}/${params.entry}`;
}

async function updatePluginVersion(version, name) {
    axios.defaults.headers['Content-Type'] = 'application/json; charset=utf-8';
    const url = `${noteApiPath2}/extensions/update`;
    const data = { userId, teamId, version, name }
    return await axios.post(url, data);
}

const developApi = {
    installedList,
    unInstalledList,
    recommendedList,
    loadPath,
    install,
    laodImgs,
    loadScript,
    loadJavaScriptPlugins,
    updatePluginVersion,
    loadLocalSystemScript
};

export default developApi;
