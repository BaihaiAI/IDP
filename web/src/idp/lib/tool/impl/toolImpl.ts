import IRegister from '@/idp/base/index';
import { Tool } from '@/idp/lib/tool/type/tool';
import { Node } from '@/idp/lib/tool/type/node';
import HeaderGlobal, { HeaderGlobal as HeaderGlobalBean } from '@/idp/global/header';
import { action, observable } from 'mobx';
/**
 * 工具实现类
 */
export class ToolImpl<T> implements IRegister<T> {

    @observable toolMap: any = []; // 外部数据集
    @observable idpToolMap: any = []; // 内部数据集
    @observable nodeKey: Node;
    @observable renderAntdMenuHtml: React.ReactNode;
    @observable antdMenus: [];

    @observable lineNumbers: boolean = true;
    @observable collapseAllInput: boolean = false;
    @observable collapseAllOutput: boolean = false;
    @observable autoWarpOutput: boolean = true;
    @observable sneltoetsListVisible: boolean = false;

    @observable feedbackView: boolean = false;

    @observable headerGlobal: HeaderGlobalBean;

    constructor() {
        this.headerGlobal = new HeaderGlobalBean();
    }

    // 外部api方法
    @action register(name: string | number, data: T) {
        this.toolMap = this.toolMap.concat([data]);
    };

    // 只对外部的数据销毁
    @action destroyRegister(name: string) {
        this.toolMap = [];
    };

    // 内部api方法
    @action idpRegister(data: T) {
        this.idpToolMap = this.idpToolMap.concat([data]);
    }

    @action openFeedbackView() {
        this.feedbackView = true;
    }

    @action closeFeedbackView() {
        this.feedbackView = false;
    }

    @action updateLineNumbers(action) {
        this.lineNumbers = action;
    }

    @action updateCollapseAllInput(action) {
        this.collapseAllInput = action;
    }

    @action updateCollapseAllOutput(action) {
        this.collapseAllOutput = action;
    }

    @action updateAutoWarpOutput(action) {
        this.autoWarpOutput = action;
    }

    @action updateSneltoetsListVisible(action) {
        this.sneltoetsListVisible = action;
    }
}

export default new ToolImpl<Tool>();