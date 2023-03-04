import { ReactElement } from "react"
import { action, observable, toJS } from "mobx"
import { PluginsConfigInfo } from "../plugins";

export type HeaderToolBean = {
    component: ReactElement
    key: string,
    configJson?: PluginsConfigInfo,
    type?: 'header_tool',
    weight?: number
}

class HeaderTool {

    @observable headerToolList: HeaderToolBean[];

    constructor() {
        this.headerToolList = [].concat([]);
    }

    @action addHeaderTool(tool: HeaderToolBean) {
        const findIndex = this.headerToolList.findIndex(item => item.key === tool.key);
        if (findIndex === -1) {
            if (this.headerToolList.length === 0 || !tool.weight) {
                this.headerToolList.push(tool);
            } else {
                const _rightSideList = [...toJS(this.headerToolList), tool];
                let _weight_y = _rightSideList.filter(it => it.weight); // 储存有 weight
                const _weight_n = _rightSideList.filter(it => !it.weight); // 储存无 weight
                if (_weight_y.length > 0) {
                    const minWeightData = this.soltWeight(_weight_y);
                    this.headerToolList = minWeightData.concat(_weight_n);
                } else {
                    this.headerToolList = _weight_y.concat(_weight_n);
                }
            }
        }
    }

    @action removeHeaderTool(footerBarKey: string) {
        this.headerToolList = this.headerToolList.filter(item => item.key !== footerBarKey)
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

export default HeaderTool