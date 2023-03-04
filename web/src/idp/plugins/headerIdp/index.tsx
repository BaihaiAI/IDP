import IdpMenu from './pages';
import globalData from "idpStudio/idp/global";
import { RegisterApi } from "idpStudio/idp/register";

const configJson = require("./config.json")

const config = {
    key: 'idp',
    component: (<IdpMenu />),
    configJson,
    weight: 1
}

globalData.register(RegisterApi.header_meun_api, {
    headerMenu: config,
    autoStart: false,
    id: `${configJson.fileName}/${configJson.entry}`,
})
