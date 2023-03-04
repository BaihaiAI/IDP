import { action, observable } from 'mobx';
import { historyOpenProject } from '../../../store/cookie';

class FileManager {

    @observable loadNodeList = ['/']; // workspace/dir/browse接口入参中的path字段
    @observable sourceFileNode: any; // 源文件
    @observable targetFileNode: any; // 目标文件
    @observable fileOperationType: string = 'COPY' || "CUT" || "NONE";
    @observable expandedFilePaths = ['/']; // 点击文件管理器打开的文件夹目录路径path
    @observable filePath: string = ''; // 当前点击的文件夹或者文件path
    @observable fileName: string = ''; // 当前打点击文件的名称

    /**
     * 修改locastore值
     * @param sourcePath 源node path
     * @param targetPath 目标 node path
     * @param projectId 项目id
     */
    @action updateHistoryOpenFile(sourcePath, targetPath, projectId) {
        const store_historyOpenFile = projectId ? projectId : window.localStorage.getItem('historyOpenFile');
        if (store_historyOpenFile) {
            const store = JSON.parse(store_historyOpenFile);
            if (historyOpenProject) {
                store[historyOpenProject].forEach((item) => {
                    if (item.name === sourcePath) {
                        item.name = targetPath;
                        item.status = 'close';
                        return;
                    }
                });
                localStorage.setItem('historyOpenFile', JSON.stringify(store));
            }
        }
    }

    /**
     * 当前点击的文件夹或者文件path
     * @param filePath 
     */
    @action updateFilePath(filePath: string) {
        this.filePath = filePath
    }

    @action getFilePath() {
        return this.filePath;
    }

    @action updateFileName(fileName: string) {
        this.fileName = fileName;
    }

    /**
     * 点击文件管理器打开的文件夹目录路径path集合
     * @param expandedFilePaths 
     */
    @action pushExpandedFilePaths(expandedFilePaths: string) {
        this.expandedFilePaths = this.expandedFilePaths.concat([expandedFilePaths]);
    }

    /**
     * get expandedFilePaths value
     * @returns 
     */
    @action getExpandedFilePaths() {
        return this.expandedFilePaths;
    }

    /**
     * 重置
     * @param expandedFilePaths 
     */
    @action updateExpandedFilePaths(expandedFilePaths = []) {
        this.expandedFilePaths = expandedFilePaths;
    }

    /**
     * push value
     * @param loadNodeList 
     */
    @action pushLoadNodeList(loadNodeList: string) {
        this.loadNodeList = this.loadNodeList.concat([loadNodeList]);
    }

    /**
     * 重置 loadNodeList
     * @param loadNodeList 
     */
    @action updateLoadNodeList(loadNodeList: []) {
        this.loadNodeList = loadNodeList;
    }

    /**
     * 重置 sourceFileNode
     * @param sourceFileNode 
     */
    @action updateSourceFileNode(sourceFileNode: any) {
        this.sourceFileNode = sourceFileNode;
    }

    /**
     * 重置 targetFileNode
     * @param targetFileNode 
     */
    @action updateTargetFileNode(targetFileNode: any) {
        this.targetFileNode = targetFileNode;
    }

    @action getLoadNodeList() {
        return this.loadNodeList;
    }

    @action getSourceFileNode() {
        return this.sourceFileNode;
    }

    @action getTargetFileNode() {
        return this.targetFileNode;
    }

    @action updateFileOperationType(fileOperationType = 'COPY' || "CUT" || "NONE") {
        this.fileOperationType = fileOperationType;
    }

    @action getFileOperationType() {
        return this.fileOperationType;
    }
}

export default new FileManager();