import {RouteComponentProps} from "react-router"
import React from "react"
import {action, observable} from "mobx"

export type routerMenu = {
  key: string
  name: string
  iconUnChecked: React.ComponentElement<any, any> // 未选中的icon， 默认false
  iconChecked: boolean, // 选中时的icon, 默认false
  menuClassName: {} | '' // 默认{}
  flg: boolean, // 是否显示
  component: React.ComponentType<RouteComponentProps<any>>
  needCache?:boolean
}

class RouterMenuControl {
  @observable currentRoutes:routerMenu[]

  constructor() {
    this.currentRoutes = [].concat([])
  }

  @action addRoute = (route:routerMenu)=>{
    const findIndex = this.currentRoutes.findIndex(item=>item.key===route.key)
    if(findIndex===-1){
      this.currentRoutes.push(route)
    }
  }
  @action removeRoute = (routeKey:string)=>{
    this.currentRoutes = this.currentRoutes.filter(route => route.key !== routeKey)
  }

}

export default RouterMenuControl
