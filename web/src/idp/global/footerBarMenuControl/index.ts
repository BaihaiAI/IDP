import {ReactElement} from "react"
import {action, observable,computed} from "mobx"
import defaultFooterBarMenuList from "@/idp/component/footer/footerBarDefaultConfig"


export type footerBarMenu = {
  position:'left'| 'right'
  component:ReactElement
  key:string
}


class FooterBarMenuControl {
  @observable footerBarMenuList:footerBarMenu[]
  constructor() {
    this.footerBarMenuList = [].concat(defaultFooterBarMenuList)
  }

  @computed get footerBarLeftMenuList(){
    return this.footerBarMenuList.filter(item=>item.position === 'left')
  }
  @computed get footerBarRightMenuList(){
    return this.footerBarMenuList.filter(item=>item.position ==='right')
  }

  @action addFooterBarMenu(newFooterBarMenu:footerBarMenu){
    const findIndex = this.footerBarMenuList.findIndex(item=>item.key===newFooterBarMenu.key)
    if(findIndex===-1){
      this.footerBarMenuList.push(newFooterBarMenu)
    }
  }
  @action removeFooterBarMenu(footerBarKey:string){
    this.footerBarMenuList = this.footerBarMenuList.filter(item=>item.key!==footerBarKey)
  }

}

export default FooterBarMenuControl
