import { ReactElement } from "react"
import { action, observable, computed, toJS } from "mobx"
import defaultFooterBarMenuList from "../../component/footer/footerBarDefaultConfig"
import { PluginsConfigInfo } from "../plugins";

export type footerBarMenu = {
    position: 'left' | 'right'
    component: ReactElement
    key: string,
    configJson?: PluginsConfigInfo,
    type?: string,
    weight?: number
}
class FooterBarMenuControl {

    @observable footerBarMenuList: footerBarMenu[];

    constructor() {
        this.footerBarMenuList = [].concat(defaultFooterBarMenuList)
    }

    @computed get footerBarLeftMenuList() {
        return this.footerBarMenuList.filter(item => item.position === 'left')
    }
    @computed get footerBarRightMenuList() {
        return this.footerBarMenuList.filter(item => item.position === 'right')
    }

    @action addFooterBarMenu(newFooterBarMenu: footerBarMenu) {
        const findIndex = this.footerBarMenuList.findIndex(item => item.key === newFooterBarMenu.key);
        if (findIndex === -1) {
            if (this.footerBarMenuList.length === 0 || !newFooterBarMenu.weight) {
                this.footerBarMenuList.push(newFooterBarMenu);
            } else {
                const _rightSideList = [...toJS(this.footerBarMenuList), newFooterBarMenu];
                let _weight_y = _rightSideList.filter(it => it.weight); // 储存有 weight
                const _weight_n = _rightSideList.filter(it => !it.weight); // 储存无 weight
                if (_weight_y.length > 0) {
                    const minWeightData = this.soltWeight(_weight_y);
                    this.footerBarMenuList = minWeightData.concat(_weight_n);
                } else {
                    this.footerBarMenuList = _weight_y.concat(_weight_n);
                }
            }
        }
    }

    @action removeFooterBarMenu(footerBarKey: string) {
        this.footerBarMenuList = this.footerBarMenuList.filter(item => item.key !== footerBarKey)
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

export default FooterBarMenuControl
