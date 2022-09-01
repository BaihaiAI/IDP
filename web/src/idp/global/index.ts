import { action, observable } from "mobx"
import AppComponentData from "idp/global/appCompeontData"
import RouterMenuControl, { routerMenu } from "idp/global/routerMenuControl"
import RightSideControl, { rightSideData } from "idp/global/rightSideControl"
import { RegisterApi } from "idp/register"
import FooterBarMenuControl, { footerBarMenu } from "idp/global/footerBarMenuControl"

type PluginConfig = {
  rightSide?: rightSideData,
  routeConfig?: routerMenu,
  footerBarMenu?: footerBarMenu
  id: string,
  autoStart?: Boolean
}


class GlobalData {
  @observable appComponentData: AppComponentData
  @observable routerMenuControl: RouterMenuControl
  @observable pluginIdArr: string[]
  @observable rightSideControl: RightSideControl
  @observable footerBarMenuControl: FooterBarMenuControl

  constructor() {
    this.appComponentData = new AppComponentData()
    this.routerMenuControl = new RouterMenuControl()
    this.rightSideControl = new RightSideControl()
    this.footerBarMenuControl = new FooterBarMenuControl()
    this.pluginIdArr = []
  }

  @action register(type: RegisterApi, pluginConfig: PluginConfig) {
    if (!pluginConfig.autoStart) {
      if (type === RegisterApi.menu_api) {
        this.routerMenuControl.addRoute(pluginConfig.routeConfig)
      } else if (type === RegisterApi.right_side_api) {
        this.rightSideControl.addRightSide(pluginConfig.rightSide)
      } else if (type === RegisterApi.footer_bar_api) {
        this.footerBarMenuControl.addFooterBarMenu(pluginConfig.footerBarMenu)
      }
    }
  }
}

const globalData = new GlobalData()



export default globalData
