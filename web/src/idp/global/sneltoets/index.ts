import { HeaderGlobal as HeaderGlobalBean } from '@/idp/global/historyOpenFiles';
import { action, observable } from 'mobx';

class Sneltoets {

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

const sneltoets = new Sneltoets();

export default sneltoets;