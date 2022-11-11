import { action, observable, toJS } from "mobx"
import rightSidesDefaultConfig from "@/idp/component/workspaceSides/rightSidesDefaultConfig"
import { ReactElement } from "react"

export type rightSideData = {
    key: string,
    icon: ReactElement,
    menuItemStyle: { [x: string]: any }
    title?: string | Function
    component: ReactElement,
    name?: string,
    weight?: number
}
class RightSideControl {

    @observable rightSideList: rightSideData[];

    constructor() {
        this.rightSideList = [].concat(rightSidesDefaultConfig);
    }

    @action addRightSide(rightSide: rightSideData) {
        const findIndex = this.rightSideList.findIndex(item => item.key === rightSide.key);
        if (findIndex === -1) {
            if (this.rightSideList.length === 0 || !rightSide.weight) {
                this.rightSideList.push(rightSide);
            } else {
                const _rightSideList = [...toJS(this.rightSideList), rightSide];
                let _weight_y = _rightSideList.filter(it => it.weight); // 储存有 weight
                const _weight_n = _rightSideList.filter(it => !it.weight); // 储存无 weight
                if (_weight_y.length > 0) {
                    const minWeightData = this.soltWeight(_weight_y);
                    this.rightSideList = minWeightData.concat(_weight_n);
                } else {
                    this.rightSideList = _weight_y.concat(_weight_n);
                }
            }
        }
    }

    @action removeRightSide(rightSideKey: string) {
        this.rightSideList = this.rightSideList.filter(item => item.key !== rightSideKey);
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

export default RightSideControl

