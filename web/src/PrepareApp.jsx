import React from "react"
import { withRouter } from "react-router"
import { connect } from "react-redux"
import cookie from "react-cookies"
import intl from "react-intl-universal"

import {Layout, Modal, notification} from "antd"
import { NotificationOutlined } from "@ant-design/icons"

import "./App.less"

import { Provider } from "./context"
import { projectId } from "./store/cookie"
import AppLoading from "./layout/appLoading"


import probeApi from "./services/probeApi"
import contentApi from "./services/contentApi"

import { findFileTreeParentKey, locationToProjectListPage } from "./utils"
import { GloablFocus, GloablBlur } from './components/workspace/keymap'

import PubSub from "pubsub-js"

import {
  getHistoryOpenFile,
  handlerSaveHistoryOpenFile,
  historyDelFile,
  initHistoryFile,
  saveHistoryOpenFile,
} from "./utils/storage"

import {
  contentCatAsync,
  updateNotebookListFromTabListAsync,
  updatePath,
  variableListAsync,
} from "./store/features/notebookSlice"

import {
  addNewFile,
  changeActivePath,
  clearTabsFromList,
  updateFileProp,
  addFileAndContentAsync,
} from "./store/features/filesTabSlice"

import {
  updateClientHeight,
  updateClientWidth,
} from "./store/features/configSlice"

import globalData from "@/idp/global"
import {observer} from "mobx-react"


const locales = {
  enUS: require("./locales/en-US.json"),
  zhCN: require("./locales/zh-CN.json"),
}

const defaultOpenFile = {
  path: "/玩转IDP/快速上手/helloworld.ipynb",
  name: "helloworld.ipynb",
  suffix: "ipynb",
}

const setDefaultLang = ()=>{
  let lang = cookie.load("locale")
  if (undefined === lang || "" === lang) {
    lang = "zhCN"
    cookie.save("locale", lang)
  }
}


// 大版本更新提示框
@observer
class App extends React.Component {
  constructor(props) {
    super(props)

    setDefaultLang()
    // 初始化historyFile
    initHistoryFile()

    this.state = {
      intlInit: false,
      theme: "dark-theme",
      isHealth: false,
      initTips: "正在为您配置计算资源...",
    }
    // 根据累加次数切换初始化信息
    this.healthCheckCount = 0
    this.appComponentData = globalData.appComponentData
  }

  componentDidMount() {
    this.loadLocales()
    this.checkHealth()


    window.onresize = () => {
      this.props.updateClientHeight(document.body.clientHeight)
      this.props.updateClientWidth(document.body.clientWidth)
    }
  }

  // 打开最近的文件
  openLocalFile = ()=>{
    const qs = new URLSearchParams(window.location.search)
    const shareId = qs.get("shareId")
    if (shareId && qs.get('projectId')) {
      // 打开分享链接中的文件
      this.setOpenShareFile(shareId)
    } else {
      // 打开最近一次打开的文件
      this.setOpenFile()
    }
  }

  // 打开分享链接中的文件
  setOpenShareFile = (shareId) => {
    contentApi
      .loadShared({ shareId })
      .then((res) => {
        const fileName = res.data
        const openFile = {
          path: fileName.slice(fileName.indexOf("/notebooks") + 10),
          name: fileName.slice(fileName.lastIndexOf("/") + 1),
          suffix: fileName.slice(fileName.lastIndexOf(".") + 1),
        }
        this.addFileAndHandleIpynb(openFile)
      })
      .catch((err) => {
        console.log(err)
      })
  }

  // 打开最近一次打开的文件
  setOpenFile = () => {
    const historyOpenFile = getHistoryOpenFile()
    if (
      (!historyOpenFile[projectId] ||
        historyOpenFile[projectId]?.length === 0) &&
      projectId === "1"
    ) {
      contentApi.cat({ path: defaultOpenFile.path }).then(() => {
        this.addFileAndHandleIpynb(defaultOpenFile)
        historyOpenFile[projectId] = [
          { name: defaultOpenFile.path, status: "open" },
        ]
        saveHistoryOpenFile(historyOpenFile)
      })
      return
    }

    const cacheFileObj = { ...historyOpenFile }
    let firstOpenFile = null
    if (
      Array.isArray(cacheFileObj[projectId]) &&
      cacheFileObj[projectId].length > 0
    ) {
      const projectList = cacheFileObj[projectId]
      for (let i = 0; i < projectList.length; i++) {
        const projectListElement = projectList[i]
        if (
          projectListElement &&
          projectListElement.name &&
          projectListElement.status === "open"
        ) {
          const fileName = projectListElement.name
          const name = fileName.slice(fileName.lastIndexOf("/") + 1)
          const suffix = fileName.slice(fileName.lastIndexOf(".") + 1)
          const openFile = {
            path: fileName,
            name,
            suffix,
          }
          if (i === 0) {
            firstOpenFile = openFile
          }
          this.addFileAndHandleIpynb(openFile,true)
        }
      }
      if (firstOpenFile) {
        const fileName = firstOpenFile.path
        this.props.changeActivePath(fileName)
      }
    }
  }



  addFileAndHandleIpynb = (openFile,notNeedChangePath) => {
    if(notNeedChangePath){
      openFile = {
        ...openFile,
        notNeedChangePath:true
      }
    }
    const suffix = openFile.suffix
    if (suffix === "ipynb" || suffix === "idpnb") {
      this.props.addNewFile(openFile)
      this.props.contentCatAsync(openFile).then((res) => {
        if (res.payload) {
          const { inode } = res.payload.response.content.metadata
          const path = res.payload.path
          this.props.variableListAsync({ path, inode })
        }else{
          this.appComponentData.notebookTabRef.current.removeTab(openFile.path)
          Modal.warning({
            title: `open ${openFile.path} fail`,
            content:res.error.message
          })
        }
      })
    }else {
      this.props.addFileAndContentAsync(openFile)
    }
  }

  wsSelected = (key, name) => {
    const openFile = {
      path: key,
      name,
      suffix: key.slice(key.lastIndexOf(".") + 1),
    }
    this.addFileAndHandleIpynb(openFile)
    handlerSaveHistoryOpenFile(key, name, "open")
    const { pathname, search } = this.props.location
    if (pathname !== "/workspace") {
      this.props.history.replace("/workspace" + search)
    }
  }
  wsRename = (oldkey, newKey, name, isLeaf) => {
    const tabList = this.props.tabList
    let isNeedUpdateKernel = false


    if (isLeaf) {
      if (tabList.some((item) => item.path === oldkey)) {
        const openFile = {
          path: newKey,
          name,
          suffix: newKey.slice(newKey.lastIndexOf(".") + 1),
        }
        this.appComponentData.notebookTabRef.current &&
        this.appComponentData.notebookTabRef.current.updateDeleteFlag(oldkey).then(() => {
          historyDelFile(oldkey)
          handlerSaveHistoryOpenFile(newKey, name, "open")

          this.props.updateFileProp({ path: oldkey, newProps: openFile })
          if (this.props.activeTabKey === oldkey) {
            this.props.changeActivePath(newKey)
          }
          if (openFile.suffix === "ipynb" || openFile.suffix === "idpnb") {
            if(!isNeedUpdateKernel){
              PubSub.publish("updateCollapseKernel")
              isNeedUpdateKernel = true
            }
            this.props.contentCatAsync(openFile).then((res) => {
              if (res.payload) {
                //contentCatAsync
                const { inode } = res.payload.response.content.metadata
                const path = res.payload.path
                this.props.variableListAsync({ path, inode })
              }else{
                this.appComponentData.notebookTabRef.current.removeTab(openFile.path)
              }
            })
          }
          this.props.updatePath({ path: oldkey, newPath: newKey })
        })
      }
    } else {
      const filterTabList = tabList.filter(
        (item) => findFileTreeParentKey(item.path) === oldkey
      )
      if (filterTabList.length > 0) {
        filterTabList.forEach((item) => {
          const openFile = {
            path: newKey + "/" + item.name,
            name: item.name,
            suffix: item.name.slice(item.name.lastIndexOf(".") + 1),
          }
          const itemOldKey = oldkey + "/" + item.name
          const itemNeWKey = newKey + "/" + item.name

          this.appComponentData.notebookTabRef.current &&
          this.appComponentData.notebookTabRef.current
            .updateDeleteFlag(itemOldKey)
            .then(() => {
              historyDelFile(itemOldKey)
              handlerSaveHistoryOpenFile(itemNeWKey, item.name, "open")

              this.props.updateFileProp({
                path: itemOldKey,
                newProps: openFile,
              })
              if (this.props.activeTabKey === itemOldKey) {
                this.props.changeActivePath(itemNeWKey)
              }
              if (openFile.suffix === "ipynb" || openFile.suffix === "idpnb") {

                if(!isNeedUpdateKernel){
                  PubSub.publish("updateCollapseKernel")
                  isNeedUpdateKernel = true
                }

                this.props.contentCatAsync(openFile).then((res) => {
                  if (res.payload) {
                    const path = res.payload.path
                    const { inode } = res.payload.response.content.metadata
                    this.props.variableListAsync({ path, inode })
                  }else{
                    this.appComponentData.notebookTabRef.current.removeTab(openFile.path)
                  }
                })
              }
              this.props.updatePath({ path: itemNeWKey, newPath: itemOldKey })
            })
        })
      }
    }
  }
  wsDelete = (keys) => {
    if (keys.length === 1) {
      const key = keys[0]
      if (this.props.tabList.find((item) => item.path === key)) {
        this.appComponentData.notebookTabRef.current &&
        this.appComponentData.notebookTabRef.current.updateDeleteFlag(key).then(() => {
          this.appComponentData.notebookTabRef.current
            .removeTab(key)
            .then((newTargetKey) => {})
        })
      }
    } else {
      const promiseList = []
      for (let i = 0; i < keys.length; i++) {
        const key = keys[i]
        if (this.appComponentData.notebookTabRef.current) {
          promiseList.push(this.appComponentData.notebookTabRef.current.updateDeleteFlag(key))
        }
      }
      Promise.all(promiseList).then((results) => {
        this.props.clearTabsFromList(keys)
        this.props.updateNotebookListFromTabListAsync()
      })
    }
  }

  checkHealth = () => {
    const _this = this
    let healthTimer = setTimeout(() => {
      if (!_this.state.isHealth) {
        clearTimeout(healthTimer)
        probeApi
          .health()
          .then(function () {
            _this.setState({ isHealth: true })
            _this.openLocalFile()

            // 获取项目信息
            globalData.appComponentData.getProjectInfo()
            // 版本更新通知
            const majorVersionUpdate = cookie.load("majorVersionUpdate")
            if (!majorVersionUpdate) {
              // 更新版本时的通知
              // _this.versionUpdateNotification()
            }
          })
          .catch(function (err) {
            _this.checkHealth()
          })
        this.healthCheckCount += 1
        if (this.healthCheckCount > 30) {
          this.setState({ initTips: "正在为您初始化环境..." })
        } else if (this.healthCheckCount > 15) {
          this.setState({ initTips: "正在为您配置硬盘存储资源..." })
        }
      }
    }, 2000)
  }
  loadLocales() {
    let currentLocale = intl.determineLocale({
      urlLocaleKey: "locale",
      cookieLocaleKey: "locale",
    })
    // react-intl-universal 是单例模式, 只应该实例化一次
    intl
      .init({
        currentLocale,
        locales,
      })
      .then(() => {
        this.setState({ intlInit: true })
      })
  }


  versionUpdateNotification = () => {
    notification.open({
      message: (
        <div style={{ color: "red" }}>
          <NotificationOutlined />
          <span style={{ paddingLeft: 10 }}>通知</span>
        </div>
      ),
      description:
        "为了不断提高产品与服务品质，我们将于5月13日（本周五）19:00-21:00停机改版升级，本次升级会重置您测试账号中的数据，请您提前备份文件管理器中的数据。感谢您一如既往的支持。",
      className: "custom-class",
      duration: null,
      placement: "bottomRight",
      style: {
        width: 600,
      },
      onClose: () => {
        cookie.save("majorVersionUpdate", true)
      },
    })
  }

  render() {
    const { projectInfo } = globalData.appComponentData
    // 查看projectInfo对象中 是否有后端返回的id属性
    return this.state.isHealth && this.state.intlInit && projectInfo.id ? (
      <Provider
        value={{
          addFileAndHandleIpynb: this.addFileAndHandleIpynb,
          onWsSelected:this.wsSelected,
          onWsDelete:this.wsDelete,
          onRename:this.wsRename,
        }}
      >
          <div className={this.state.theme}
            tabIndex="3"
            onFocus={() => GloablFocus({
              openGlobalSearch(event){
                event.preventDefault();
                PubSub.publish("openGlobalSearch")
              }
            })}
            onBlur={() => GloablBlur()}
          >
            <Layout
              className="layout">
              {
                this.props.children
              }
            </Layout>
          </div>
      </Provider>
    ) : (
      <AppLoading initTips={this.state.initTips} />
    )
  }
}
export default connect(
  (state) => ({
    tabList: state.filesTab.fileList,
    activeTabKey: state.filesTab.activePath,
    clientHeight: state.config.clientHeight,
    clientWidth: state.config.clientWidth,
  }),
  {
    addNewFile,
    contentCatAsync,
    changeActivePath,
    updateFileProp,
    updatePath,
    variableListAsync,
    updateClientHeight,
    updateClientWidth,
    clearTabsFromList,
    updateNotebookListFromTabListAsync,
    addFileAndContentAsync
  }
)(withRouter(App))
