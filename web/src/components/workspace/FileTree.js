import React, {
  useEffect,
  useImperativeHandle,
  useMemo,
  useState,
} from "react"
import intl from "react-intl-universal"
import {
  Button,
  Input,
  message,
  Tooltip,
  Tree,
} from "antd"
import {
  Item,
  Menu,
  Separator,
  theme,
  useContextMenu,
  Submenu,
} from "react-contexify"

import "react-contexify/dist/ReactContexify.min.css"
import dataSetApi from '../../services/dataSetApi'
import { CopyToClipboard } from "react-copy-to-clipboard"

import {
  FolderOpenOutlined,
  SearchOutlined,
} from "@ant-design/icons"
import MySqlIcon from "../../assets/logo/sql.svg"
import PostgreSqllIcon from "./svgicons/postgerSQL.svg"
import SparkIcon from './svgicons/spark.svg'
import hiveIcon from './svgicons/hive.svg'


import "./WorkspaceLeft.less"
import { useDebounceEffect, useDebounceFn, useMemoizedFn } from "ahooks"
import classNames from "classnames"
import { findFileOrDirName, mergeArray } from "../../utils"
import { addDeleteNodeKey, removeDeleteNodeKey } from "./keymap"
import Icons from "../Icons/Icons"
import ReconnectionDataBase from './ReconnectionDataBase'
import ReconnectionCloudDataBase from "./ReconnectionCloudDataBase"
import FileTreeCollapse from "./components/FileTreeCollapse"
import FileTreeList from "./components/FileTreeList"
import useShowFooter from "../../utils/hook/useShowFooter"
import { region, userDir, userId } from '../../store/cookie'
import { useClipboard } from 'use-clipboard-copy'
import terminalApi from '../../services/terminalApi'
import userExtensionConfig from '../../../config/user-extension-config';
import terminal from "@/idp/lib/terminal"
import { observer } from "mobx-react"
import fileManager from "@/idp/global/fileManager";

const { DirectoryTree } = Tree


function createExpandedKeyArr(string) {
  const arr = []
  const stringArr = string.split("/")
  stringArr.pop()
  while (stringArr.length > 0) {
    const str = stringArr.join("/")
    if (str) {
      arr.push(str)
    }
    stringArr.pop()
  }
  return arr
}

export const objectStorageType = (name) => {
  let storageType;
  switch (name) {
    case 'aliyun s3':
      storageType = '阿里云 OSS'
      break
    case 'amazon s3':
      storageType = 'Amazon S3'
      break
    case 'ucloud s3':
      storageType = 'Ucloud S3'
      break
    case 'ucloud nfs':
      storageType = 'Ucloud NFS'
      break
    default:
      storageType = ''
      break
  }
  return storageType
}

const cloudNameMethods = (title) => {
  let cloudName;
  switch (title) {
    case 'ucloud nfs':
      cloudName = 'ucloud'
      break
    case 'aliyun nas':
      cloudName = 'aliyun'
      break
    default:
      cloudName = ''
  }
  return cloudName
}
export const cloudChild = (key) => {
  let cloundChild = key.slice(1, key.indexOf("/", key.indexOf("/") + 1)),
    num = key.split("/").length - 1;
  return cloundChild === "storage-service" && num === 2;
}


const FileTree = (props, ref) => {
  const {
    treeData,
    contextMenu,
    selectedKeys,
    onSelect,
    setRoot,
    parentExpandedkey,
    dataSourceTreeData,
    dropFileTree,
    handleDeleteNodeKey,
    onLoadData,
    cutOrCopyKey,
    getDataCloudListInfoMETHODS,
    fileTreeDragAble,
    clickNotebookState,
    handlerCutFileKey,
    handlerCopyFileKey,
    handlerStickFileKey,
    updateUnzpiDisabled,
    unzpiDisabled
  } = props

  const isShowFooter = useShowFooter()
  const [expandedKeys, setExpandedKeys] = useState([])
  const [searchValue, setSearchValue] = useState("")
  const [autoExpandParent, setAutoExpandParent] = useState(false)
  const [preExpandedKeys, setPreExpandedKeys] = useState([]) //为根目录折叠展开缓存数据

  const [menuState, setMenuState] = useState(
    /CUT_COPY_FILE|CUT_FILE|STICK|ADD_FOLDER|ADD_FILE|RENAME|SAVE_AS|SAVE_AS_PY|EXPORT|DOWNLOAD|DELETE|COPY_RELATIVE_PATH|COPY_ABSOLUTE_PATH|EXPORT_IDPNB|EXPORT_IPYNB|EXPORT_HTML|EXPORT_PDF|EXPORT_PYTHON|TENSORBOARD|COMPRESSED_TO_ZIP|UNZIP|CREATE_MODEL/
  )
  const [filePath, setFilePath] = useState("")
  const [relativePathData, setRelativePathData] = useState("")


  // 重连框显示隐藏
  const [reconnectModalVisible, setReconnectModalVisible] = useState(false)
  const [reconnectCloudModalVisible, setReconnectCloudModalVisible] = useState(false)
  const [dataBaseItem, setDataBaseItem] = useState({})
  const [cloudDataBaseItem, setCloudDataBaseItem] = useState({})
  const clipboard = useClipboard()



  const { run: runFindExpandedParentKey } = useDebounceFn(
    () => {
      findExpandedParentKey()
    },
    {
      wait: 1000,
    }
  )

  const findExpandedParentKey = () => {
    if (selectedKeys[0]) {
      const arr = createExpandedKeyArr(selectedKeys[0])
      setExpandedKeys((expandedKeys) => {
        const newExpandedKeys = mergeArray(expandedKeys, arr)
        //  是因为expandedKeys更新了 导致onLoadData函数自动执行了
        return newExpandedKeys
      })
    }
  }

  useEffect(() => {
    runFindExpandedParentKey()
  }, [selectedKeys])

  /* 数据源树相关的内容 start*/
  const [expandedDataSourceTreeKey, setExpandedDataSourceTreeKey] = useState([])

  const handleExpandedDataSource = (expandedKeys) => {
    setExpandedDataSourceTreeKey(expandedKeys)
  }

  /*end 数据源树相关的内容*/

  /*目录树相关逻辑 start*/
  const ContextMenu = useMemoizedFn((a, b) => {
    return (
      <Menu id={contextMenu.menuId} theme={theme.light}>
        {contextMenu.items.map((item) => {
          if (item.key.startsWith("SEPARATOR")) {
            if (filePath === "/") {
              return null
            } else if (
              cloudChild(filePath) &&
              item.key.startsWith("SEPARATOR_0")
            ) {
              return null
            } else {
              return <Separator key={item.key} />
            }
          }

          if (item.key.startsWith("COPY_")) {
            return item.key.startsWith("COPY_RELATIVE_PATH") ? (
              <Item
                key={item.key}
                style={{ display: menuState.test(item.key) ? "" : "none" }}
                onClick={() => {
                  clipboard.copy(relativePathData)
                }}
              >
                <span>{item.name}</span>
              </Item>
            ) : (
              <Item
                key={item.key}
                style={{ display: menuState.test(item.key) ? "" : "none" }}
                onClick={() => {
                  clipboard.copy(filePath)
                }}
              >
                <span>{item.name}</span>
              </Item>
            )
          }
          if (item.key.startsWith('SAVE_AS') && menuState.test(item.key)) {
            return <Submenu
              key={item.key}
              label={intl.get('SAVE_AS')}
              arrow={<i className={"ant-menu-submenu-arrow"} />}
            >
              {item.children.map((childrenItem) => {
                return (
                  <Item
                    key={childrenItem.key}
                    onClick={childrenItem.handler}
                  >
                    {childrenItem.name}
                  </Item>
                )
              })}
            </Submenu>
          }
          if (item.key.startsWith("EXPORT") && menuState.test(item.key)) {
            const fileName = findFileOrDirName(filePath)
            return (fileName.includes(".ipynb") || fileName.includes(".idpnb")) ? (
              <Submenu
                key={item.key}
                label={intl.get('EXPORT')}
                arrow={<i className={"ant-menu-submenu-arrow"} />}
              >
                {item.children.map((childrenItem) => {
                  return (
                    <Item
                      key={childrenItem.key}
                      onClick={childrenItem.handler}
                      style={{
                        display: menuState.test(childrenItem.key)
                          && ((childrenItem.name !== '.ipynb' && childrenItem.name !== '.idpnb') || fileName.endsWith(childrenItem.name)) ? "" : "none",
                      }}
                    >
                      {childrenItem.name}
                    </Item>
                  )
                })}
              </Submenu>
            ) : (
              <Item
                key={item.key}
                onClick={item.handler}
                style={{
                  display: menuState.test(item.key) ? "" : "none",
                }}
              >
                {item.name}
              </Item>
            )
          }
          if (item.key.startsWith("DOWNLOAD") && menuState.test(item.key)) {
            return (
              // <div>这里是导出文件夹</div>
              <Item
                key={item.key}
                onClick={item.handler}
                style={{ display: menuState.test(item.key) ? "" : "none" }}
              >
                {item.name}
              </Item>
            )
          }
          if (item.key.startsWith("STICK") && menuState.test(item.key)) {
            return (
              <Item
                key={item.key}
                onClick={item.handler}
                style={{ display: menuState.test(item.key) ? "" : "none" }}
                disabled={!cutOrCopyKey}
              >
                {item.name}
              </Item>
            )
          }
          if (item.key.startsWith("UNZIP") && menuState.test(item.key)) {
            return (
              <Item
                key={item.key}
                onClick={item.handler}
                disabled={unzpiDisabled}
              >
                {item.name}
              </Item>
            )
          }
          return (
            <Item
              key={item.key}
              onClick={item.handler}
              style={{ display: menuState.test(item.key) ? "" : "none" }}
            >
              {item.name}
            </Item>
          )
        })}
      </Menu>
    )
  })

  const { show } = useContextMenu({
    id: contextMenu.menuId,
  })

  //relativePath 相对路径
  const relativePath = (relativeP, absoluteP) => {
    relativeP = "/root" + relativeP
    absoluteP = "/root" + absoluteP
    let rela = relativeP.split("/")
    rela.shift()
    let abso = absoluteP.split("/")
    abso.shift()
    let num = 0
    for (let i = 0; i < rela.length; i++) {
      if (rela[i] === abso[i]) {
        num++
      } else {
        break
      }
    }
    rela.splice(0, num)
    abso.splice(0, num)
    let str = ""
    for (let j = 0; j < abso.length - 1; j++) {
      str += "../"
    }
    if (!str) {
      str += "./"
    }
    str += rela.join("/")
    setRelativePathData(str)
  }

  const checkFileZipType = (node) => {
    const fileSuffixTypes = ['zip', 'gzip', 'tar.gz', 'tgz'];
    const fileSuffixName = node.name.split('.').pop();
    let _unzpiDisabled = true;
    if (node.fileType === 'FILE') {
      if (fileSuffixName === 'gz') {
        let gzFiles = node.name.split('.');
        if (gzFiles.length > 2) {
          const zgSuffix = gzFiles[gzFiles.length - 2];
          if (zgSuffix === 'tar') {
            _unzpiDisabled = false;
          }
        }
      } else {
        if (fileSuffixTypes.includes(fileSuffixName)) {
          _unzpiDisabled = false;
        }
      }
    }
    updateUnzpiDisabled(_unzpiDisabled, node);
  }

  const handleContextMenu = (event, node) => {
    event.preventDefault();
    // 如果fileType为数据源相关的类型 则不进行后面的逻辑
    if (node.fileType === "database" || node.fileType === "database-table") {
      return
    }
    if (node.fileType === "DIRECTORY" && node.name === "storage-service") {
      return
    }
    node.key !== '/' && checkFileZipType(node);
    setFilePath(node.key)
    relativePath(node.key, selectedKeys[0])
    if (cloudChild(node.key)) {
      setMenuState(
        /ADD_FOLDER|ADD_FILE|DOWNLOAD|COPY_RELATIVE_PATH|COPY_ABSOLUTE_PATH/
      )
    } else {
      if (node.isRoot) {
        setMenuState(/ADD_FOLDER|ADD_FILE/)
        // DOWNLOAD_FOLDER
      } else {
        if (node.isLeaf) {
          if (node.key.endsWith('.idpnb') || node.key.endsWith('.ipynb')) {
            setMenuState(
              /CUT_FILE|CUT_COPY_FILE|STICK|ADD_FOLDER|ADD_FILE|RENAME|SAVE_AS|EXPORT|DELETE|COPY_RELATIVE_PATH|COPY_ABSOLUTE_PATH|TENSORBOARD|COMPRESSED_TO_ZIP|CREATE_MODEL/
            )
          } else {
            setMenuState(
              /CUT_FILE|CUT_COPY_FILE|STICK|ADD_FOLDER|ADD_FILE|RENAME|EXPORT|DELETE|COPY_RELATIVE_PATH|COPY_ABSOLUTE_PATH|TENSORBOARD|COMPRESSED_TO_ZIP|UNZIP|CREATE_MODEL/
            )
          }
        } else {
          setMenuState(
            /CUT_FILE|CUT_COPY_FILE|STICK|ADD_FOLDER|ADD_FILE|RENAME|DOWNLOAD|DELETE|COPY_RELATIVE_PATH|COPY_ABSOLUTE_PATH|TENSORBOARD|COMPRESSED_TO_ZIP|UNZIP|CREATE_MODEL/
          )
        }
      }
    }
    //延迟到下次循环执行
    setTimeout(function () {
      show(event, {
        props: {
          info: node,
        },
      })
    }, 0)
  }

  const handleRightClick = (e) => {
    const node = {
      key: "/",
      isRoot: true,
    }
    handleContextMenu(e, node)
  }
  //在目录节点右键新增文件/目录时，展开目录，parentExpandedkeys由父组件传递
  useEffect(() => {
    setExpandedKeys(expandedKeys.concat(parentExpandedkey))
  }, [parentExpandedkey])

  const generateList = (data, dataList) => {
    for (let i = 0; i < data.length; i++) {
      const node = data[i]
      const { key, title } = node
      dataList.push({ key, title })
      if (node.children) {
        generateList(node.children, dataList)
      }
    }
  }

  const dataList = useMemo(() => {
    const arr = []
    generateList(treeData, arr)
    return arr
  }, [treeData])



  const getParentKey = (key, tree) => {
    let parentKey
    for (let i = 0; i < tree.length; i++) {
      const node = tree[i]
      if (node.children) {
        if (node.children.some((item) => item.key === key)) {
          parentKey = node.key
        } else {
          const result = getParentKey(key, node.children)
          if (result) {
            parentKey = result
          }
        }
      }
    }
    return parentKey
  }

  const onExpand = (expandedKeys, { expanded: bool, node }) => {
    if (!bool) {
      expandedKeys = expandedKeys.filter(item => item !== node.key)
    }
    setExpandedKeys(expandedKeys)
    setAutoExpandParent(false)
    setPreExpandedKeys([])
  }

  const onRootClick = () => {
    setRoot()
    expandedKeys.length && setPreExpandedKeys(expandedKeys)
    expandedKeys.length ? setExpandedKeys([]) : setExpandedKeys(preExpandedKeys)
  }

  const [overkeys, setOverKey] = useState(''); // 设置移入的值

  // 树节点拖拽时出发
  const onDragStart = () => {
    setOverKey('');
  };

  const onDragOver = () => {
    setOverKey('');
  }

  const onVisibleChange = (key, visible) => {
    visible ? setOverKey('') : setOverKey(key);
  }

  const loop = (data) => {
    return data.map((item) => {
      const isObject = typeof item.title == "object"
      const index = isObject ? -1 : item.title?.indexOf(searchValue)
      const beforeStr = isObject ? null : item.title?.substr(0, index)
      const afterStr = isObject
        ? null
        : item.title?.substr(index + searchValue.length)
      const showTitle = item.title

      let title =
        searchValue !== "" && index > -1 ? (
          <span id={item.key.replace(/\s*/g, "").replace(new RegExp("/", "g"), '_')} className={classNames("filename" + item.key, item.fileType)}>
            {beforeStr}
            <span className="file-tree-search-value">{searchValue}</span>
            {afterStr}
          </span>
        ) : isObject ? (
          <span id={item.key.replace(/\s*/g, "").replace(new RegExp("/", "g"), '_')} className={classNames("filename" + item.key, item.fileType)}>
            {showTitle}
          </span>
        ) : (
          <div>
            <Tooltip title={item.key} mouseEnterDelay={1.5} visible={overkeys === item.key} onVisibleChange={() => onVisibleChange(item.key, overkeys == item.key)} placement="topLeft" >
              <span className={"title-container"}>
                <span style={{ color: '#2C2F33'}} id={item.key.replace(/\s*/g, "").replace(new RegExp("/", "g"), '_')} className={classNames("filename" + item.key, item.fileType)}>
                  {showTitle}
                </span>
                {
                  (cloudChild(item.key) && !item.active) ?
                    (
                      <span className={"data-source-type-title"}>
                        <span className="data-source-name">&nbsp;{objectStorageType(item.sourceType)}</span>
                        <Tooltip placement="top" title={intl.get("RECONNECT")}>
                          <Icons.BHDisconnectDatabase
                            onClick={(e) => {
                              e.stopPropagation()
                              reconnectionCloudDataBase(item)
                            }}
                          />
                        </Tooltip>
                      </span>) : (
                      <span className={"data-source-type-title"}>
                        <span className="data-source-name" style={{ paddingRight: '20px' }}>&nbsp;{objectStorageType(item.sourceType)}</span>
                      </span>
                    )
                }
              </span>
            </Tooltip>
          </div>

        )
      let icon = null
      if (item.fileType === "database") {
        let dataSourceTypeTitle = ""
        switch (item.dataSourceType) {
          case "mysql":
            dataSourceTypeTitle = "MySQL"
            icon = (
              <img
                style={{ width: 15, height: 15 }}
                src={MySqlIcon}
                alt={"数据源图标"}
              />
            )
            break
          case "mysql6":
            dataSourceTypeTitle = "MySQL"
            icon = (
              <img
                style={{ width: 15, height: 15 }}
                src={MySqlIcon}
                alt={"数据源图标"}
              />
            )
            break
          case "postgresql":
            dataSourceTypeTitle = "PostgreSQL"
            icon = (
              <img
                style={{ width: 15, height: 15 }}
                src={PostgreSqllIcon}
                alt={"数据源图标"}
              />
            )
            break
          case "hive2":
            dataSourceTypeTitle = "Hive"
            icon = (
              <img
                style={{ width: 15, height: 15 }}
                src={hiveIcon}
                alt={"数据源图标"}
              />
            )
            break
          case 'sparksql':
            dataSourceTypeTitle = "Spark"
            icon = (
              <img
                style={{ width: 15, height: 15 }}
                src={SparkIcon}
                alt={"数据源图标"}
              />
            )
            break
        }

        title = (
          <div className={"title-container"}>
            <span>{title}</span>
            <span className={"data-source-type-title"}>
              <span className="data-source-name">{dataSourceTypeTitle}</span>
              {item.status === '0' ?
                <span className="data-source-name" style={{ paddingRight: '20px' }}></span> :
                (<Tooltip placement="top" title={intl.get('RECONNECT')}>
                  <Icons.BHDisconnectDatabase
                    onClick={(event) => {
                      event.stopPropagation()
                      reconnectionDataBase(item)
                    }} />
                </Tooltip>)}
            </span>
          </div>
        )
      }
      // fileSystem icon逻辑， 勿删
      // if(item.sourceType){
      //   switch(item.sourceType){
      //     case 'aliyun s3':
      //       icon = (
      //         <img
      //           src={aliyunImg}
      //           style={{ width: 15, height: 15 }}
      //           alt="阿里云OSS"/>
      //       )
      //       break
      //     case 'amazon s3':
      //       icon = (
      //         <img
      //           src={amazonImg}
      //           style={{ width: 15, height: 15 }}
      //           alt="Amazon s3"/>
      //       )
      //       break
      //     case 'ucloud s3':
      //       icon = (
      //         <img
      //           src={ucloudImg}
      //           style={{ width: 15, height: 15 }}
      //           alt="Ucloud S3"/>
      //       )
      //       break
      //     default:
      //       break
      //   }
      // }
      if (item.children) {
        return {
          title,
          key: item.key,
          name: item.title,
          isLeaf: item.isLeaf,
          fileType: item.fileType,
          tableName: item.tableName,
          parentKey: item.parentKey,
          children: loop(item.children),
          status: item.fileType === 'database' ? item.status : "",
          // S3 图标逻辑 勿删
          // icon: item.fileType === 'DIRECTORY' && item.key === '/storage-service'?
          // (<Icons.BHStorageServiceIcon/>) : icon
          icon
        }
      }

      return {
        title,
        key: item.key,
        name: item.title,
        isLeaf: item.isLeaf,
        fileType: item.fileType,
        tableName: item.tableName,
        parentKey: item.parentKey,
      }
    })
  }

  /*弹出重连modal框*/
  const reconnectionDataBase = (item) => {
    setDataBaseItem(item)
    setReconnectModalVisible(true)
  }
  /*隐藏重连modal框*/
  const showReconnectionModal = () => {
    setReconnectModalVisible(false)
  }



  /* 弹出重连Cloud框 */
  const reconnectionCloudDataBase = (item) => {
    if (item.sourceType === 'ucloud nfs' || item.sourceType === 'aliyun nas') {
      const data = {
        cloudName: cloudNameMethods(item.sourceType),
        mountPath: item.title,
        endPoint: item.endPoint,
        alias: item.title,
        fsType: item.sourceType,
        db: item.sourceType,
        aliasDB: item.title
      }
      dataSetApi.nfsMount(data)
        .then(res => {
          if (res.code === 200) {
            message.success(intl.get('RELINK_SUCCESSFULLY'))
            getDataCloudListInfoMETHODS()
          } else {
            message.error(res.message)
          }
        })
        .catch(err => {
          console.log(err)
        })
    } else {
      setCloudDataBaseItem(item)
      setReconnectCloudModalVisible(true)
    }
  }

  /* 弹出Cloud重连modal框 */
  const showReconnectionCloudModal = () => {
    setReconnectCloudModalVisible(false)
  }


  /*end 目录树相关逻辑*/
  useImperativeHandle(
    ref,
    () => ({

    }),
    []
  )

  /*input搜索框相关逻辑 start*/
  const onChange = () => {
    const value = searchValue
    if (value !== "") {
      const expandedKeys = dataList
        .map((item) => {
          if (item.title.indexOf(value) > -1) {
            return getParentKey(item.key, treeData)
          }
          return null
        })
        .filter((item, i, self) => item && self.indexOf(item) === i)
      let realExpandedKeys = []
      expandedKeys.forEach((expandedKey) => {
        const arr = createExpandedKeyArr(expandedKey)
        arr.push(expandedKey)
        realExpandedKeys = mergeArray(realExpandedKeys, arr)
      })
      setExpandedKeys(realExpandedKeys)
    } else {
      setExpandedKeys([])
    }
    setAutoExpandParent(false)
  }

  useDebounceEffect(
    () => {
      onChange()
    },
    [searchValue],
    { wait: 1000 }
  )
  /*end  input搜索框相关逻辑 */

  const [vscodeLoading, setVscodeLoading] = useState(false)
  const handleOpenVscode = () => {
    setVscodeLoading(true)
    terminalApi.openVscode().then((res) => {
      console.log(`start vscode in port ${res.data.port}`)
      setVscodeLoading(false)
      window.open(`/${region}/vscode/?folder=${userDir}`)
    }).catch((error) => {
      console.log(error)
      setVscodeLoading(false)
    })
  }
  const hasVscode = () => {
    for (const conf of userExtensionConfig) {
      if (conf.userId === userId && conf.extentions.indexOf('vscode') >= 0) {
        return true
      }
    }
    return false
  }

  return (
    <div className="sider-box">
      <div className="sider-box-warrper">
        <Input
          value={searchValue}
          placeholder={intl.get("SEARCH_FILE_OR_FOLDER")}
          onChange={(event) => {
            setSearchValue(event.target.value)
          }}
          prefix={<SearchOutlined />}
        />
      </div>

      <div
        style={{
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
          height: terminal.clientHeight - (isShowFooter ? 125 : 108),
          overflow: "hidden",
        }}
      >
        <div
          className={"file-tree-container"}
          style={{
            flexGrow: 2,
            minHeight: 315 - 150,
            overflow: "auto",
          }}
        >
          <div className={"data-source-container"}>
            <DirectoryTree
              onSelect={onSelect}
              treeData={loop(dataSourceTreeData)}
              onExpand={handleExpandedDataSource}
              expandedKeys={expandedDataSourceTreeKey}
              selectedKeys={selectedKeys}
            />
          </div>

          <Button
            icon={<FolderOpenOutlined />}
            size="small"
            type="text"
            onClick={onRootClick}
            style={{ width: "90%", textAlign: "left" }}
            onContextMenu={handleRightClick}
          >
            /
          </Button>

          {process.env.NODE_OPEN !== 'true' && hasVscode() ? <Tooltip placement="rightTop" title={'在vscode中打开项目'}>
            <Button
              icon={<Icons.BHVscodeIcon />}
              size="small"
              type="text"
              style={{ width: "10%" }}
              onClick={() => handleOpenVscode()}
              loading={vscodeLoading}
            >
            </Button>
          </Tooltip> : <></>}

          <div
            className="tree-box"
            tabIndex="1"
            onFocus={() => addDeleteNodeKey(() => handleDeleteNodeKey())}
            onBlur={() => removeDeleteNodeKey()}
          >
            <FileTreeList
              onLoadData={onLoadData}
              selectedKeys={selectedKeys}
              expandedKeys={expandedKeys}
              autoExpandParent={autoExpandParent}
              treeData={treeData}
              onExpand={onExpand}
              onSelect={onSelect}
              handleContextMenu={handleContextMenu}
              fileTreeDragAble={fileTreeDragAble}
              onDragStart={onDragStart}
              onDragOver={onDragOver}
              dropFileTree={dropFileTree}
              searchValue={searchValue}
              overkeys={overkeys}
              onVisibleChange={onVisibleChange}
              reconnectionCloudDataBase={reconnectionCloudDataBase}
              reconnectionDataBase={reconnectionDataBase}
              handlerCutFileKey={handlerCutFileKey}
              handlerCopyFileKey={handlerCopyFileKey}
              handlerStickFileKey={handlerStickFileKey}
            />
            <ContextMenu />
          </div>
          {props.children}
        </div>


        {/*kernel折叠面板*/}
        <FileTreeCollapse
          clickNotebookState={clickNotebookState}
        />
      </div>

      <ReconnectionDataBase
        visibleData={reconnectModalVisible}
        dataBaseItem={dataBaseItem}
        showReconnectionModal={showReconnectionModal}
        RefreshDataBase={props.getDataBaseListInfoMETHODS} />
      <ReconnectionCloudDataBase
        visibleData={reconnectCloudModalVisible}
        showReconnectionCloudModal={showReconnectionCloudModal}
        cloudItem={cloudDataBaseItem}
        RefreshDataBase={props.getDataCloudListInfoMETHODS}
      />
    </div>
  )
}

export default observer(React.forwardRef(FileTree))
