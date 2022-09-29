import { action, observable } from "mobx";

class Terminal {

    topHeight: number = 40; // 头部高度
    leftFileManageWidth: number = 300; // 文件管理器宽度
    leftBarIconWidth: number = 50; // 路由宽度
    @observable next: number = 1;
    @observable openFilePath: String;
    @observable rightSideWidth: number = 0;
    @observable rightSidePanelWidth: number = 0;
    @observable workspaceHeight: number = 0;
    @observable workspaceWidth: number = 0;
    @observable terminalHeight: number = 0;
    @observable terminalWidth: number = 0;
    @observable leftSideWidth: number = -1;
    @observable terminalClientHeight: number = 0;
    @observable documentBodyClientWidth: number = document.body.clientWidth;
    @observable workspaceTabBarClickFile: string = '';
    @observable terminalVisabled: boolean = false;
    @observable rightBarOpenStatus: boolean = false;
    @observable clientHeight: number = 0;

    @action updateClientHeight(clientHeight) {
        this.clientHeight = clientHeight;
    }

    @action updateWorkspaceHeight(workspaceHeight) {
        this.workspaceHeight = workspaceHeight;
    }

    @action setRightBarOpenStatus(rightBarOpenStatus = false) {
        this.rightBarOpenStatus = rightBarOpenStatus;
    }

    @action setTerminalVisabled(terminalVisabled = false) {
        this.terminalVisabled = terminalVisabled;
    }

    @action setWorkspaceWidth(workspaceWidth) {
        this.workspaceWidth = workspaceWidth;
    }

    @action getWorkspaceWidth() {
        if (this.workspaceWidth === 0) {
            return this.workspaceWidth = this.documentBodyClientWidth + (this.leftSideWidth === -1 ? -this.leftFileManageWidth : this.leftSideWidth) - this.leftBarIconWidth - this.rightSideWidth + this.rightSidePanelWidth;
        } else {
            return this.workspaceWidth;
        }
    }

    @action setNext(next: number) {
        this.isOpenRightBar(this.openFilePath);
        if (next === 1) {
            // 默认样式
            this.workspaceHeight = document.body.clientHeight;
            this.terminalHeight = 0;
            this.terminalClientHeight = 0;
        } else if (next === 2) {
            // termina 半屏展开
            this.terminalHeight = 300;
            this.terminalWidth = this.documentBodyClientWidth + (this.leftSideWidth === -1 ? -this.leftFileManageWidth : this.leftSideWidth) - this.leftBarIconWidth - this.rightSideWidth + this.rightSidePanelWidth - 5; // 右边\
            this.workspaceHeight = document.body.clientHeight - 200 - 36 - 20 - 46 - 25;
            this.setWorkspaceWidth(this.terminalWidth);
            this.terminalClientHeight = document.body.clientHeight - this.workspaceHeight + 100 - 46 - 38 - 20 - 36; // 38: 最底部bar高度
        } else if (next === 3) {
            // 全屏展示
            this.terminalHeight = document.body.clientHeight - 60;
            this.workspaceHeight = 95;
            this.terminalWidth = this.documentBodyClientWidth + (this.leftSideWidth === -1 ? -this.leftFileManageWidth : this.leftSideWidth) - this.leftBarIconWidth - this.rightSideWidth + this.rightSidePanelWidth - 5;
            this.setWorkspaceWidth(this.terminalWidth);
            this.terminalClientHeight = document.body.clientHeight - 46 - 32 - 20 - 38; // 38: 最底部bar高度
        }
        this.next = next;
    }

    @action setLeftFileManageWidth(leftFileManageWidth) {
        this.leftSideWidth = leftFileManageWidth;
        this.terminalWidth = this.documentBodyClientWidth + leftFileManageWidth - this.leftBarIconWidth - this.rightSideWidth + this.rightSidePanelWidth - 5;
        this.setWorkspaceWidth(this.terminalWidth);
    }

    @action setRightSideWidth(rightSideWidth, load = true) {
        this.rightSideWidth = rightSideWidth;
        if (load) {
            this.terminalWidth = this.documentBodyClientWidth - this.leftBarIconWidth + ((this.leftSideWidth === -1 ? -this.leftFileManageWidth : this.leftSideWidth)) + this.rightSidePanelWidth - rightSideWidth - 5;
            this.setWorkspaceWidth(this.terminalWidth);
        };
    }

    @action setRightSidePanelWidth(rightSidePanelWidth = 0, load = true) {
        this.rightSidePanelWidth = rightSidePanelWidth;
        if (load) {
            this.terminalWidth = this.documentBodyClientWidth - this.leftBarIconWidth + ((this.leftSideWidth === -1 ? -this.leftFileManageWidth : this.leftSideWidth)) + rightSidePanelWidth - this.rightSideWidth - 5;
            this.setWorkspaceWidth(this.terminalWidth);
        };

    }

    @action setOpenFilePath(openFilePath) {
        this.openFilePath = openFilePath;
    }

    @action updateDocumentBodyClientWidth(documentBodyClientWidth) {
        this.documentBodyClientWidth = documentBodyClientWidth;
        this.terminalWidth = documentBodyClientWidth - this.leftBarIconWidth + ((this.leftSideWidth === -1 ? -this.leftFileManageWidth : this.leftSideWidth)) + this.rightSidePanelWidth - this.rightSideWidth - 5;
        this.setWorkspaceWidth(this.terminalWidth);
    }

    @action updateWorkspaceTabBarClickFile(files) {
        this.terminalWidth = this.documentBodyClientWidth - this.leftBarIconWidth + ((this.leftSideWidth === -1 ? -this.leftFileManageWidth : this.leftSideWidth)) + this.rightSidePanelWidth - (this.isOpenRightBar(files) ? 48 : 0) - 5;
        this.setWorkspaceWidth(this.terminalWidth);
    }

    isOpenRightBar(openFilePath) {
        let rightOpenFlg = false;
        const openFile = openFilePath || this.openFilePath;
        if (['ipynb', 'idpnb'].includes(openFile.slice(openFile.lastIndexOf(".") + 1))) {
            rightOpenFlg = true;
            this.rightSideWidth = 48;
        } else {
            this.rightSideWidth = 0;
        }
        return rightOpenFlg
    }
}

export default new Terminal();
