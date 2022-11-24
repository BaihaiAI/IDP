import React, { useContext } from "react"
import { connect } from "react-redux"
import intl from "react-intl-universal"
import FileSaver from 'file-saver'
import { LoadingOutlined } from "@ant-design/icons"
import PubSub from "pubsub-js"
import CloneDeep from "lodash-es/cloneDeep"
import {
  message,
  Spin,
  Layout, Modal,
  notification,
} from "antd"

import "./WorkspaceLeft.less"

import IdpTerminal from '@/idp/lib/terminal';

import dataSetApi from "../../services/dataSetApi"
import { noteApiPath2 } from "@/services/httpClient"
import workspaceApi from "../../services/workspaceApi"

import { projectId } from "@/store/cookie"
import { findFileOrDirName, findFileTreeParentKey, getBrowserType } from "@/utils"

import { updateNotebookListFromTabListAsync } from "@/store/features/notebookSlice"
import { clearTabsFromList } from "@/store/features/filesTabSlice"

import AddFileNoPopInput from "./AddFileNoPopInput"
import AddFolderNoPopInput from "./AddFolderNoPopInput"
import WorkspaceLeftTitle from "./components/WorkspaceLeftTitle"
import CollapseWorkspaceLeft from "./components/CollapseWorkspaceLeft"
import FileTreeChildren from "./components/FileTreeChildren"
import AddFolderModal from "./components/Modals/AddFolderModal"
import AddFileModal from "./components/Modals/AddFileModal"
import DeleteModal from "./components/Modals/DeleteModal"
import RenameModal from "./components/Modals/RenameModal"
import FileTree from "./FileTree"
import { withRouter } from "react-router"
import { unNeedRequestErrMsg } from "@/services/extraRequestConfig"
import globalData from "idp/global"
import { observer } from "mobx-react"
import appContext from "@/context"
import CreateModel from '@/components/createOrAddAModelOrVersion/createModel'
import warenhouseApi from '../../services/warenhouseApi';
import createOrAddAModelOrVersionApi from '../createOrAddAModelOrVersion/services/createOrAddAModelOrVersionApi'
import fileManager from "@/idp/global/fileManager";
import environmentAPI from '../../services/environment'

let lastLoadTime = null

const { Sider } = Layout

let last = 0;

const loadingIcon = <LoadingOutlined style={{ fontSize: 24 }} spin />

function flattenDirFileChildren(list) {
  const fileList = []
  list.forEach((item) => {
    if (item.fileType === "DIRECTORY") {
      fileList.push(...flattenDirFileChildren(item.children))
    } else {
      fileList.push(item.browserPath)
    }
  })
  return fileList
}

const updateTreeData = (list, key, children) => {
  return list.map((node) => {
    if (node.key === key) {
      return { ...node, children }
    }
    if (node.children) {
      return {
        ...node,
        children: updateTreeData(node.children, key, children),
      }
    }
    return node
  })
}

@observer
class WorkspaceLeft extends React.Component {
  constructor(props) {
    super(props)
    this.state = {
      fileTreeDragAble: true,
      treeData: [],
      dataSourceTreeData: [],
      selectedKey: props.activePath,
      selectedName: "",
      isLeaf: true,
      spinning: false,
      filePath: "",
      deleteWarnMsg: "",

      visible: this.defaultVisible(),
      inputVisible: false,
      addAction: null,
      expandedKeys: [],

      fileInfo: this.defaultFileInfo(),
      uploadFileList: [],
      cutOrCopyKey: "",
      pasteType: "",

      //sider width
      siderWidth: 300,
      // sider show
      siderShow: false,
      downOrup: 0,
      exportFileLoading: false,
      duration: null,
      durationTitle: '',

      // 存储 创建模型所基于的 文件串 以及文件名
      modelFile: {
        string: "",
        name: ""
      },
      // 创建模型时依赖的notebook环境
      modelEnv: null,
      // 加载的时候loading
      cover: 'none',
      // create 是否显示
      createDrawerVisible: false,
      // create 某个类别数组
      category: [],

      apiLock: false,
      unzpiDisabled: true,
      checkedNode: "",
    }
    this.fileTreeRef = React.createRef()
  }

  setExportFileLoading = (exportFileLoading) => {
    this.setState({
      exportFileLoading
    })
  }

  setDragAble = (fileTreeDragAble) => {
    this.setState({
      fileTreeDragAble
    })
  }

  setTreeData(data) {
    let treeData = []
    // console.log(data)
    for (const item of data) {
      let node = {
        title: item.fileName,
        key: item.browserPath,
        isLeaf: "FILE" === item.fileType,
        fileType: item.fileType,
        active: item.active,
        sourceType: item.sourceType,
        endPoint: item.endPoint,
        bucket: item.bucket,
      }
      if (item.hasChildren) {
        node.children = this.setTreeData(item.children)
      }
      treeData.push(node)
    }
    // console.log(treeData);
    return treeData
  }

  /**
   * 重新生成新的tree
   * @param {*} treeData 数据源
   * @param {*} sourceFile 源文件路径
   * @param {*} targetFile 目标文件路径
   * @param {*} dropIndex 要插入的位置
   */
  filterDragTree = (treeData = [], sourceFile, targetFile, dropIndex, fileType = 'FILE') => {

    let sourceData = {};

    const sourceFileArray = sourceFile.split('/').filter(it => it != '');
    const sourceFileName = sourceFileArray[sourceFileArray.length - 1];

    const crecursionSourceFile = (treelist) => {
      for (let i = 0; i < treelist.length; i++) {
        if (treelist[i]['key'] === sourceFile) {
          sourceData = treelist[i];
          treelist.splice(i, 1);
          return;
        } else {
          Array.isArray(treelist[i]['children']) && crecursionSourceFile(treelist[i]['children']);
        }
      }
    };

    const fileArr = targetFile.split('/').filter(it => it != '');
    let filelocation = '';
    if (fileType === "FILE") {
      filelocation = "/" + (fileArr.splice(0, fileArr.length - 1)).join('/'); // 获取文件夹结构，不包含文件
    }
    if (fileType === "DIRECTORY") {
      filelocation = targetFile; // 获取文件夹结构，不包含文件
    }
    sourceData.key = (filelocation === '/' ? '' : filelocation) + "/" + sourceFileName; // 拼接文件路径
    let targetPath = (filelocation === '/' ? '' : filelocation) + "/" + sourceFileName; // 重新拷贝，避免数据污染

    const crecursionTargetFile = (treelist) => {
      for (let i = 0; i < treelist.length; i++) {
        if (treelist[i]['key'] === filelocation) {
          treelist[i]['children'].splice(dropIndex, 0, Object.assign(sourceData, { key: targetPath }));
          return;
        } else {
          Array.isArray(treelist[i]['children']) && crecursionTargetFile(treelist[i]['children']);
        }
      }
    };

    const crecursionTargetFileRoot = (treelist) => {
      Object.assign(sourceData, { key: targetPath });
      treelist.push(sourceData);
    }

    let fileFlg = false; // 是否执行生成新的tree逻辑
    const findFile = (treelist) => {
      for (let i = 0; i < treelist.length; i++) {
        if (treelist[i]['key'] === filelocation) {
          fileFlg = treelist[i]['children'].some(it => it.title === sourceFileName);
          return;
        } else {
          Array.isArray(treelist[i]['children']) && findFile(treelist[i]['children']);
        }
      };
    }
    findFile(treeData)
    if (!fileFlg) {
      crecursionSourceFile(treeData);
      if (filelocation === '/') {
        crecursionTargetFileRoot(treeData);
      } else {
        crecursionTargetFile(treeData);
      }
    }
    return { newTreeData: treeData, sourceData, targetPath, isfile: fileFlg }
  };

  // 文件移动
  fileDrop = (info, treeData, fileType) => {
    let canMove = false;
    let needRequest = false;
    let warnMessage = "";

    const targetFile = info.node.key; // 目标文件
    const sourceFile = info.dragNode.key; // 源文件
    const dropIndex = info.dropPosition; // 到目标文件位置

    const { newTreeData, targetPath: realTargetPath, isfile } = this.filterDragTree(treeData, sourceFile, targetFile, dropIndex, fileType);
    let data = CloneDeep(newTreeData);
    let targetPath = realTargetPath;
    if (!isfile) {
      canMove = true;
      needRequest = true;
    } else {
      warnMessage = intl.get("THERE_IS_A_FILE_WITH_THE_SAME_NAME_IN_THE_DIRECTORY");;
    }
    return { data, canMove, targetPath, needRequest, warnMessage };
  }

  dropFileTree = async (info) => {
    const { node, dragNode, dropPosition, dropToGap } = info;
    // node         代表当前被drop 的对象
    // dragNode     代表当前需要drop 的对象
    // dropPosition 代表drop后的节点位置；不准确
    // dropToGap    代表移动到非最顶级组第一个位置
    const isDir = dragNode.fileType === "DIRECTORY"

    let isRootKey = false
    let needRequest = false

    const dropKey = node.key
    const dragKey = dragNode.key
    const dropPos = node.pos.split("-")
    const originPath = dragKey
    let targetPath = ""
    let dragData

    const count = dropKey.split("").reduce((preCount, item, index) => {
      return preCount + (item === "/" ? 1 : 0)
    }, 0)
    if (count === 1) {
      isRootKey = true
    }

    // trueDropPosition: ( -1 | 0 | 1 ) dropPosition计算前的值，可以查看rc-tree源码;
    // -1 代表移动到最顶级组的第一个位置
    const trueDropPosition = dropPosition - Number(dropPos[dropPos.length - 1])

    const loop = (data, key, callback) => {
      for (let i = 0; i < data.length; i++) {
        if (data[i].key === key) {
          return callback(data[i], i, data)
        }
        if (data[i].children) {
          loop(data[i].children, key, callback)
        }
      }
    }

    const { treeData } = this.state
    let data = CloneDeep(treeData);

    loop(data, dragKey, (item, index, arr) => {
      dragData = arr.splice(index, 1)[0];
    });

    let canMove = false;
    let warnMessage = "";
    if (!dragData) {
      message.warning(intl.get("INVALID_DRAG"));
      return;
    }
    if (!dropToGap) {
      // 文件夹的拖拽 chongyin
      if (info.node.fileType === "DIRECTORY") {
        loop(data, dropKey, (item) => {
          // where to insert 示例添加到头部，可以是随意位置
          item.children = item.children || [];
          let hasSameName = item.children.some((childrenItem) => childrenItem.title === dragData.title);
          if (!hasSameName) {
            needRequest = true;
            targetPath = node.key + "/" + dragNode.name;
            dragData.key = targetPath;
            item.children.unshift(dragData);
            canMove = true;
          } else {
            canMove = false;
            warnMessage = intl.get("THERE_IS_A_FILE_WITH_THE_SAME_NAME_IN_THE_DIRECTORY");
          }
        })
        const loadNodeList = fileManager.getLoadNodeList().filter(it => it.indexOf(dragNode.key) == -1);
        fileManager.updateLoadNodeList(loadNodeList);
      } else {
        // 文件的拖拽：移动到非最顶级组第一个位置, 此处拖拽文件到目标文件，逻辑重构，思路：拖拽文件，目标文件，目标就近文件夹
        const fileDada = this.fileDrop(info, treeData, info.node.fileType);
        data = fileDada.data;
        canMove = fileDada.canMove;
        targetPath = fileDada.targetPath;
        needRequest = fileDada.needRequest;
        warnMessage = fileDada.warnMessage;
      }
    } else {
      if (isRootKey) {
        targetPath = "/" + dragNode.name;
        needRequest = true;
      }
      const fileDada = this.fileDrop(info, treeData, info.node.fileType);
      data = fileDada.data;
      canMove = fileDada.canMove;
      targetPath = fileDada.targetPath;
      needRequest = fileDada.needRequest;
      warnMessage = fileDada.warnMessage;
      const loadNodeList = fileManager.getLoadNodeList().filter(it => it.indexOf(dragNode.key) == -1);
      fileManager.updateLoadNodeList(loadNodeList);
    }
    if (canMove) {
      try {
        if (needRequest) {
          if (originPath !== targetPath) {
            unNeedRequestErrMsg()
            await workspaceApi.moveFileOrDir({ originPath, targetPath })
            if (isDir) {
              this.loadTree({ forceLoad: true })
              const fileList = this.props.fileList;
              const removeKeyList = fileList.filter((item) => item.path.startsWith(originPath)).map((item) => item.path)
              this.props.onWsDelete(removeKeyList);
            } else {
              this.setState({ treeData: data })
              const name = dragNode.name;
              this.props.onRename(originPath, targetPath, name, true);
            }
          }
        }
      } catch (e) {
        if (e.code === 41001001) {
          Modal.confirm({
            title: '有相关联的kernel正在运行 继续操作将会关闭它/它们 是否继续操作?',
            onOk: () => {
              return workspaceApi.moveFileOrDir({ originPath, targetPath, autoClose: true }).then(() => {
                if (isDir) {
                  this.loadTree({ forceLoad: true })
                  const fileList = this.props.fileList;
                  const removeKeyList = fileList.filter((item) => item.path.startsWith(originPath)).map((item) => item.path)
                  this.props.onWsDelete(removeKeyList);
                } else {
                  this.setState({ treeData: data })
                  const name = dragNode.name;
                  this.props.onRename(originPath, targetPath, name, true);
                }
              })
            },
          })
        }
      }
    } else {
      message.warning(warnMessage)
    }
  }


  onLoadData = (node) => {
    return new Promise((resolve) => {
      const path = [node.key];
      if ( node.fileType != 'database' && node.fileType !== 'database-table' && node.fileType != 'FILE' && fileManager.getLoadNodeList().indexOf(node.key) === -1) {
        fileManager.pushLoadNodeList(node.key)
      }
      node.fileType != 'database' && node.fileType != 'FILE' && node.fileType !== 'database-table' && workspaceApi.lazyLoadDirBrowse({ path }).then((res) => {
          const { treeData } = this.state
          const nodeChildren = this.setTreeData(res.data.children)
          const newTreeData = updateTreeData(
            treeData,
            node.key,
            nodeChildren
          )
          this.setState({
            treeData: node.key === '/' ? nodeChildren : newTreeData,
          })
          resolve()
        }).catch(() => {
          resolve()
        })
    })
  }

  // 重新请求workspace/dir/browse接口
  loadFileList = (pathParms) => {
    let allTreeData = [];
    let fileTreeData = [];
    const path = pathParms ? pathParms : fileManager.getLoadNodeList();
    workspaceApi.lazyLoadDirBrowse({ path }).then((result) => {
        const { children } = result.data;
        fileTreeData = this.setTreeData(children);
        allTreeData = [].concat(fileTreeData);
        this.setState({ spinning: false, treeData: allTreeData });
      }).catch((err) => {
        this.setState({ spinning: false })
      })
  }

  loadTree = async ({ forceLoad = false, loadDataSource = false } = {}) => {
    // 控制刷新频率
    // 当forceLoad为true的时候 不进行刷新频率的判断控制 需要的时候 手动传入true
    if (!forceLoad && lastLoadTime && Date.now() - lastLoadTime < 6000) {
      console.log("prevent loadTree.....")
      return
    }

    if (!forceLoad) {
      // 当执行该函数 为非强制执行的时候 才更新lastLoadTime的值
      lastLoadTime = Date.now()
    }

    this.setState({ spinning: true })

    this.loadFileList()
    if (loadDataSource) {
      this.getDataBaseListInfo()
    }
  }

  getDataBaseListInfo = () => {
    const promiseList = []
    const dataBaseAliasList = []
    const indexList = []
    Promise.all([
      dataSetApi.getDataBaseList(),
      dataSetApi.getActiveDataBaseList(),
      // dataSetApi.getActiveDataBaseList_v2(),
    ]).then((results) => {
      const databaseList = results[0].data.record.filter((item) => {
        return item.type === "database"
      })
      const activeList = results[1].data
      // const activeList_v2 = results[2].data
      // let intersect = [...activeList].filter(item => [...activeList_v2].some(prop => item['alias'] === prop['alias']))
      const newDataSetList = databaseList.map((item) => {
        const dataSourceType = item.datasource?.toLowerCase()
        return {
          ...item,
          createTime: Date.now(),
          title: item.alias,
          dataSourceType: item.datasource,
          fileType: item.type,
          key: item.alias + "-database",
          isLeaf: false,
          children: [],
          status:
            activeList.findIndex(
              (activeItem) => activeItem.alias === item.alias
            ) !== -1
              ? "0"
              : "1",
        }
      })

      newDataSetList.forEach((item, index) => {
        if (item.status === "0") {
          dataBaseAliasList.push(item.alias)
          promiseList.push(dataSetApi.getTableList(item.alias))
          indexList.push(index)
        }
      })
      if (promiseList.length !== 0) {
        Promise.all(promiseList).then((results) => {
          results.forEach((result, index) => {
            const { record: dataList, db } = result.data
            const activeDataBaseAlias = dataBaseAliasList[index]

            const field = this.getKeyFields(db)

            if (dataList.length > 0) {
              newDataSetList[indexList[index]].children = dataList
                // .slice(1)
                .map((item) => {
                  const value = item[field]
                  return {
                    title: value,
                    key: activeDataBaseAlias + "/" + value,
                    tableName: value,
                    isLeaf: true,
                    parentKey: activeDataBaseAlias,
                    fileType: "database-table",
                  }
                })
            } else {
              newDataSetList[indexList[index]].children = []
            }
            this.setState({
              dataSourceTreeData: newDataSetList,
            })
          })
        })
      } else {
        // console.log(newDataSetList)
        this.setState({
          dataSourceTreeData: newDataSetList,
        })
      }
    })
  }

  getKeyFields = (key) => {
    let field;
    switch (key) {
      case "hive2":
        field = 'tab_name'
        break
      case 'sparksql':
        field = 'tablename'
        break
      case "postgresql":
        field = "tablename"
        break
      case "mysql6":
        field = "table_name"
        break
      case "mysql":
        field = "table_name"
        break
    }

    return field
  }

  componentDidMount() {
    console.log("workspace didMount")
    lastLoadTime = null
    const { isShow } = this.props;
    isShow && this.loadTree({ loadDataSource: true })
    globalData.appComponentData.workspaceRef = this

    this.updateSelectKeysSubscriber = PubSub.subscribe(
      "updateSelectKeys",
      (msg, data) => {
        this.setState({
          selectedKey: data,
        })
      }
    )
  }
  componentWillUnmount() {
    PubSub.unsubscribe(this.updateSelectKeysSubscriber)
  }

  clickNotebookState = (notebookState) => {
    const keys = [notebookState.notebookPath]
    const info = {
      node: {
        key: notebookState.notebookPath,
        name: notebookState.fileName,
        isLeaf: true,
        fileType: "FILE",
      },
    }
    this.onSelect(keys, info)
  }


  getDataSetShowList = ({ aliasDB, tableName }) => {
    const data = { aliasDB, tableName }
    if (!aliasDB) {
      message.info('数据库数据库名为空');
      return;
    }
    if (!tableName) {
      message.info('数据库数据库表名为空');
      return;
    }
    Promise.all([
      dataSetApi.getSelectList(data),
      dataSetApi.getSchemaList(data),
    ]).then((results) => {
      const selectList = results[0].data.record
      const selectListHeader = results[0].data.schema
      const schemaList = results[1].data.record
      const schemaListHeader = results[1].data.schema
      const search = this.props.location.search
      this.props.history.push('/dataset' + search)
      PubSub.publish("getDataSetShowData", {
        aliasDB,
        tableName,
        selectList,
        selectListHeader,
        schemaList,
        schemaListHeader,
      })
    })
  }

  onSelect = (keys, info) => {
    const selectedKey = info.node.key //  相对路径 path
    const selectedName = info.node.name // 文件名 name
    const isLeaf = info.node.isLeaf // leaf
    const fileType = info.node.fileType || (isLeaf ? "FILE" : "DIRECTORY") // 文件类型
    const children = info.node.children;
    // 只对文件处理
    if ("FILE" === fileType) {
      IdpTerminal.setOpenFilePath(selectedKey);
      let _next = 1;
      const next = IdpTerminal.next;
      if (next == 1) _next = next;
      if (next == 2) _next = next;
      if (next == 3) _next = 2;
      const theFileType = keys[0].slice(keys[0].lastIndexOf(".") + 1);
      if (theFileType === 'ipynb' || theFileType === 'idpnb') {
        IdpTerminal.setRightSideWidth(0);
        IdpTerminal.setNext(_next);
      } else {
        IdpTerminal.setRightSideWidth(48);
        IdpTerminal.setNext(_next);
      }
    }
    IdpTerminal.setTerminalVisabled(true);

    const visible = this.state.visible
    visible.renameValue = selectedName
    this.setState({ visible })

    const fileInfo = this.state.fileInfo
    fileInfo.key = selectedKey
    fileInfo.selectedName = selectedName
    fileInfo.isLeaf = isLeaf
    fileInfo.fileType = fileType
    this.setState({ fileInfo }, () => {
      PubSub.publish("refreshHeader")
    })
    this.setState({ selectedKey,selectedName: selectedName,isLeaf })
    if (isLeaf) {
      if (fileType === "database-table") {
        const aliasDB = info.node.parentKey
        const tableName = info.node.tableName
        this.getDataSetShowList({ aliasDB, tableName })

      } else if (fileType === "FILE") {
        this.props.onWsSelected(selectedKey, selectedName)
        this.setState({ deleteWarnMsg: "" })
      }
    } else {
      if (fileType === "database" && children.length === 0) {
        const status = info.node.status
        if (status === '1') {
          message.warning('数据库已断开,需要重新连接')
        } else {
          message.warning(
            intl.get("THERE_ARE_NO_TABLES_UNDER_THE_CURRENT_DATA_SOURCE"),
            1.5
          )
        }
      }
      this.setState({
        deleteWarnMsg: intl.get("DELETE_CONFIRM_DESCRIPTION_EX"),
      })
    }

    let path = "/"
    if ("/" !== selectedKey) {
      if (isLeaf) {
        path = this.getParentPath(`${selectedKey}`)
      } else {
        path = `${selectedKey}`
      }
    }
    this.setState({ filePath: path },()=> {
       const filePath = fileManager.getFilePath();
       if ( !info.node.expanded ) {
        if ( path == filePath ) {
          this.onLoadData(info.node);
        }
        fileManager.pushExpandedFilePaths(path);
       } else {
          const expandedFilePathsList = fileManager.getExpandedFilePaths();
          const exFile = expandedFilePathsList.filter( it=> it != path );
          fileManager.updateExpandedFilePaths(exFile);
       }
       fileManager.updateFilePath(path);
    });
  }

  // 此处涉及到游览器下载问题，需要进行分情况处理下载功能，目前只对于火狐用FileSaver实现导出，其他游栏器用streamSaver实现导出
  fetchDownloadOrExportFile = (url, fileName, isLoading) => {
    new Promise((resolve) => {
      const browser = getBrowserType(); // 获取游览器类型
      resolve(browser);
    }).then(browRes => {
      if (isLoading) {
        this.setExportFileLoading(true)
      }
      fetch(url).then((res) => {
        setTimeout(() => {
          res.blob().then(resFiles => {
            this.setExportFileLoading(false)
            FileSaver.saveAs(resFiles, `${fileName}`);
          });
        });
      }).catch((error) => {
        this.setExportFileLoading(false)
        console.log(intl.get("DOWNLOAD_FAILED"), error)
      });
    })
  }

  exportTypeClick = (outputType) => {
    return ({ event, props }) => {
      if (props.info.isLeaf) {
        event.preventDefault()
        event.stopPropagation()

        const selectedKey = props.info.key
        const lastpos = selectedKey.lastIndexOf("/")
        let fileName = selectedKey.substring(lastpos + 1) //设置文件名称

        const url = `${noteApiPath2}/workspace/file/exportAs?path=${encodeURIComponent(selectedKey)}&outputType=${outputType}&projectId=${projectId}`
        const newFileName =
          fileName.split(".")[0] +
          "." +
          (outputType === "python" ? "py" : outputType)
        let isLoading
        if (outputType === 'html') {
          isLoading = true
        }

        this.fetchDownloadOrExportFile(url, newFileName, isLoading)
      } else {
        console.log("需要选择文件")
        message.warning(intl.get("SELECT_FILE"))
      }
    }
  }

  exportFolderClick = ({ event, props }) => {
    const url = `${noteApiPath2}/workspace/dir/export`
    this.downloadFolder(url, props.info.key, props.info.name)
  }

  openNotification = (key, msg, durationTitle, duration, type) => {
    const config = {
      key,
      message: msg,
      description: durationTitle,
      duration: duration
    }
    notification[type](config);
  };

  downloadFolder(url, folderName, name) {
    console.log(url)
    const fileName = name ? name : globalData.appComponentData.projectInfo.name + ".zip"
    const data = {
      exportPath: folderName,
      projectId: Number(projectId),
      projectName: name ? name : globalData.appComponentData.projectInfo.name
    }
    this.openNotification('fileExport', '文件导出', `${fileName}正在压缩中...`, null, 'info');
    fetch(url, { method: "POST", body: JSON.stringify(data), headers: { "Content-Type": "application/json" } }).then((res) => {
      res.blob().then(resFiles => {
        notification.close('fileExport');
        this.openNotification('fileExport', '文件导出', `${fileName}已经下载完成，请在浏览器中查看`, 3, 'success');
        FileSaver.saveAs(resFiles, `${fileName}`);
      });
    }).catch((error) => {
      notification.close('fileExport');
      this.openNotification('fileExport', '文件导出', `${fileName}文件导出失败`, 3, 'error');
    })
  }

  onExportClick = ({ event, props }) => {
    if (props.info.isLeaf) {
      event.preventDefault()
      event.stopPropagation()
      const selectedKey = props.info.key
      const lastpos = selectedKey.lastIndexOf("/")
      let fileName = selectedKey.substring(lastpos + 1) //设置文件名称
      const url = `${noteApiPath2}/workspace/file/download?path=${encodeURIComponent(selectedKey)}&projectId=${projectId}`
      // this.fetchDownloadOrExportFile(url, fileName)
      this.aTagDownload(url, fileName)
    } else {
      console.log("需要选择文件")
      message.warning(intl.get("SELECT_FILE"))
    }
  }

  aTagDownload(url, fileName) {
    console.log(url, fileName)
    const el = document.createElement('a');
    el.style.display = 'none';
    el.setAttribute('target', '_blank');
    fileName && el.setAttribute('download', fileName);
    el.href = url;
    console.log(el);
    document.body.appendChild(el);
    el.click();
    document.body.removeChild(el);
  }

  menu_onExportClick = (outputType) => {
    let fileInfo = {}
    if (null == this.state.fileInfo) {
      message.warning(intl.get("SELECT_FILE"))
      return
    } else {
      fileInfo = this.state.fileInfo
    }

    if (fileInfo.isLeaf) {
      const selectedKey = fileInfo.key
      const lastpos = selectedKey.lastIndexOf("/")
      let fileName = selectedKey.substring(lastpos + 1) //设置文件名称

      if (!outputType) {
        const url = `${noteApiPath2}/workspace/file/download?path=${encodeURIComponent(selectedKey)}&projectId=${projectId}`
        this.fetchDownloadOrExportFile(url, fileName)
      } else {
        const url = `${noteApiPath2}/workspace/file/exportAs?path=${encodeURIComponent(selectedKey)}&outputType=${outputType}&projectId=${projectId}`
        const newFileName =
          fileName.split(".")[0] +
          "." +
          (outputType === "python" ? "py" : outputType)

        this.fetchDownloadOrExportFile(url, newFileName)
      }
    } else {
      console.log("需要选择文件")
      message.warning(intl.get("SELECT_FILE"))
    }
  }


  defaultVisible = () => {
    return {
      delete: false,
      addFolder: false,
      folderInputDisabled: false,
      folderValue: "newFolder",
      addFile: false,
      fileInputDisabled: false,
      fileValue: "newFile.idpnb",
      rename: false,
      renameInputDisabled: false,
      renameValue: "",
      displayWarning: "none",
      fileNameValidator: "",
      confirmLoading: false,
    }
  }

  defaultFileInfo = () => {
    return {
      key: "",
      selectedName: "",
      isLeaf: true,
      fileType: "DIRECTORY",
    }
  }

  resetVisible = () => {
    this.setState({ visible: this.defaultVisible() })
  }

  setInputValue = (e, type) => {
    const value = e.target ? e.target.value : e
    const action = type ? type : this.state.addAction
    let visible = this.state.visible
    switch (action) {
      case "folder":
        visible.folderValue = value
        break
      case "file":
        visible.fileValue = value
        break
      case "rename":
        visible.renameValue = value
        break
      default:
        break
    }
    console.log(visible.renameValue);
    this.setState({ visible })
  }

  setSelect = (props) => {
    if (props) {
      this.setState({
        selectedKey: props.key,
        selectedName: props.name,
        isLeaf: props.isLeaf,
      })
    }
  }

  checkFileName = (visible, value, key, isLeaf, isFile) => {
    if (value.indexOf(" ") >= 0) {
      // 加入下部代码 新建文件文件名无法加入空格
      // visible.displayWarning = ""
      // visible.fileNameValidator = intl.get("FILE_NAME_INVALID_2")
      // this.setState({ visible })
      // return false
    } else if (null === value.match(/^(?!\.)[^\\:\*\?"<>\|]{1,255}$/)) {
      visible.displayWarning = ""
      visible.fileNameValidator = intl.get("FILE_NAME_INVALID_1")
      this.setState({ visible })
      return false
    } else if (value.length > 50) {
      visible.displayWarning = ""
      visible.fileNameValidator = "文件或文件夹名长度不能超过50"
      this.setState({
        visible
      })
      return false
    } else {
      const arr = this.state.treeData
      let childArr = [...arr]
      if ("/" !== key) {
        const dir = isLeaf ? key.slice(0, key.lastIndexOf("/")) : key
        if (dir !== "") {
          childArr = this.findChildren(dir, arr)
        }
      }
      for (const item of childArr) {
        if (value === item.title) {
          visible.displayWarning = ""
          visible.fileNameValidator = isFile
            ? intl.get("FILE_NAME_INVALID_3")
            : intl.get("FILE_NAME_INVALID_4")
          this.setState({ visible, treeData: this.state.treeData })

          return false
        }
      }
    }
    return true
  }

  handleIsFileTree = () => {
    const { fileInfo } = this.state
    if (
      fileInfo.fileType === "database" ||
      fileInfo.fileType === "database-table" ||
      fileInfo.key === "/storage-service"
    ) {
      message.warning(
        intl.get("CURRENTLY_SELECTED_IS_NOT_A_FILE_DIRECTORY_OR_FILE") +
        " " +
        intl.get("THIS_OPERATION_IS_NOT_POSSIBLE"),
        1.5
      )
      return false
    }
    return true
  }

  addFolder = ({ event, props }) => {
    if (!this.handleIsFileTree()) {
      return
    }
    props && this.setSelect(props.info)
    let visible = this.state.visible
    visible.addFolder = true
    this.setState({ visible })
  }
  addFile = ({ event, props }) => {
    if (!this.handleIsFileTree()) {
      return
    }
    props && this.setSelect(props.info)
    //const pos = this.findChildren(props.info.key, this.state.treeData)
    let visible = this.state.visible
    visible.addFile = true
    this.setState({ visible })
  }

  setFileValue = (value) => {
    let visible = this.state.visible
    visible.fileValue = value
    this.setState({ visible })
  }

  submitAddFolder = () => {
    const _this = this //先存一下this，以防使用箭头函数this会指向我们不希望它所指向的对象。
    let visible = _this.state.visible
    const value = visible.folderValue
    if ("" === value ||
      !this.checkFileName(
        visible,
        value,
        _this.state.selectedKey,
        _this.state.isLeaf,
        false
      )
    ) {
      return
    }

    visible.folderInputDisabled = true
    visible.confirmLoading = true
    _this.setState({ visible })
    let path = "/"
    if ("/" !== _this.state.selectedKey) {
      if (_this.state.isLeaf) {
        path = _this.getParentPath(`${_this.state.selectedKey}`)
      } else {
        path = `${_this.state.selectedKey}/`
      }
    }

    const params = {
      path: `${path}${value}`,
    }
    this.throttle(function () {
      workspaceApi.dirNew(params).then(function (response) {
        _this.setDragAble(true)
        _this.resetVisible()
        _this.setState({ inputVisible: false })
        _this.loadTree({ forceLoad: true })
      }).catch(function (error) {
        _this.setDragAble(true)
        _this.resetVisible()
        _this.setState({ inputVisible: false })
        message.error(intl.get("ADD_FAILED"))
      })
    })
  }

  // 利用时间差进行防抖
  throttle = (callback) => {
    let now = new Date().getTime();;
    if (now - last > 1000) {
      last = new Date().getTime();
      callback()
    }
  }

  submitAddFile = () => {
    const _this = this //先存一下this，以防使用箭头函数this会指向我们不希望它所指向的对象。
    let visible = _this.state.visible
    const value = visible.fileValue
    if (
      "" === value ||
      !this.checkFileName(
        visible,
        value,
        _this.state.selectedKey,
        _this.state.isLeaf,
        true
      )
    ) {
      return
    }

    visible.fileInputDisabled = true
    visible.confirmLoading = true
    _this.setState({ visible })

    let path = "/"
    if ("/" !== _this.state.selectedKey) {
      if (_this.state.isLeaf) {
        path = _this.getParentPath(`${_this.state.selectedKey}`)
      } else {
        path = `${_this.state.selectedKey}/`
      }
    }

    const filePath = `${path}${value}`
    workspaceApi.fileNew({ path: filePath }).then(function (response) {
      _this.resetVisible()
      setTimeout(() => {
        _this.onSelect([filePath], {
          node: {
            key: filePath,
            name: value,
            isLeaf: true,
          },
        })
      });
      _this.setDragAble(true)
      _this.setState({ inputVisible: false })
      _this.loadTree({ forceLoad: true })
    }).catch(function (error) {
      _this.setDragAble(true)
      _this.resetVisible()
      _this.setState({ inputVisible: false })
      message.error(intl.get("ADD_FAILED"))
    })
  }

  getParentPath = (selectPath) => {
    let parent = selectPath.substring(0, selectPath.lastIndexOf("/"))
    return parent + "/"
  }

  rename = ({ event, props }) => {
    props && this.setSelect(props.info)
    // this.setSelect(props.info);
    let visible = this.state.visible
    visible.rename = true
    visible.renameValue = props.info.name
    this.setState({ visible })
  }
  menu_rename = () => {
    let visible = this.state.visible
    if (null != visible.renameValue && visible.renameValue !== "") {
      let visible = this.state.visible
      visible.rename = true
      this.setState({ visible })
    } else {
      message.warning(intl.get("SELECT_FILE"))
    }
  }

  submitRename = () => {
    const _this = this //先存一下this，以防使用箭头函数this会指向我们不希望它所指向的对象。
    let visible = _this.state.visible
    const value = visible.renameValue
    if (
      "" === value ||
      !this.checkFileName(
        visible,
        value,
        _this.state.selectedKey,
        true,
        _this.state.isLeaf
      )
    ) {
      return
    }

    visible.renameInputDisabled = true
    visible.confirmLoading = true
    _this.setState({ visible })

    let path = "/"
    if ("/" !== _this.state.selectedKey) {
      path = `${_this.state.selectedKey}`
    }

    let parentPath = _this.getParentPath(path)

    const dest = _this.setSuffix(value)
    unNeedRequestErrMsg()

    const realPath = _this.state.selectedKey
    const isFile = _this.state.isLeaf
    workspaceApi
      .fileRename({
        source: _this.state.selectedName,
        path: parentPath,
        dest: value
      })
      .then(function (response) {
        if (!isFile) {
          const loadNodeList = fileManager.getLoadNodeList().filter(item => item !== path);
          fileManager.updateLoadNodeList(loadNodeList);
        }

        _this.resetVisible()
        _this.loadTree({ forceLoad: true })
        _this.setState({ inputVisible: false })
        _this.props.onRename(
          _this.state.selectedKey,
          `${parentPath}${dest}`,
          value,
          _this.state.isLeaf
        )
        _this.setRoot()
        // message.success(intl.get('RENAME_SUCCEEDED'));
      })
      .catch(function (error) {
        if (error.code === 41001001) {
          Modal.confirm({
            title: '有相关联的kernel正在运行 继续将会关闭它/它们 是否继续操作?',
            onOk: () => {
              return workspaceApi
                .fileRename({
                  source: _this.state.selectedName,
                  path: parentPath,
                  value,
                  autoClose: true
                })
                .then(function (response) {
                  _this.resetVisible()
                  _this.loadTree({ forceLoad: true })
                  _this.setState({ inputVisible: false })

                  _this.props.onRename(
                    _this.state.selectedKey,
                    `${parentPath}${dest}`,
                    value,
                    _this.state.isLeaf
                  )
                  _this.setRoot()
                  // message.success(intl.get('RENAME_SUCCEEDED'));
                }).catch(() => {
                  _this.resetVisible()
                  _this.setState({ inputVisible: false })
                })
            },
            onCancel: () => {
              _this.resetVisible()
              _this.setState({ inputVisible: false })
            }
          })

        } else {
          _this.resetVisible()
          _this.setState({ inputVisible: false })
        }
      })
  }
  setSuffix = (value) => {
    const input_suffix = value.match(/\.\w+$/)
    const cur_suffix = this.state.selectedName.match(/\.\w+$/)
    let val = value
    if (cur_suffix) {
      //目标为文件
      val = input_suffix ? value : value + cur_suffix[0]
    } else {
      val = value
    }
    return val
  }

  delete = ({ event, props }) => {
    if (props) {
      this.setSelect(props.info)
    } else {
      if ("/" === this.state.selectedKey) {
        message.warning(intl.get("DELETE_ERROR_1"))
        return
      }
    }
    const expandedFilePaths = fileManager.getExpandedFilePaths();
    const newExpandedFilePaths = expandedFilePaths.filter( it => it.indexOf(props.info.key) == -1);
    const loadNodeList = fileManager.getLoadNodeList();
    const newLoadNodeList = loadNodeList.filter( it => it.indexOf(props.info.key) == -1);
    fileManager.updateExpandedFilePaths(newExpandedFilePaths);
    fileManager.updateLoadNodeList(newLoadNodeList);
    const filePath = fileManager.getFilePath();
    if ( filePath === props.info.key ) fileManager.updateFilePath('');
    let visible = this.state.visible
    visible.delete = true
    this.setState({ visible })
  }

  submitDelete = () => {
    if (!this.state.visible.delete) return false;
    const _this = this //先存一下this，以防使用箭头函数this会指向我们不希望它所指向的对象。
    let visible = _this.state.visible
    visible.confirmLoading = true
    _this.setState({ visible })

    unNeedRequestErrMsg()
    const path = _this.state.selectedKey
    const isFile = _this.state.isLeaf
    workspaceApi
      .wdelete({
        path,
        isFile,
      })
      .then(function (response) {
        if (!isFile) {
          const loadNodeList = fileManager.getLoadNodeList().filter(item => item !== path);
          fileManager.updateLoadNodeList(loadNodeList);
        }

        _this.resetVisible()
        _this.loadTree({ forceLoad: true })
        if (_this.state.isLeaf) {
          _this.props.onWsDelete([_this.state.selectedKey])
        } else {
          const childrenList = response.data.children
          const keys = flattenDirFileChildren(childrenList)
          _this.props.onWsDelete(keys)
        }
        _this.setRoot()
        message.success(intl.get("DELETE_SUCCEEDED"))
      })
      .catch(function (error) {
        if (error.code === 41001001) {
          Modal.confirm({
            title: '有相关联的kernel正在运行 继续将会关闭它/它们 是否继续操作?',
            onOk: () => {
              return workspaceApi
                .wdelete({
                  path: _this.state.selectedKey,
                  isFile: _this.state.isLeaf,
                  autoClose: true
                }).then((function (response) {

                  _this.resetVisible()
                  _this.loadTree({ forceLoad: true })
                  if (_this.state.isLeaf) {
                    _this.props.onWsDelete([_this.state.selectedKey])
                  } else {
                    const childrenList = response.data.children
                    const keys = flattenDirFileChildren(childrenList)
                    _this.props.onWsDelete(keys)
                  }
                  _this.setRoot()
                  message.success(intl.get("DELETE_SUCCEEDED"))
                })).catch(() => {
                  _this.resetVisible()
                })
            },
            onCancel: () => {
              _this.resetVisible()
            },
          })
        } else {
          _this.resetVisible()
        }

      })
  }

  // 删除节点快捷键
  handleDeleteNodeKey = () => {
    this.delete({
      props: {
        info: {
          key: this.state.selectedKey,
          name: this.state.selectedName,
          isLeaf: this.state.isLeaf,
        },
      },
    })
  }


  findChildren = (key, tree) => {
    for (const node of tree) {
      if (!node.isLeaf) {
        if (key === node.key) {
          return node.children ? node.children : []
        } else if (node.children) {
          const keys = this.findChildren(key, node.children)
          if (keys.length > 0) return keys
        }
      }
    }
    return []
  }

  /*************The Action form MouseRight event******************/
  addNewFolder = ({ event, props }) => {
    this.setDragAble(false)
    const template = {
      title: (
        <AddFolderNoPopInput
          placeholder={intl.get("ADD_FOLDER_PLACEHOLDER")}
          defaultValue={this.state.visible.folderValue}
          onChange={(e) => this.setInputValue(e, "folder")}
          onBlur={this.addNewFileInputBlur}
          onPressEnter={this.submitAddFolder}
          disabled={this.state.visible.folderInputDisabled}
          style={{ width: 150, height: 20, lineHeight: 20 }}
          treeData={this.state.treeData}
          isLeaf={this.state.isLeaf}
          selectedKey={this.state.selectedKey}
        />
      ),
      isLeaf: false,
      key: "temp" + Math.floor(Math.random(1000) * 10000) + "newFile",
      isTemp: true,
    }
    props && this.setSelect(props.info)
    this.insertTempFile(props.info, this.state.treeData, false, template)
    this.setState({ addAction: "folder" })
  }
  addNewFile = ({ event, props }) => {
    this.setDragAble(false)
    const template = {
      title: (
        <AddFileNoPopInput
          placeholder={intl.get("ADD_FILE_PLACEHOLDER")}
          setFileValue={this.setFileValue}
          disabled={this.state.visible.fileInputDisabled}
          onPressEnter={this.submitAddFile}
          onBlur={this.addNewFileInputBlur}
          treeData={this.state.treeData}
          selectedKey={this.state.selectedKey}
          defaultValue={this.state.visible.fileValue}
          style={{ width: 150, height: 20, lineHeight: 20 }}
        />
      ),
      isLeaf: true,
      key: "temp" + Math.floor(Math.random(1000) * 10000) + "newFile.idpnb",
      isTemp: true,
    }
    props && this.setSelect(props.info)
    this.insertTempFile(props.info, this.state.treeData, true, template)
    this.setState({ addAction: "file" })
  }

  //Input失去焦点时撤销操作
  addNewFileInputBlur = () => {
    setTimeout(() => {
      this.setDragAble(true)
    }, 300)
    this.delTempFile(this.state.treeData) //删除节点
    this.resetVisible() //重置状态
  }

  /*  插入临时占位节点
   *  info     : 触发事件的节点数据(tree)
   *  tree     : 当前树形数据
   *  type     : 要插入的类型{true: 文件类型，false: 目录类型}
   *  template : 占位模版
   */

  insertTempFile = (info, tree, type, template) => {
    const _this = this
    //新用户
    if (info.isRoot) {
      template.key = info.key.replace(
        /\/?\w+\.\w+$/g,
        Math.floor(Math.random(10000) * 10000) + "/newFile.idpnb"
      )
      tree.unshift(template)
      this.setState({
        treeData: this.state.treeData,
      })
      return
    }
    for (let i = 0; i < tree.length; i++) {
      let node = tree[i]
      if (node.isLeaf) {
        if (info.key === node.key) {
          template.key = info.key.replace(
            /\/?\w+\.\w+$/g,
            Math.floor(Math.random(10000) * 10000) + "/newFile.idpnb"
          )
          tree.splice(i + 1, 0, template)

          this.setState({
            treeData: this.state.treeData,
          })
          return
        }
      } else {
        if (info.key === node.key) {
          /*需要重新计算位置 */
          template.key =
            info.key +
            "/" +
            Math.floor(Math.random(10000) * 10000) +
            "newFile.idpnb"
          if (node.children) {
            node.children.unshift(template)
          } else {
            node.children = []
            node.children.push(template)
          }
          this.setState(
            {
              treeData: this.state.treeData,
            },
            function () {
              this.setState({
                expandedKeys: [info.key],
              })
            }
          )
          return
        }
        node.children &&
          this.insertTempFile(info, node.children, type, template)
      }
    }
  }



  delTempFile = (tree) => {
    for (let i = 0; i < tree.length; i++) {
      let node = tree[i]
      if (node.isTemp) {
        tree.splice(i, 1)
        this.setState({
          treeData: this.state.treeData,
        })
        return
      }
      node.children && this.delTempFile(node.children)
    }
  }


  uploadSubmit = (filePath, file, fileType) => {
    const _this = this //先存一下this，以防使用箭头函数this会指向我们不希望它所指向的对象。
    const name = file.name //文件名
    const size = file.size //总大小

    const shardSize = 2 * 1024 * 1024 //以2MB为一个分片
    let shardCount = Math.ceil(size / shardSize) //总片数
    shardCount = shardCount === 0 ? 1 : shardCount

    const fileKey =
      fileType === "file"
        ? filePath + file.name
        : filePath + file.webkitRelativePath

    if (fileType === "file") {
      if (filePath.substring(filePath.length - 1) === "/") {
        filePath = filePath.substring(0, filePath.length - 1)
      }
    } else {
      filePath =
        filePath +
        file.webkitRelativePath.slice(
          0,
          file.webkitRelativePath.lastIndexOf("/")
        )
    }

    for (let i = 0; i < shardCount; ++i) {
      //计算每一片的起始与结束位置
      let start = i * shardSize
      let end = Math.min(size, start + shardSize)

      //构造一个表单，FormData是HTML5新增的
      let form = new FormData()
      form.append("datafile", file.slice(start, end)) //slice方法用于切出文件的一部分
      form.append("name", name)
      form.append("total", shardCount) //总片数
      form.append("index", i + 1) //当前是第几片
      form.append("filePath", filePath) //目录

      workspaceApi
        .uploadFile(form)
        .then(function (response) {
          if (response.data === "over") {
            // message.success(`${name} ${intl.get('UPLOAD_SUCCEEDED')}`);
            let uploadFileList = [..._this.state.uploadFileList]
            let index = -1
            for (let i = 0; i < uploadFileList.length; i++) {
              if (uploadFileList[i].key === fileKey) {
                index = i
                break
              }
            }
            if (index > -1) {
              uploadFileList.splice(index, 1)
              _this.setState({ uploadFileList })
            }
            _this.loadTree()
          } else {
            let uploadFileList = [..._this.state.uploadFileList]
            for (let item of uploadFileList) {
              if (item.key === fileKey) {
                item.completeSize += shardSize
              }
            }
            _this.setState({ uploadFileList })
          }
        })
        .catch(function (error) {
          console.log(error)
          message.error(`${name} ${intl.get("UPLOAD_FAILED")}`)
          let uploadFileList = [..._this.state.uploadFileList]
          let index = -1
          for (let i = 0; i < uploadFileList.length; i++) {
            if (uploadFileList[i].key === fileKey) {
              index = i
              break
            }
          }
          if (index > -1) {
            uploadFileList.splice(index, 1)
          }
          _this.setState({ uploadFileList })
        })
    }
  }

  sleep = (ms) => {
    return new Promise(function (resolve, reject) {
      setTimeout(resolve, ms)
    })
  }
  uploadFolder = async (path, files) => {
    let uploadFileList = [...this.state.uploadFileList]
    for (const file of files) {
      // if (file.size < 4 * 1024 * 1024) continue
      const key = path + file.webkitRelativePath
      const fileInfo = {
        key: key,
        name: file.name,
        totalSize: file.size,
        completeSize: 0,
      }
      uploadFileList.push(fileInfo)
    }
    this.setState({ uploadFileList })
    for (const file of files) {
      // 判断文件名中是否有空格
      // if (file.name.indexOf(' ') !== -1) {
      //   message.warning(file.name + ' ' + intl.get('UPLOAD_ERROR_2'));
      //   continue;
      // }
      this.uploadSubmit(path, file, "folder")
      await this.sleep(50)
    }
    document.getElementById("chooseFolder").value = ""
  }

  uploadFiles = (path, files, childArr) => {
    let fileList = []
    for (const file of files) {
      // 判断文件名中是否有空格
      // if (file.name.indexOf(' ') !== -1) {
      //   message.warning(file.name + ' ' + intl.get('UPLOAD_ERROR_2'));
      //   continue;
      // }

      // 判断文件名是否冲突
      let isConflict = false

      // 判断有没有正在上传
      const fileKey = path + file.name
      for (const item of this.state.uploadFileList) {
        if (item.key === fileKey) {
          message.warning(
            file.name + " " + intl.get("UPLOAD_ERROR_SAME_FILE_1")
          )
          isConflict = true
          break
        }
      }
      if (isConflict) continue

      // 判断目录中是否已经存在
      for (const item of childArr) {
        if (file.name === item.title) {
          message.warning(file.name + " " + intl.get("UPLOAD_ERROR_SAME_FILE"))
          isConflict = true
          break
        }
      }

      if (isConflict) continue
      fileList.push(file)
    }

    let uploadFileList = [...this.state.uploadFileList]
    for (const file of fileList) {
      if (file.size < 4 * 1024 * 1024) continue
      const key = path + file.name
      const fileInfo = {
        key: key,
        name: file.name,
        totalSize: file.size,
        completeSize: 0,
      }
      uploadFileList.push(fileInfo)
    }
    this.setState({ uploadFileList })
    for (const file of fileList) {
      this.uploadSubmit(path, file, "file")
    }
    document.getElementById("chooseFiles").value = ""
  }

  handleFileChange = (fileType, domId) => {
    const files = document.getElementById(domId).files
    if (files.length === 0) return

    // 获取上传文件所在目录
    let path = "/"
    if ("/" !== this.state.selectedKey) {
      if (this.state.isLeaf) {
        path = this.getParentPath(`${this.state.selectedKey}`)
      } else {
        path = `${this.state.selectedKey}/`
      }
    }

    // 获取当前目录下的文件/文件夹列表，用来判断是否冲突
    const key = this.state.selectedKey
    const arr = this.state.treeData
    let childArr = [...arr]
    if ("/" !== key) {
      const dir = this.state.isLeaf ? key.slice(0, key.lastIndexOf("/")) : key
      if (dir !== "") {
        childArr = this.findChildren(dir, arr)
      }
    }

    if (fileType === "file") {
      this.uploadFiles(path, files, childArr)
    } else {
      const webkitRelativePath = files[0].webkitRelativePath
      const folder = webkitRelativePath.slice(
        0,
        webkitRelativePath.indexOf("/")
      )
      // 判断文件夹名是否冲突
      for (const item of childArr) {
        if (folder === item.title) {
          message.warning(folder + " " + intl.get("UPLOAD_ERROR_SAME_FILE"))
          return false
        }
      }
      this.uploadFolder(path, files)
    }
  }

  setRoot = () => {
    this.setState({
      selectedKey: "/",
      selectedName: "",
      isLeaf: false,
      fileInfo: this.defaultFileInfo()
    })
  }


  handlerCutFileKey = () => {
    const props = {
      info: this.state.fileInfo
    }
    this.handlerCutFile({ props })
  }

  handlerCopyFileKey = (event) => {
    const props = {
      info: this.state.fileInfo
    }
    this.handlerCopyFile({ props })
  }

  handlerStickFileKey = () => {
    const props = {
      info: this.state.fileInfo
    }
    this.handlerStickFile({ props })
  }



  handlerCutFile = ({ props }) => {
    fileManager.updateSourceFileNode(props.info);
    fileManager.updateFileOperationType('CUT');
    this.setSelect(props.info)
    this.setState({
      cutOrCopyKey: props.info.key,
      pasteType: "cut",
    })
    message.success("剪切成功")
  }
  handlerCopyFile = ({ props }) => {
    fileManager.updateSourceFileNode(props.info);
    fileManager.updateFileOperationType('COPY');
    this.setSelect(props.info)
    this.setState({
      cutOrCopyKey: props.info.key,
      pasteType: "copy",
    })
    message.success("复制成功")
  }

  handlerStickFile = ({ props }) => {
    const selectKey = props.info.key
    let originParentKey
    let targetParentKey
    if (props.info.fileType === "DIRECTORY") {
      targetParentKey = selectKey
    } else {
      targetParentKey = findFileTreeParentKey(selectKey)
    }
    const fileOrDirName = findFileOrDirName(this.state.cutOrCopyKey)
    originParentKey = findFileTreeParentKey(this.state.cutOrCopyKey)
    const originPath = originParentKey + "/" + fileOrDirName
    const targetPath = targetParentKey + "/" + fileOrDirName

    const loop = (data, key, callback) => {
      for (let i = 0; i < data.length; i++) {
        if (data[i].key === key) {
          return callback(data[i], i, data)
        }
        if (data[i].children) {
          loop(data[i].children, key, callback)
        }
      }
    }

    const data = CloneDeep(this.state.treeData)
    let canMove = true
    let warnMessage
    let hasSameName
    loop(data, targetParentKey, (item) => {
      if (item.fileType === "DIRECTORY") {
        item.children = item.children || []
        hasSameName = item.children.some(
          (childrenItem) => childrenItem.title === fileOrDirName
        )
        if (hasSameName) {
          canMove = false
          warnMessage = intl.get("THERE_IS_A_FILE_WITH_THE_SAME_NAME_IN_THE_DIRECTORY")
        }
      }
    })

    if (this.state.pasteType === "cut") {
      unNeedRequestErrMsg();
      workspaceApi.moveFileOrDir({ originPath, targetPath }).then((res) => {
          const sourceFileNode = fileManager.getSourceFileNode();
          const loadNodeList = fileManager.getLoadNodeList();
          const fileList = loadNodeList.filter( it => it != sourceFileNode.key);
          fileManager.updateLoadNodeList(fileList);
          const menuFileActionType = fileManager.getFileOperationType();
          if ( menuFileActionType == 'CUT') {
            fileManager.updateHistoryOpenFile(originPath, targetPath);
            this.props.onRename(originPath, targetPath, sourceFileNode.name, true);
          }
          this.loadFileList();
          this.setState({cutOrCopyKey: "",pasteType: "" });
          message.success("粘贴成功");
        }).catch(error => {
          if (error.code === 41001001) {
            Modal.confirm({
              title: '有相关联的kernel正在运行 继续操作将会关闭它/它们 是否继续操作?',
              onOk: () => {
                return workspaceApi.moveFileOrDir({ originPath, targetPath, autoClose: true }).then((res) => {
                  this.loadFileList();
                  this.setState({ cutOrCopyKey: "",pasteType: ""});
                  message.success("粘贴成功");
                })
              }
            })
          } else {
            message.error('粘贴失败')
          }
        })
    }

    if (this.state.pasteType === "copy") {
      workspaceApi.copyFileOrDir({ originPath, targetPath }).then((res) => {
        const file = fileManager.getSourceFileNode();
        if ( file && file.fileType === 'DIRECTORY') {
          fileManager.pushLoadNodeList(targetPath);
        }
        this.loadFileList();
        this.setState({ cutOrCopyKey: "", pasteType: "" });
        message.success(intl.get("PASTED_SUCCESSFULLY"));
      })
    }

  }

  handleSaveAs = (event, props, outputType) => {
    if (props.info.isLeaf) {
      event.preventDefault()
      const selectedKey = props.info.key
      workspaceApi.convertFile({ path: encodeURIComponent(selectedKey), outputType })
        .then(() => {
          message.success('保存成功')
          this.loadFileList()
        })
        .catch((err) => {
          console.log(err)
        })
    } else {
      console.log("需要选择文件")
      message.warning(intl.get("SELECT_FILE"))
    }
  }


  /*拖拽框的控制逻辑*/
  handleClickShrink = (props) => {
    const { notebookTabRef } = globalData.appComponentData
    this.setState({
      siderShow: true,
    })
    if (this.state.siderWidth !== 1) {
      this.setState({ siderWidth: 1 })
      IdpTerminal.setLeftFileManageWidth(0);
    } else {
      this.setState({ siderWidth: 300 })
      IdpTerminal.setLeftFileManageWidth(-300);
    }
    notebookTabRef.current && notebookTabRef.current.fun()
  }
  hanClickDown = () => {
    const { notebookTabRef } = globalData.appComponentData
    if (this.state.siderWidth === 1) {
      return
    } 300
    this.setState({
      siderShow: false,
      downOrup: 1,
    })
    let that = this
    document.onmousemove = function (e) {
      let { clientX } = e
      if (clientX < 349) {
        clientX = 349
      }
      if (clientX < 0) {
        clientX = 0
      }
      if (clientX > document.body.clientWidth - 200) {
        clientX = document.body.clientWidth - 200
      }
      console.log()
      clientX = clientX - 49
      that.setState({ siderWidth: clientX })
      IdpTerminal.setLeftFileManageWidth(clientX);
    }
    document.onmouseup = () => {
      if (this.state.downOrup === 1) {
        notebookTabRef.current && notebookTabRef.current.fun()
        document.onmousemove = null
        this.setState({
          downOrup: 0,
        })
      }
    }
  }

  updateUnzpiDisabled = (unzpiDisabled, node) => {
    this.setState({ unzpiDisabled, checkedNode: node });
  }

  unZipFolder = async (e) => {
    const { key } = e.props.info;
    const unzpiDisabled = this.state.unzpiDisabled;
    const checkedNode = this.state.checkedNode;
    // 只对压缩文件处理，其他不做任何处理
    if (!unzpiDisabled && checkedNode.fileType == 'FILE' && !checkedNode.expanded) {
      const node = checkedNode.key.replace(/\s*/g, "");
      const nodeFileName = checkedNode.name.replace(/\s*/g, "");
      const nodes = node.split(nodeFileName).filter(it => it != '').join('');
      const nodePath = nodes.substring(0, nodes.length - 1);
      const nodeIds = key.replace(/\s*/g, "").replace(new RegExp("/", "g"), '_');
      const nodeClass = document.getElementById(nodeIds);
      nodeClass.style.opacity = 1;
      try {
        const reuslt = await warenhouseApi.decompressFile(key, nodePath.length === 0 ? '/' : nodePath);
        if (reuslt.code == '21000000') {
          nodeClass.style.opacity = 0;
          this.onLoadData({ key: nodePath.length === 0 ? '/' : nodePath });
        } else {
          nodeClass.style.opacity = 0;
          message.error(`${intl.get('FILE_ZIP_DECOMPRESS_ERROR')}: ${reuslt.message}`)
        }
      } catch (error) {
        nodeClass.style.opacity = 0;
      }
    }
  }

  getContextMenu = () => {
    return {
      menuId: "fileTree",
      items: [
        { key: "ADD_FOLDER", name: intl.get("ADD_FOLDER"), handler: this.addNewFolder },
        { key: "ADD_FILE", name: intl.get("ADD_FILE"), handler: this.addNewFile },
        { key: "SEPARATOR_1" },
        { key: "CUT_FILE", name: intl.get('CUT'), handler: this.handlerCutFile },
        { key: "CUT_COPY_FILE", name: intl.get('COPY'), handler: this.handlerCopyFile },
        { key: "STICK", name: intl.get('PASTE'), handler: this.handlerStickFile },
        { key: "SEPARATOR_2" },
        { key: "RENAME", name: intl.get("RENAME"), handler: this.rename },
        {
          key: "SAVE_AS",
          name: intl.get("SAVE_AS"),
          children: [{
            key: "SAVE_AS_PY",
            name: ".py",
            handler: ({ event, props }) => this.handleSaveAs(event, props, 'python')
          }]
        },
        { key: "DELETE", name: intl.get("DELETE"), handler: this.delete },
        {
          key: "EXPORT",
          name: intl.get("EXPORT"),
          handler: this.onExportClick,
          children: [
            {
              key: "EXPORT_IDPNB",
              name: ".idpnb",
              handler: this.onExportClick,
            },
            {
              key: "EXPORT_IPYNB",
              name: ".ipynb",
              handler: this.onExportClick,
            },
            // {
            //   key: "EXPORT_HTML",
            //   name: this.state.exportFileLoading ? <Spin indicator={loadingIcon} /> : "HTML",
            //   handler: this.exportTypeClick("html"),
            // },
            /*            {
              key: "EXPORT_PDF",
              name: "PDF",
              handler: this.exportTypeClick("pdf"),
            },*/
            {
              key: "EXPORT_PYTHON",
              name: ".py",
              handler: this.exportTypeClick("python"),
            },
          ],
        },
        {
          key: "DOWNLOAD_FOLDER",
          name: intl.get("EXPORTFOLDER"),
          handler: this.exportFolderClick,
        },
        { key: "SEPARATOR_3" },
        {
          key: "COPY_RELATIVE_PATH",
          name: intl.get("COPY_RELATIVE_PATH"),
        },
        {
          key: "COPY_ABSOLUTE_PATH",
          name: intl.get("COPY_ABSOLUTE_PATH"),
        },
        // { key: "SEPARATOR_4" },
        // {
        //   key: "TENSORBOARD",
        //   name: intl.get("TENSORBOARD"),
        //   handler: ({ event, props }) => {
        //     const tensor = props.info.isLeaf ? props.info.key.slice(0, props.info.key.lastIndexOf('/')) : props.info.key
        //     this.props.history.push(`/tensorboard?projectId=${projectId}&tensor=${tensor}`)
        //   },
        // },
        { key: "SEPARATOR_5" },
        { key: "UNZIP", name: intl.get('FILE_ZIP_DECOMPRESS'), handler: this.unZipFolder },
        {
          key: "COMPRESSED_TO_ZIP",
          name: intl.get('FILE_ZIP_COMPRESS'),
          handler: (e, prop) => {
            const { key } = e.props.info;
            warenhouseApi.compression({ path: key })
              .then(res => {
                const { code, message: msg } = res.data
                if (code > 20000000 && code < 30000000) {
                  message.success(intl.get('FILE_ZIP_COMPRESS_SUCCEEDED'))
                  this.loadTree({ forceLoad: true })
                }else{
                  if(code === 51040000){
                    message.warning(intl.get('FILE_ZIP_COMPRESS_INFO_1'))
                  }else{
                    message.error(
                      `${
                        process.env.NODE_ENV === "development" ? "后端接口" + code + "异常" : ""
                      } ${msg ? "message:" + msg : ""}`,
                      1.5
                    )
                  }
                }
              })
          }
        },
        // { key: "SEPARATOR_6" },
        // {
        //   key: "CREATE_MODEL",
        //   name: '创建并发布模型',
        //   handler: async (e) => {
        //     // console.log(e, "--------e-------")
        //     const { key, name, isLeaf } = e.props.info;
        //     if(isLeaf && name.substring(name.length-3) !== "zip"){
        //       message.warning("只有zip压缩包可以创建并发布模型")
        //       return
        //     }
        //     // this.setState({
        //     //   cover: 'block'
        //     // })
        //     let packageName
        //     // 获取当前notebook环境
        //     let currentEnv = null;
        //     await environmentAPI.getEnvironmentName()
        //       .then(res => {
        //         const data = res.data
        //         currentEnv = data
        //       })
        //       .catch(err => {
        //         console.log(err)
        //       })

        //     const modelFileName = isLeaf ? key : `${key}/`
        //     const modelEnv = isLeaf ? null : `IDP:${currentEnv}`
        //     if(isLeaf){
        //       packageName = await createOrAddAModelOrVersionApi.getSuccessString({path:key}).then(res => res.data.packageName)
        //     }

        //     let category = null
        //     await createOrAddAModelOrVersionApi.getCategory({})
        //       .then(res => {
        //         const { data } = res;
        //         category = data
        //       })

        //     this.setState({
        //       modelFile: {
        //         string: packageName? packageName : modelFileName,
        //         name: modelFileName,
        //         category: category
        //       },
        //       modelEnv: modelEnv
        //     })

        //     this.changeCreateModelVisible(true)
        //   }
        // }
      ],
    }
  }

  changeCreateModelVisible = (bool) => this.setState({createDrawerVisible: bool});



  render() {
    const contextMenu = this.getContextMenu()
    const { isShow } = this.props
    return (
      <Sider
        style={{ display: isShow ? 'block' : 'none' }}
        key="workspace"
        theme="light"
        width={this.state.siderWidth}
        className={this.state.siderShow ? "show-sider" : ""}
      >
        <div
          style={{
            height: document.body.clientHeight - 60,
            flexFlow: 'column'
          }}
        >
          <WorkspaceLeftTitle
            addFolder={this.addFolder}
            addFile={this.addFile}
            handleIsFileTree={this.handleIsFileTree}
            handleFileChange={this.handleFileChange}
            loadTree={this.loadTree}
          />

          <Spin spinning={this.state.spinning}>
            <FileTree
              fileTreeDragAble={this.state.fileTreeDragAble}
              cutOrCopyKey={this.state.cutOrCopyKey}
              onLoadData={this.onLoadData}
              dropFileTree={this.dropFileTree}
              ref={this.fileTreeRef}
              selectedKeys={[this.state.selectedKey]}
              setRoot={this.setRoot}
              parentExpandedkey={this.state.expandedKeys}
              onSelect={this.onSelect}
              treeData={this.state.treeData}
              dataSourceTreeData={this.state.dataSourceTreeData}
              contextMenu={contextMenu}
              clickNotebookState={this.clickNotebookState}
              handleDeleteNodeKey={this.handleDeleteNodeKey} // 删除快捷键
              getDataBaseListInfoMETHODS={this.getDataBaseListInfo}
              getDataCloudListInfoMETHODS={this.loadFileList}
              fileInfo={this.state.fileInfo}
              unzpiDisabled={this.state.unzpiDisabled}
              handlerCutFileKey={this.handlerCutFileKey}
              handlerCopyFileKey={this.handlerCopyFileKey}
              handlerStickFileKey={this.handlerStickFileKey}
              updateUnzpiDisabled={this.updateUnzpiDisabled}
            >
              <FileTreeChildren
                uploadFileList={this.state.uploadFileList}
              />
            </FileTree>
          </Spin>

          <CollapseWorkspaceLeft
            hanClickDown={this.hanClickDown}
            handleClickShrink={this.handleClickShrink}
            siderWidth={this.state.siderWidth}
          />

          <AddFolderModal
            addFolder={this.state.visible.addFolder}
            submitAddFolder={this.submitAddFolder}
            confirmLoading={this.state.visible.confirmLoading}
            resetVisible={this.resetVisible}
            setInputValue={this.setInputValue}
            displayWarning={this.state.visible.displayWarning}
            fileNameValidator={this.state.visible.fileNameValidator}
            folderValue={this.state.visible.folderValue}
            folderInputDisabled={this.state.visible.folderInputDisabled}
          />

          <AddFileModal
            addFile={this.state.visible.addFile}
            submitAddFile={this.submitAddFile}
            confirmLoading={this.state.visible.confirmLoading}
            resetVisible={this.resetVisible}
            setFileValue={this.setFileValue}
            fileInputDisabled={this.state.visible.fileInputDisabled}
            fileValue={this.state.visible.fileValue}
            displayWarning={this.state.visible.displayWarning}
            fileNameValidator={this.state.visible.fileNameValidator}
          />

          <DeleteModal
            deleteVisible={this.state.visible.delete}
            submitDelete={this.submitDelete}
            confirmLoading={this.state.visible.confirmLoading}
            resetVisible={this.resetVisible}
            selectedName={this.state.selectedName}
          />

          <RenameModal
            rename={this.state.visible.rename}
            submitRename={this.submitRename}
            confirmLoading={this.state.visible.confirmLoading}
            resetVisible={this.resetVisible}
            renameValue={this.state.visible.renameValue}
            setInputValue={this.setInputValue}
            renameInputDisabled={this.state.visible.renameInputDisabled}
            displayWarning={this.state.visible.displayWarning}
            fileNameValidator={this.state.visible.fileNameValidator}
          />

          {/* 创建模型 发布模型 */}
          <CreateModel
            clicksViewFlg={false}
            loadPage={1}
            createDrawerVisible={this.state.createDrawerVisible}
            changeCreateDrawerVisible={() => {
              this.setState({
                createDrawerVisible: false,
                modelFile: {string: "", name: ""}
              })
            }}
            createOrAdd={0}   // 创建模型还是添加新版本 0创建 1新增
            category={this.state.category}
            setCreateDrawerVisible={this.changeCreateModelVisible}
            packageId={"000"} // 新增版本 ID 目前传递任意 string 即可
            PreFillIn={undefined}
            noteBookModelFile={this.state.modelFile}
            noteBookModelEnv={this.state.modelEnv}
          />

          <div className="cover" style={{
            display: this.state.cover
          }}>
            <Spin size="large" />
          </div>
        </div>
      </Sider>
    )
  }
}

function WorkspaceLeftWithContext(props) {
  const {
    onWsSelected,
    onWsDelete,
    onRename,
  } = useContext(appContext)


  return (
    <WorkspaceLeft {...props}
      onWsSelected={onWsSelected}
      onWsDelete={onWsDelete}
      onRename={onRename} />
  )

}

export default connect(
  (state) => {
    return {
      fileList: state.filesTab.fileList,
      activePath: state.filesTab.activePath
    }
  },
  {
    updateNotebookListFromTabListAsync,
    clearTabsFromList,
  }
)(withRouter(WorkspaceLeftWithContext))
