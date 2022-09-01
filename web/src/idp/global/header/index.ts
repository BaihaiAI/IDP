import { action, observable } from 'mobx';
import { getHistoryOpenFile } from "@/utils/storage";
import globalData from "@/idp/global";
import { projectId } from '@/store/cookie';

export class HeaderGlobal {

    constructor() { }

    @observable historyOpenFiles: Object[] = [];
    @observable click: Promise<any> = Promise.resolve().then() ;

    @action setHistoryOpenFiles(historyOpenFiles: Object[]) {
        this.historyOpenFiles = historyOpenFiles;
    }

    @action onTitleMouseEnter() {
        const cancelObj: any = getHistoryOpenFile();
        let openFiles: Object[] = [];
        if (Array.isArray(cancelObj[projectId])) {
            for (const file of cancelObj[projectId]) {
                if (file.name) openFiles.push(file.name);
            }
            this.historyOpenFiles = openFiles;
        }
    }

    @action onClick(event) {
        this.click = new Promise((resolve, reject) => {
            resolve(event)
        });
    }

    @action isShowExportChildren() {
        let flg = false;
        const { workspaceRef } = globalData.appComponentData;
        if (workspaceRef) {
            const fileInfo = workspaceRef.state.fileInfo;
            if (fileInfo.isLeaf) {
                const fileKey = fileInfo.key;
                const values = fileKey.split(".");
                flg = values[values.length - 1] === "ipynb" || values[values.length - 1] === "idpnb";
            }
        }
        return flg;
    }

}

export default new HeaderGlobal();