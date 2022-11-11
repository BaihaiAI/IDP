import ToolImpl from '@/idp/lib/tool/impl/toolImpl';
import { Tool } from '@/idp/lib/tool/type/tool';
import { message } from 'antd';
import HeaderGlobal from '@/idp/global/header';
import { Nodes } from '../common';

/**
 * hearder外部api注册方法
 */
export namespace IdpTools {

    export function registerIdpTool(nodeKey: string, data: Tool, api?: boolean) {
        if (nodeKey.length === 0) return;
        if (!Nodes.someNode(nodeKey)) {
            Object.assign(data, { nodeKey });
            api ? ToolImpl.register(data.key, data) : ToolImpl.idpRegister(data);
        } else {
            toolMsg(nodeKey);
        }
    }

    /**
     * 工具方法
     * lineNumbers: 编辑器显示行号
     * collapseAllInput: 折叠所有输入
     * collapseAllOutput 折叠所有输出
     * autoWarpOutput 输出字符是否换行，默认换行
     * sneltoetsListVisible  快捷键组建是否显示
     * feedbackView
     * @returns 
     */
    export function utils() {
        return {
            lineNumbers: ToolImpl.lineNumbers,
            collapseAllInput: ToolImpl.collapseAllInput,
            collapseAllOutput: ToolImpl.collapseAllOutput,
            autoWarpOutput: ToolImpl.autoWarpOutput,
            sneltoetsListVisible: ToolImpl.sneltoetsListVisible,
            feedbackView: ToolImpl.feedbackView,
            openFeedbackView: ToolImpl.openFeedbackView,
            closeFeedbackView: ToolImpl.closeFeedbackView,
            updateLineNumbers: ToolImpl.updateLineNumbers,
            updateCollapseAllInput: ToolImpl.updateCollapseAllInput,
            updateCollapseAllOutput: ToolImpl.updateCollapseAllOutput,
            updateAutoWarpOutput: ToolImpl.updateAutoWarpOutput,
            updateSneltoetsListVisible: ToolImpl.updateSneltoetsListVisible
        }
    }

    /**
     * 获取的历史文件
     * @returns 
     */
    export function getHistoryOpenFile() {
        return ToolImpl.headerGlobal.historyOpenFiles;
    }

    /**
     * 执行历史记录方法
     * @returns 
     */
    export function exeHistoryOpenFile() {
        return ToolImpl.headerGlobal.onTitleMouseEnter();
    }

    /**
     * 获取菜单栏keys
     * @returns 
     */
    export async function getMenuKey(): Promise<any> {
        return await HeaderGlobal.click;
    }
}

function toolMsg(name) {
    message.warn(`未找到注册的nodekey节点【${name}】`)
}