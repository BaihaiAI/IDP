import App from './pages';
import { RegisterApi } from "idpStudio/idp/register";
import globalData from '@/idp/global';
import userinfo from '@/idp/global/userinfo';
const configJson = require("./config.json");

let flg = true;
if (process.env.NODE == 'pro') {
    if (process.env.REACT_APP_VERSION === 'SAAS') {
        if ( userinfo?.navType === 'AIGC') {
            flg = false;
        } else {
            flg = true;
        }
    } else {
        flg = false;
    }
} else {
    if (!Boolean(process.env.NODE_PLUGIN)) {
        flg = false;
    }
    if ( userinfo?.navType === 'AIGC') {
        flg = false;
    }
}

const routeConfig = {
    key: 'develop', // key值，和路由保持一致，必填
    name: '扩展包管理',
    iconUnChecked: <img src={require('./assets/develop.svg').default} style={{width: '36px'}}></img>, // 未选中的icon， 默认false
    iconChecked: false, // 选中时的icon, 默认false
    menuClassName: {
        paddingLeft: "1px",
        paddingTop: "0px",
        paddingRight: '0px'
    }, // 默认{}
    flg,
    component: flg ? App : null
}

globalData.register(RegisterApi.menu_api, {
    routeConfig,
    id: `${configJson.fileName}/${configJson.entry}`,
    title: '扩展包管理'
})