import { action, observable } from "mobx";
import AppComponentData from "./appCompeontData";
import RouterMenuControl, { routerMenu } from "./routerMenuControl";
import RightSideControl, { rightSideData } from "./rightSideControl";
import { RegisterApi } from "../register";
import FooterBarMenuControl, { footerBarMenu } from "./footerBarMenuControl";
import HeaderTool, { HeaderToolBean } from './headerTool';
import HeaderMeun, { HeaderMenuBean } from './headerMenu';

export type PluginConfig = {
    rightSide?: rightSideData,
    routeConfig?: routerMenu,
    footerBarMenu?: footerBarMenu,
    pluginsOptions?: HeaderToolBean | HeaderMenuBean,
    headerTool?: HeaderToolBean,
    headerMenu?: HeaderMenuBean,
    id: string,
    autoStart?: Boolean,
}
class GlobalData {

    @observable appComponentData: AppComponentData;
    @observable routerMenuControl: RouterMenuControl;
    @observable pluginIdArr: string[];
    @observable rightSideControl: RightSideControl;
    @observable footerBarMenuControl: FooterBarMenuControl;
    @observable headerMenu: HeaderMeun;
    @observable headerTool: HeaderTool;

    constructor() {
        this.appComponentData = new AppComponentData();
        this.routerMenuControl = new RouterMenuControl();
        this.rightSideControl = new RightSideControl();
        this.footerBarMenuControl = new FooterBarMenuControl();
        this.headerMenu = new HeaderMeun();
        this.headerTool = new HeaderTool();
        this.pluginIdArr = [];
    }

    @action register(type: RegisterApi, pluginConfig: PluginConfig) {
        if (!pluginConfig.autoStart) {
            if (type === RegisterApi.menu_api) {
                pluginConfig.routeConfig.type = RegisterApi.menu_api;
                this.routerMenuControl.addRoute(pluginConfig.routeConfig)
            } else if (type === RegisterApi.right_side_api) {
                pluginConfig.rightSide.type = RegisterApi.right_side_api;
                this.rightSideControl.addRightSide(pluginConfig.rightSide);
            } else if (type === RegisterApi.footer_bar_api) {
                pluginConfig.footerBarMenu.type = RegisterApi.footer_bar_api;
                this.footerBarMenuControl.addFooterBarMenu(pluginConfig.footerBarMenu);
            } else if (type === RegisterApi.header_meun_api) {
                pluginConfig.headerMenu.type = RegisterApi.header_meun_api;
                this.headerMenu.addHeaderMenu(pluginConfig.headerMenu);
            } else if (type === RegisterApi.header_tool_api) {
                pluginConfig.headerTool.type = RegisterApi.header_tool_api;
                this.headerTool.addHeaderTool(pluginConfig.headerTool);
            }
        }
    }

    @action unRegister(type: RegisterApi, key: string) {
        if (type === RegisterApi.menu_api) {
            this.routerMenuControl.removeRoute(key)
        } else if (type === RegisterApi.right_side_api) {
            this.rightSideControl.removeRightSide(key)
        } else if (type === RegisterApi.footer_bar_api) {
            this.footerBarMenuControl.removeFooterBarMenu(key)
        } else if (type === RegisterApi.header_meun_api) {
            this.headerMenu.removeHeaderMenu(key)
        } else if (type === RegisterApi.header_tool_api) {
            this.headerTool.removeHeaderTool(key)
        }
    }
}

const globalData = new GlobalData()

export default globalData
