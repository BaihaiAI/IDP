import { RouteComponentProps } from "react-router"
import React from "react"
import { action, observable, toJS } from "mobx"
import { PluginsConfigInfo } from "../plugins";

export type routerMenu = {
    key: string
    name: string
    iconUnChecked: React.ComponentElement<any, any> // 未选中的icon， 默认false
    iconChecked: boolean, // 选中时的icon, 默认false
    menuClassName: {} | '' // 默认{}
    flg: boolean, // 是否显示
    component: React.ComponentType<RouteComponentProps<any>>
    needCache?: boolean,
    weight?: number,
    configJson?: PluginsConfigInfo,
    type?: string
    notNeedExact?:boolean
}

class RouterMenuControl {
    @observable currentRoutes: routerMenu[]

    constructor() {
        this.currentRoutes = [].concat([])
    }

    @action addRoute = (route: routerMenu) => {
        const findIndex = this.currentRoutes.findIndex(item => item.key === route.key);
        if (findIndex === -1) {
            if (this.currentRoutes.length === 0 || !route.weight) {
                this.currentRoutes.push(route);
            } else {
                const _rightSideList = [...toJS(this.currentRoutes), route];
                let _weight_y = _rightSideList.filter(it => it.weight); // 储存有 weight
                const _weight_n = _rightSideList.filter(it => !it.weight); // 储存无 weight
                if (_weight_y.length > 0) {
                    const minWeightData = this.soltWeight(_weight_y);
                    this.currentRoutes = minWeightData.concat(_weight_n);
                } else {
                    this.currentRoutes = _weight_y.concat(_weight_n);
                }
            }
        }
    }

    @action removeRoute = (routeKey: string) => {
        this.currentRoutes = this.currentRoutes.filter(route => route.key !== routeKey)
    }

    soltWeight(weightList = []) {
        const l = weightList.length;
        //以数组第一项为基准值
        if (l < 2) return weightList;
        const basic = weightList[0], left = [], right = [];
        for (let i = 1; i < l; i++) {
            const iv = weightList[i];
            iv.weight < basic['weight'] && left.push(iv);
            iv.weight >= basic['weight'] && right.push(iv);
        }
        //递归调用每一次把基准值放回中间
        return this.soltWeight(left).concat(basic, this.soltWeight(right))
    }
}

export default RouterMenuControl
