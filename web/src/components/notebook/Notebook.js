import React, {useState, useEffect, useRef, useImperativeHandle, useContext} from "react"
import { useDispatch, useSelector } from "react-redux"
import kernelApi from "../../services/kernelApi"
import { kernelWsSend } from "./lib/kernelWs"
import intl from "react-intl-universal"
import appContext from "../../context"
import { message, Spin, notification, Modal, Button } from "antd"

import ToolAddCell from "./cells/cellTools/ToolAddCell"

import { LspWebsocket } from "./lib/LspWebsocket"
import TopToolsBar from "./topToolsBar/TopToolsBar"
import {
  updateCellProps,
  updateCellPropFocus,
  updateCellProp,
  updateCell,
  contentAddCell,
  kernelExecuteStateAsync,
  variableListAsync,
  getNoteBookIndexFromPath,
  popUpTheCurrentReverseWithdrawal,
  contentWithdrawCell,
  updateCellPropState
} from "@/store/features/notebookSlice"
import Cell from "./cells/Cell"
import { userId, region, projectId ,teamId} from "@/store/cookie"
import { selectActivePath } from "@/store/features/filesTabSlice"

import "./Notebook.less"
import contentApi from "../../services/contentApi"
import {useMemoizedFn, useUnmount} from "ahooks"
import { store } from "@/store"
import { getHistoryOpenFile, saveHistoryOpenFile } from "@/utils/storage"
import PubSub from "pubsub-js"

import { NoteBookonBlur, NoteBookonFocuse } from '../workspace/keymap'
import {contentContext} from "@/layout/content"
import globalData from "@/idp/global"
import {observer} from "mobx-react"

let sendWs = null
let lspWebsocket = new LspWebsocket()
let lspStore = {
  lspKeywords: new Set(),
  lspQuickFixWords: null,
  lspDiagnostics: new Set(),
}
let deleteflag = false

export const NotebookComponentContext = React.createContext({})

const Notebook = (props, ref) => {
  const {
    path,
    cells,
    metadata,
    cellProps,
    isExecuting,
    isPaused,
    notebookJson,
    variableList,
    position,
    workSpaceHeight,
    sourceVeiw,
  } = props
  deleteflag = props.deleteflag


  const {setShowSaveVersion} =  useContext(contentContext)
  // lsp didOpen
  const lspDidOpen = () => {
    if (path && !lspWebsocket.didOpenFile[path]) {
      lspWebsocket.didOpen(path, props.content, lspStore)
    }
  }

  useImperativeHandle(ref, () => ({
    switchRecalculation,
    runCell: () => runCell(getFocusCellId()),
    runPreCell: runCurrentCellAndAbove,
    runNextCell: runCurrentCellAndBelow,
    runAllCell: () => runAllCell(),
    stopCell: () => cellRef.current.stopCell(getFocusCellId()),
    stopAllCell: () => stopAllCell(),
    restartKernel: () => restartKernel(),
    updateDeleteflag: (value) => {
      deleteflag = value
    },
  }))

  const dispatch = useDispatch()


  const handlerCellContainerScroll = useMemoizedFn(()=>{
    const focusCell = document.getElementById(`cellbox-${hasFocusCell()}`)
    if(!focusCell){
      return
    }
    const focusCellContainer = focusCell.parentNode
    const rect = focusCellContainer.getBoundingClientRect()
    const fixRect = document.getElementById(`cells-content${path}`).getBoundingClientRect()
    const distance = rect.top - fixRect.top
    if(distance> 592 || distance < -focusCellContainer.offsetHeight){
      // 查看有无代码提示框 如果有就删除
      const findDom = document.querySelector('.CodeMirror-hints')
      if(findDom){
        findDom.parentNode.removeChild(findDom)
      }
    }
  })

  useEffect(() => {
    const cellContainer =  document.getElementById(`cells-content${path}`)
    cellContainer.addEventListener('scroll',handlerCellContainerScroll)
    return () => {
      cellContainer.removeEventListener('scroll',handlerCellContainerScroll)
    }
  }, [])


  const [notebookLoaded, setNotebookLoaded] = useState(false)

  const hasFocusCell = () => {
    if (!cellProps) return null
    let focusId = false
    const cellIdList = Object.keys(cellProps)
    for (let i = 0; i < cellIdList.length; i++) {
      const cellsIdElement = cellIdList[i]
      if (cellProps[cellsIdElement].focus) {
        focusId = cellsIdElement
        break
      }
    }
    return focusId
  }

  const writeFocusIdToLocal = () => {
    const focusId = hasFocusCell()
    if (focusId) {
      let historyOpenFileObj = getHistoryOpenFile()

      if (Array.isArray(historyOpenFileObj[projectId])) {
        const fileArr = historyOpenFileObj[projectId]
        if (fileArr.length > 0) {
          if (fileArr[0].name === path) {
            fileArr[0].focusId = focusId
          } else {
            const findIndex = fileArr.findIndex((item) => item.name === path)
            if (findIndex) {
              fileArr[findIndex] = { ...fileArr[findIndex], focusId }
            }
          }
        } else {
          fileArr.unshift({ name: path, focusId })
        }
        historyOpenFileObj[projectId] = fileArr
        saveHistoryOpenFile(historyOpenFileObj)
      }
    }
  }

  useUnmount(() => {
    if(!props.deleteflag){
      //  在组件挂载前 如果有focusId 记录当前的focusId 保存到local中
      writeFocusIdToLocal()
    }
  })



  const saveVersion = (content) => {
    // console.log('执行了')
    const params = {
      path,
      label: intl.get("SAVE_VERSION_AUTO"),
    }
    contentApi.snapshot(params).catch((error) => {})
  }

  // 提供给菜单栏调用
  const cellRef = useRef()
  const getFocusCellId = () => {
    for (const key in cellProps) {
      if (cellProps[key].focus) {
        return cellProps[key].cellType === "code" ||
          cellProps[key].cellType === "sql"
          ? key
          : null
      }
    }
    return null
  }
  const getPreFocusCellId = (focusCellId) => {
    const cellId = focusCellId ? focusCellId : getFocusCellId()
    // console.log(cellId,'=====cellId',cells)
    let cellIdList = []
    for (let i = 0; i < cells.length; i++) {
      if (cells[i].metadata.id === cellId) {
        for (let j = 0; j <= i; j++) {
          if (cells[j]['cell_type'] === 'markdown') continue
          cellIdList.push(cells[j].metadata.id)
        }
        break
      }
    }
    return {cellIdList}
  }
  const getNextFocusCellId = (focusCellId) => {
    const cellId = focusCellId? focusCellId : getFocusCellId()
    let cellIdList = []
    for (let i = 0; i < cells.length; i++) {
      if (cells[i].metadata.id === cellId) {
        for (let j = i; j < cells.length; j++) {
          if (cells[j]['cell_type'] === 'markdown') continue
          cellIdList.push(cells[j].metadata.id)
        }
        break
      }
    }
    return cellIdList
  }

  const socketMessage = (msg) => {
    const msgJson = JSON.parse(msg)
    const { cellId,path } = msgJson
    /*    if (!msgJson["parent_header"]) return
    const msgId = msgJson["parent_header"]["msg_id"]
    const flag = msgId.slice(0, msgId.lastIndexOf("/"))
    const msgPath = flag.slice(0, flag.lastIndexOf("/"))
    const cellId = flag.slice(flag.lastIndexOf("/") + 1)*/

    if (projectId !== msgJson.projectId || teamId !== msgJson.teamId) return
    if (!path || !cellId ) return

    if ("error_on_receive" === msg) {
      dispatch(updateCellProp({ path, cellId, cellProp: { state: "ready" } }))
      message.error(intl.get("EDITOR_ERROR_2"))
      return
    }

    // kernel crash
    if (msgJson.content && msgJson.content.message === 'kernel crash') {
      dispatch(updateCellPropState({ path, state: 'ready'}))
      Modal.error({
        title: `${path} kernel crash`,
      });
      return
    }

    const notebookList = store.getState().notebook.notebookList
    for (const notebook of notebookList  ) {
      let cells = notebook.cells
      for (let i = 0; i < cells.length; i++) {
        if (cells[i].metadata.id === cellId) {

          let cell = { ...cells[i] }
          let cellState = "ready"
          if ("start_kernel" == msgJson["msgType"]) {
            cell["outputs"] = [{
              "name": "stdout",
              "outputType": "stream",
              "text": [intl.get("START_KERNEL")]
            }]
            cellState = "pending"
          } else if ("execute_input" === msgJson["msgType"]) {
            cell["outputs"] = []
            cellState = "executing"
          } else if ("reply_on_stop" === msgJson["msgType"] || ('status' === msgJson['msgType'] && 'idle' === msgJson['content']['execution_state'])) {
            cellState = "ready"
          } else {
            const res = sendWs.parseMessage(msgJson)
            if (Object.keys(res).length === 0) return

            // runtime_error 说明还没提交给kernel执行就已经出错了
            if ("runtime_error" === msgJson["msgType"]) {
              cell["outputs"] = []
              if (msgJson["content"] && msgJson["content"]["message"] === 'No enough resource') {
                Modal.error({
                  title: intl.get('NO_ENOUGH_RESOURCE'),
                });
              }
            }

            if ("outputs" in res) {
              res["outputs"] = [...cell["outputs"], ...res["outputs"]]
            }
            cell = { ...cell, ...res }

            if (
              "duration" in msgJson.content ||
              "runtime_error" === msgJson["msgType"] ||
              "error" === msgJson["msgType"] ||
              (msgJson["content"] && msgJson["content"]["is_busy"] === false)
            ) {
              cellState = "ready"
              const { inode } = notebook.metadata;
              dispatch(variableListAsync({path, inode}))
            } else {
              cellState = "executing"
              // document.getElementById(`cellbox-${cellId}`).scrollIntoView({
              //   behavior: "smooth",
              // });
            }
          }
          // console.log(msgJson["msgType"],'=====',cellState)
          dispatch(updateCell({ path, cellId, cell }))
          dispatch(
            updateCellProp({
              path,
              cellId,
              cellProp: { state: cellState },
            })
          )

          // 获取当前focus cell的光标，页面刷新后自动定位到当前的光标
          let cellInstance = undefined
          let cursor = undefined
          for (const key in cellProps) {
            if (cellProps[key].cellFocus) {
              cellInstance = cellProps[key].instance
              if (cellInstance) {
                cursor = cellInstance.getCursor()
              }
            }
          }

          // 当前滚动条位置
          const scrollTop = document.getElementById(`cells-content${path}`)
            ? document.getElementById(`cells-content${path}`).scrollTop
            : 0

          if (cellInstance) {
            cellInstance.setCursor(cursor)
            document.getElementById(`cells-content${path}`).scrollTo(0, scrollTop)
          }
          break
        }
      }
    }
  }

  const handleHeartbeat = (isAlive) => {
    globalData.appComponentData.setSocketAlive(isAlive)

    if (!isAlive) {
      socketErrorCount += 1
      if (socketErrorCount > 1 && showSocketError) {
        notification.warning({
          key: "socketError",
          message: intl.get("NETWORK_ABNORMAL"),
          description: intl.get("NETWORK_ERROR_DESCRIPTION"),
          duration: 5,
          placement: "bottomRight",
        })
        socketErrorCount = 0
        showSocketError = false
      }

      // 当websocket断掉之后 会执行resetKernel这个函数 导致当前cell的cellProps全部变为ready
      // 注释掉这段代码后 如果websocket重连后 是可以有正常效果的
      // resetKernel()
    } else {
      socketErrorCount = 0
      showSocketError = true
    }
  }


  sendWs = new kernelWsSend({
    path,
    handleSocketMessage: socketMessage,
    handleHeartbeat,
  })
  sendWs.ws.socketMessage = socketMessage
  let socketErrorCount = 0
  let showSocketError = true

  const activeKey = useSelector(selectActivePath)
  if (path === activeKey) {
    sendWs.ws.socketMessage = socketMessage
    sendWs.ws.handleHeartbeat = handleHeartbeat
  }

  const arrToString = (arr) => {
    let str = ""
    for (const item of arr) {
      str += item
    }
    return str
  }

  useEffect(() => {
    const updateScrollSubscriber = PubSub.subscribe(
      "updateNotebookScroll",
      (msg, data) => {
        setNewCellId(data)
      }
    )
    return () => {
      PubSub.unsubscribe(updateScrollSubscriber)
    }
  }, [])

  const [newCellId, setNewCellId] = useState(null) // 用于判断是否添加了cell，添加cell后滚动条自动滚动到cell所在位置
  useEffect(() => {
    if (newCellId) {
      const findIndex = cells.findIndex(
        (cell) => cell.metadata.id === newCellId
      )
      if (findIndex !== 0) {
        let dom = document.getElementById(`cellbox-${newCellId}`)
        dom.scrollIntoView(true)
        setNewCellId(null)
      }
    }
  }, [newCellId])
  const onAddCell = (cellId) => {
    setNewCellId(cellId)
  }
  // 检查后台是否已经在跑代码
  let checkRunTimer = null
  const checkRun = (interval) => {
    if (!showSocketError) return // 如果网络错误，跳过
    if (!checkRunTimer) {
      checkRunTimer = setTimeout(() => {
        let flag = true
        for (const key in cellProps) {
          const state = cellProps[key].state
          if (state === "executing") {
            return
          } else if (state === "pending") {
            flag = false
          }
        }
        if (!flag) {
          notification.warning({
            key: "socketError",
            message: intl.get("SERVICE_ERROR"),
            description: intl.get("SERVICE_RUN_ERROR"),
            duration: 5,
            placement: "bottomRight",
          })
        }
        clearTimeout(checkRunTimer)
        checkRunTimer = null
      }, interval)
    }
  }

  const doRunCell = (cell, isAll, batchId) => {
    const cellId = cell.metadata.id
    if ("" === cellId) {
      message.warning(intl.get("EDITOR_ERROR_1"))
      return
    }

    // 可视化cell判断是否选择了变量和X/Y维度
   /* if (cell["cell_type"] === 'visualization') {
      let warnMsg = ''
      if (!cell.metadata || !cell.metadata.chart || !cell.metadata.df_name) {
        warnMsg = '请选择变量和维度'
      } else {
        if (!cell.metadata.chart['x'] || !cell.metadata.chart['y']) {
          warnMsg = '请选择维度(X轴)和维度(Y轴)'
        }
      }
      if ('' !== warnMsg) {
        Modal.warning({ title: warnMsg })
        return
      }
    }*/

    let code = ""
    if (isAll) {
      code = arrToString(cell["source"])
    } else {
      if (cellProps[cellId] && cellProps[cellId].instance) {
        if (cellProps[cellId].instance.somethingSelected()) {
          code = cellProps[cellId].instance.getSelection()
        }
      }
      code = code || arrToString(cell["source"])
    }

    // 将开始运行时  execution_time 为null
    const newCell = {
      ...cell,
      "execution_time":null
    }
    dispatch(updateCell({path,cellId,cell:newCell}))

    // 发送websocket执行
    const randomInt = Math.ceil(Math.random() * 10000)
    // console.log(cell['metadata'])
    const resource = resourceRef.current.getResource()
    if (resource.numCpu === 0 || resource.memory === 0) {
      message.warning('请先设置资源', 3)
      return
    }
    resourceRef.current.setKernelIsExecuting(true)
    const status = sendWs.sendMessage({
      inode:metadata.inode,
      teamId,
      projectId,
      region,
      // session: "bbb5b78a-6001-415b-a1f9-45037d6a3045",
      userId,
      executeType: "cell",
      // msgId: `${path}/${cellId}/${randomInt}`,
      path: path,
      cellId: cellId,
      cellType: cell["cell_type"],
      code: code,
      meta: { uid: userId, ...cell["metadata"] },
      // kernel: metadata.kernelspec.name,
      // identity: metadata.kernelspec.identity,
      // recordExecuteTime: "true",
      batchId: batchId || new Date().getTime(),
      resource: {
        numCpu: resource.numCpu,
        numGpu: resource.numGpu,
        memory: resource.memory,
        priority: resource.priority,
      }
    })


    if (!status) {
      notification.warning({
        key: "socketError",
        message: intl.get("NETWORK_ABNORMAL"),
        description: intl.get("NETWORK_ERROR_DESCRIPTION"),
        duration: 5,
        placement: "bottomRight",
      })
      return
    }

    dispatch(
      updateCellProp({
        path,
        cellId,
        cellProp: { state: "pending",hasExecuted:true },
      })
    )

    // 10秒后检查后端是否已经开始跑了
    checkRun(10 * 1000)
  }

  const sendInputRequest = (cellId, value) => {
    const status = sendWs.sendMessage({
      inode: metadata.inode,
      teamId,
      projectId,
      region,
      userId,
      batchId: new Date().getTime(),
      executeType: "cell",
      path: path,
      cellId: cellId,
      cellType: 'code',
      code: '',
      meta: {},
      inputReply: value,
    })

    if (!status) {
      notification.warning({
        key: "socketError",
        message: intl.get("NETWORK_ABNORMAL"),
        description: intl.get("NETWORK_ERROR_DESCRIPTION"),
        duration: 5,
        placement: "bottomRight",
      })
      return
    }
  }

  const findFocusCellIdAndType = () => {
    const notebookList = store.getState().notebook.notebookList
    const notebook = notebookList.find((item) => item.path === path)
    const cellProps = notebook.cellProps
    let focusCellId = null
    let focusCellType = null
    for (const key in cellProps) {
      if (cellProps[key].focus) {
        focusCellId = key
        focusCellType = cellProps[key]["cellType"]
        break
      }
    }
    return { focusCellId, focusCellType }
  }

  const runCell = (cellId) => {
    const oldFocusCell = findFocusCellIdAndType()
    for (const cell of cells) {
      if (cell.metadata.id === cellId) {
        // 当运行时 处理新的cell滚动
        // handleScroll(cellId,cell['cell_type'])
        dispatch(updateCellPropFocus({ cellId, path }))
        const focusCell = findFocusCellIdAndType()
        if (oldFocusCell.focusCellId !== focusCell.focusCellId) {
          resetCellPosition(oldFocusCell.focusCellId)
        }
        handleScroll(focusCell.focusCellId, focusCell.focusCellType)
        doRunCell(cell, false)
        return
      }
    }
    message.warning(intl.get("EDITOR_ERROR_1"))
  }

  const runCellAndGotoNext = (cellId) => {
    let findCurrentCell = false
    let currentCell = null
    let nextCell = null
    for (const cell of cells) {
      if (
        findCurrentCell &&
        (cell["cell_type"] === "code" || cell["cell_type"] === "sql")
      ) {
        nextCell = cell
        break
      }
      if (cell.metadata.id === cellId) {
        findCurrentCell = true
        currentCell = cell
      }
    }
    console.log(nextCell, "========nextcell")
    if (nextCell) {
      cellProps[nextCell.metadata.id].instance.focus()
    } else {
      dispatch(
        contentAddCell({
          path,
          index: cells.length,
          cellType: "code",
          cells
        })
      )
        .unwrap()
        .then((res) => {
          const { data } = res
          onAddCell(data.metadata.id)
        })
    }
    console.log(currentCell)
    doRunCell(currentCell, false)
  }
  // 运行全部 cell
  const runAllCell = () => {
    let timer = null
    let count = 0
    const batchId = new Date().getTime()

    // 因为使用setInterval 与websocket 出现了异步代码执行顺序交叉的问题 所以暂时换成普通循环
    cells.forEach((cell, index) => {
      if (cell["cell_type"] !== "markdown") {
        doRunCell(cell, true, batchId)
      }
    })
    /*timer = setInterval(function () {
      if (count === cells.length) {
        clearInterval(timer)
        return
      }
      if (cells[count]["cell_type"] !== "markdown") {
        doRunCell(cells[count], true, batchId)
      }
      count += 1
    }, 0);*/
  }

  // 运行当前单元格及上方所有单元格
  const runCurrentCellAndAbove = (cellId) => {
    cellId && dispatch(updateCellProp({
      path,
      cellId,
      cellProp: { focus: true },
    }))
    const {cellIdList} = getPreFocusCellId(cellId)
    // console.log(cellIdList)

    for (const cellId of cellIdList) {
      runCell(cellId)
    }
  }

  // 运行当前单元格及下方所有单元格
  const runCurrentCellAndBelow = (cellId) => {

    const cellIdList = getNextFocusCellId(cellId)
    for (const cellId of cellIdList) {
      runCell(cellId)
    }

    cellId && dispatch(updateCellProp({
      path,
      cellId,
      cellProp: { focus: true },
    }))
  }

  // 重置kernel环境，
  const resetKernel = () => {
    let nextCellProps = {}
    for (const key in cellProps) {
      nextCellProps[key] = { ...cellProps[key], state: "ready" }
    }
    dispatch(updateCellProps({ path, cellProps: nextCellProps }))
  }

  const stopAllCell = () => {
    const params = {
      session: "bbb5b78a-6001-415b-a1f9-45037d6a3045",
      inode: metadata.inode,
      batchId: new Date().getTime(),
      path: path,
    }
    kernelApi
      .executeInterrupt(params)
      .then(function (response) {
        message.success(intl.get("KERNEL_STOP_SUCCEEDED"))
        resetKernel()
      })
      .catch(function (err) {
        if (err.message === "Request kernel already terminated or not exist.") {
          message.success(intl.get("KERNEL_STOP_SUCCEEDED"))
          resetKernel()
        } else {
          message.error(intl.get("KERNEL_STOP_FAILED"))
        }
      })
  }

  const restartKernel = (callback) => {
    const resource = resourceRef.current.getResource()
    kernelApi
      .restart({
        inode: metadata.inode,
        path: path,
        numCpu: resource.numCpu,
        numGpu: resource.numGpu,
        memory: resource.memory,
        priority: resource.priority,
      })
      .then(function (response) {
        message.success(intl.get("KERNEL_RESTART_SUCCEEDED"))
        const { inode } = metadata;
        dispatch(variableListAsync({path, inode}))
        resetKernel()
        callback && callback()
      })
      .catch(function (err) {
        message.error(intl.get("KERNEL_RESTART_FAILED"))
        resetKernel()
        callback && callback()
      })
  }

  // 恢复运行
  const resumeRun = () => {
    kernelApi
      .kernelResume({
        name: metadata.kernelspec.name,
        identity: metadata.kernelspec.identity,
        inode: metadata.inode,
        path,
      })
      .then(() => {

      })
      .catch((err) => {
        console.log(err)
        message.error(err.toString())
      })
  }

  let markdownFixedRef = useRef(false)
  const scrollMarkDownHandler = (focusCellId) => {
    const currentCell = document.getElementById("cellbox-" + focusCellId)

    if (!currentCell) {
      return
    }

    const currentMarkDownToolbar =
      currentCell.getElementsByClassName("md-editor-toolbar")[1]
    const markdownEditorPreview =
      currentCell.getElementsByClassName("md-editor-preview")[0]

    if (!currentMarkDownToolbar || !markdownEditorPreview) {
      return
    }
    const markdownRect = currentMarkDownToolbar.getBoundingClientRect()
    const currentCellRect = currentCell.getBoundingClientRect()

    if (markdownRect.top < 92 && !markdownFixedRef.current) {
      currentMarkDownToolbar.style.position = "fixed"
      currentMarkDownToolbar.style.top = "108px"
      currentMarkDownToolbar.style.left = currentCellRect.left + 15 + "px"
      markdownFixedRef.current = true
    }
    //下边界固定至边界
    if (
      Math.abs(currentCellRect.top - 92 - currentMarkDownToolbar.offsetHeight) >
      currentCell.offsetHeight - markdownEditorPreview.offsetHeight - 20
    ) {
      currentMarkDownToolbar.style.position = "absolute"
      currentMarkDownToolbar.style.top = "-14px"
      currentMarkDownToolbar.style.left = "15px"
      markdownFixedRef.current = false
    }

    if (currentCellRect.top >= 92 && markdownFixedRef.current) {
      currentMarkDownToolbar.style.position = "absolute"
      currentMarkDownToolbar.style.top = "-14px"
      currentMarkDownToolbar.style.left = "15px"
      markdownFixedRef.current = false
    }
  }

  //cell侧边按钮随动
  let toolbarfixedRef = useRef(false)
  let [hasForceRender, setHasForceRender] = useState(false)

  const switchRecalculation = () => {
    const currentcell = document.getElementById("cellbox-" + hasFocusCell())
    const toolbar = document.getElementById("cellbar-" + hasFocusCell())
    if (!currentcell || !toolbar) {
      return
    }

    const pos = currentcell.getBoundingClientRect()
    const contentHeight = currentcell.offsetHeight
    const toolbarHeight = toolbar.offsetHeight
    let tpos = toolbar.getBoundingClientRect()

    toolbar.style.position = "absolute"
    toolbar.style.top = "22px"
    toolbar.style.left = "16px"
    if (pos.top >= 108) {
    } else {
      setTimeout(() => {
        tpos = toolbar.getBoundingClientRect()
        toolbar.style.position = "fixed"
        toolbar.style.top = "109px"
        toolbar.style.left = tpos.left + "px"
      }, 500)
    }
  }

  const handleScroll = (focusCellId, focusCellType) => {
    if (focusCellType === "visualization") {
      return
    }
    // if (!focusCellId || !focusCellType) return;
    if (focusCellType === "markdown") {
      //  如果当前滚动的单元格是markdown 则调用执行markdown的滚动逻辑函数
      scrollMarkDownHandler(focusCellId)
    } else {
      // 如果当前滚动的单元格是非 markdown的 则执行之前的滚动代码逻辑
      const currentcell = document.getElementById("cellbox-" + focusCellId)
      const toolbar = document.getElementById("cellbar-" + focusCellId)
      if (!currentcell || !toolbar) {
        return
      }

      const pos = currentcell.getBoundingClientRect()
      const contentHeight = currentcell.offsetHeight
      const toolbarHeight = toolbar.offsetHeight
      let tpos = toolbar.getBoundingClientRect()
      const tposLeft = tpos.left
      // 上下边界进入
      if (pos.top <= 92 && !toolbarfixedRef.current) {
        toolbar.style.position = "fixed"
        toolbar.style.top = "109px"
        toolbar.style.left = tposLeft + "px"
        toolbarfixedRef.current = true

        // 新增的代码 为了处理 时间随着运行按钮滚动的问题
        // 同时给RunCellTool组件  新增了 isScrollFixed focusCell两个props
        if (!hasForceRender) {
          // hasForceRender = true
          // setHasForceRender(true)
        }
      }
      //下边界固定至边界
      // if (Math.abs(pos.top - 108 - toolbarHeight) > contentHeight) {
      if (pos.top + contentHeight < 101 + toolbarHeight) {
        resetCellPosition(focusCellId)
      }
      //恢复
      if (pos.top > 92 && toolbarfixedRef.current) {
        resetCellPosition(focusCellId)
        // 新增的代码 为了处理 时间随着运行按钮滚动的问题
        // hasForceRender = false
        // setHasForceRender(false)
      }
    }
  }

  const scrollEvent = (e) => {
    let focusCellId = null
    let focusCellType = null
    for (const key in cellProps) {
      if (cellProps[key].focus) {
        focusCellId = key
        focusCellType = cellProps[key]["cellType"]
        break
      }
    }
    handleScroll(focusCellId, focusCellType)

    // const cellAll = document.getElementById("cells-content")
    // if (cellAll.scrollTop < 32) {
    //   for (const key in cellProps) {
    //     if (cellProps[key].cellType === "code") {
    //       fiexdCellRun(key)
    //       break
    //     }
    //   }
    // }
  }
  const fiexdCellRun = (key) => {
    const currentcell = document.getElementById("cellbox-" + key)
    const toolbar = document.getElementById("cellbar-" + key)
    if (!currentcell || !toolbar) {
      return
    }
    const pos = currentcell.getBoundingClientRect()
    //恢复
    resetCellPosition(key)
    // 新增的代码 为了处理 时间随着运行按钮滚动的问题
    // hasForceRender = false
    setHasForceRender(false)
  }
  const resetCellPosition = (cellId) => {
    const toolbar = document.getElementById("cellbar-" + cellId)
    if (cellId && toolbar) {
      toolbar.style.position = "absolute"
      toolbar.style.top = "22px"
      toolbar.style.left = "16px"
      toolbarfixedRef.current = false
    }
  }

  useEffect(() => {
    if (path) {
      setNotebookLoaded(true)
      lspDidOpen()
      dispatch(kernelExecuteStateAsync({ path }))
      const { inode } = metadata
      dispatch(variableListAsync({path, inode}))
      const content = JSON.parse(props.content)
      if (
        content.cells.length > 0 &&
        content.cells[0].source &&
        content.cells[0].source.length > 0
      ) {
        saveVersion(props.content)
      }
      /* dispatch(contentCatAsync({ path }))
         .unwrap()
         .then((res) => {
           setNotebookLoaded(true)
           dispatch(updatePath({ path }))
           lspWebsocket.didOpen(path, res.content, lspStore)
           dispatch(kernelExecuteStateAsync({ path }))
           // 如果内容为空则不保存版本
           const content = JSON.parse(res.content)
           if (
             content.cells.length > 0 &&
             content.cells[0].source &&
             content.cells[0].source.length > 0
           ) {
             saveVersion(res.content)
           }
         })
         .catch((err) => {
           console.error(err.message)
           if ("Notebook file not found." === err.message) {
             message.error(intl.get("NOTEBOOK_FILE_OPEN_ERROR_2"))
           } else {
             message.error(err.toString())
           }
         })*/
    }

    return () => {
      if (!deleteflag) {
        /*
        dispatch(contentUpdateAllCellSource({ path }))
        dispatch(contentSnapshot({ path }))
        */
      }
      // dispatch(resetNotebookState(path))
    }
  }, [path])

  useEffect(() => {
    const activePath = store.getState().filesTab.activePath;
    const subscribe = PubSub.subscribe(`noteBookFouce${activePath}`, () => {
      document.getElementById(`cells-content${path}`)?.focus()
    })
    return () => {
      PubSub.unsubscribe(subscribe)
    }
  }, [path]);

  let resourceRef = useRef();

  return (
    <div>
      <NotebookComponentContext.Provider
        value={{
          path,
          cells,
          metadata,
          cellProps,
          isExecuting,
          variableList,
        }}
      >
        <TopToolsBar
          hasEffectiveClick={notebookLoaded}
          isExecuting={isExecuting}
          isPaused={isPaused}
          runAllCell={runAllCell}
          stopAllCell={stopAllCell}
          restartKernel={restartKernel}
          resumeRun={resumeRun}
          saveVersion={() => setShowSaveVersion(true)}
          resourceRef={resourceRef}
        />
        <div
          id={`cells-content${path}`}
          className="cells-wrapper"
          onScroll={scrollEvent}
          tabIndex="2"
          onFocus={() => NoteBookonFocuse({
            enterCallback(){
              const notebookList =   store.getState().notebook.notebookList;
              const index = getNoteBookIndexFromPath(path, notebookList);
              cells.forEach((key, i) => {
                if(notebookList[index].cellProps[key.metadata.id].focus){
                  cellProps[key.metadata.id]?.instance?.focus()
                }
              })
            },
            downIncrementCallback(){
              cells.forEach((prop, i) => {
                if(cellProps[prop.metadata.id].focus){
                  dispatch(
                    contentAddCell({
                      path,
                      index: i,
                      cellType: "code",
                      cells
                    })
                  )
                    .unwrap()
                    .then((res) => {
                      const { data } = res
                      onAddCell(data.metadata.id)
                    })
                }
              })
            },
            upIncrementCallback(){
              cells.forEach((prop, i) => {
                if(cellProps[prop.metadata.id].focus){
                  dispatch(
                    contentAddCell({
                      path,
                      index: ++i,
                      cellType: "code",
                      cells
                    })
                  )
                    .unwrap()
                    .then((res) => {
                      const { data } = res
                      onAddCell(data.metadata.id)
                    })
                }
              })
            },
            saveSnapshot(e){
              if (e.keyCode == 83 && (navigator.platform.match("Mac") ? e.metaKey : e.ctrlKey)) e.preventDefault();
              saveVersion(props.content)
            },
            rebootKernel(){
              restartKernel()
            },
            stopAllCellKey(){
              stopAllCell()
            },
            deleteCell(){
              for(let key in cellProps){
                if(cellProps[key].focus){
                  const data = {
                    key,
                    bol: false
                  }
                  PubSub.publish("mapkeyDeleteCell", data)
                  break
                }
              }
            },
            addDownCell(){
              cells.forEach((prop, i) => {
                if(cellProps[prop.metadata.id].focus){
                  dispatch(
                    contentAddCell({
                      path,
                      index: ++i,
                      cellType: "code",
                      cells
                    })
                  )
                    .unwrap()
                    .then((res) => {
                      const { data } = res
                      onAddCell(data.metadata.id)
                    })
                }
              })
            },
            selectUnitAbove(){
              let cellId;
              const notebookList = store.getState().notebook.notebookList
              const index = getNoteBookIndexFromPath(path, notebookList)

              // notebookList[index].cellProps
              cells.forEach((key, i) => {
                if(i <= 0) return;
                if(notebookList[index].cellProps[key.metadata.id].focus){
                  cellId = cells[--i].metadata.id
                }
              })
              cellId && dispatch(updateCellProp({
                path,
                cellId,
                cellProp: { focus: true },
              }))
            },
            selectUnitBelow(){
              let cellId;
              const notebookList = store.getState().notebook.notebookList
              const index = getNoteBookIndexFromPath(path, notebookList)
              // notebookList[index].cellProps
              cells.forEach((key, i) => {
                if(i >= cells.length-1) return;
                if(notebookList[index].cellProps[key.metadata.id].focus){
                  cellId = cells[++i].metadata.id
                }
              })

              // console.log(cellId)
              cellId && dispatch(updateCellProp({
                path,
                cellId,
                cellProp: { focus: true },
              }))
            },
            runCurrentCellfocusNextCell(){
              for(let prop of Object.keys(cellProps)){
                if(cellProps[prop].focus){
                  runCellAndGotoNext(prop)
                  break
                }
              }
              //
            },
            runCurrentCell(){
              for(let prop of Object.keys(cellProps)){
                if(cellProps[prop].focus){
                  runCell(prop)
                  break
                }
              }
            },
            // 撤回
            withdrawCell(){
              const withdrawCell = store.getState().notebook.retractableCellsList
              withdrawCell?.length && dispatch(contentWithdrawCell({
                path,
                withdrawCell: withdrawCell[withdrawCell.length-1]
              }))
            },
            // 反向撤回
            reverseWithdrawal(){
              console.log('反向撤回 ')
              const reverseWithdrawCell = store.getState().notebook.reverseWithdrawalCellsList;
              if(reverseWithdrawCell?.length){
                let rwKey = reverseWithdrawCell[reverseWithdrawCell.length-1].deletedCellBody.metadata.id
                console.log(rwKey, reverseWithdrawCell, "====------")
                for(let prop in cellProps){
                  console.log(prop)
                  if(prop === rwKey){
                    const data = {
                      key: prop,
                      bol: true
                    }
                    PubSub.publish("mapkeyDeleteCell", data)
                    dispatch(popUpTheCurrentReverseWithdrawal())
                    break
                  }
                }
              }
            }
          })}
          onBlur={() => NoteBookonBlur()}
        >
          {notebookLoaded ? (
            <div
              id="cells_div"
              className="cells"
              style={{
                height: sourceVeiw ? (workSpaceHeight - 80) : (document.body.clientHeight - 140),
              }}
            >
              <div id="cell-container">
                {/* start 底部add button*/}
                {/*当没有任何单元格时 隐藏top 添加的btn 让bottom添加的btn 正常显示 则不会重叠了*/}
                <div
                  style={{
                    display: cells.length ? "flex" : "none",
                  }}
                  className="addcell-btn addcell-top-btn"
                >
                  <ToolAddCell
                    key="top"
                    responsive={true}
                    onAddCell={onAddCell}
                    path={path}
                    cells={cells}
                    index={0}
                    className="add-cell-btn"
                  />
                </div>
                {/* end 底部add button*/}

                {cells.map((cell, i) => (
                  <Cell
                    cellProps={cellProps}
                    metadata={metadata}
                    key={cell.metadata.id}
                    path={path}
                    cellId={cell.metadata.id}
                    data={cell}
                    index={i}
                    isInMiddle={i < cells.length - 1}
                    runCell={runCell}
                    doRunCell={doRunCell}
                    runCellAndGotoNext={runCellAndGotoNext}
                    toolbarfixed={toolbarfixedRef.current}
                    lspStore={lspStore}
                    lspWebsocket={lspWebsocket}
                    lspDidOpen={lspDidOpen}
                    cellProp={cellProps[cell.metadata.id]}
                    onAddCell={onAddCell}
                    cellRef={cellRef}
                    resetCellPosition={resetCellPosition}
                    handleScroll={handleScroll}
                    findFocusCellIdAndType={findFocusCellIdAndType}
                    runCurrentCellAndAbove={runCurrentCellAndAbove}
                    runCurrentCellAndBelow={runCurrentCellAndBelow}
                    content={props.content}
                    cells={cells}
                    sendInputRequest={sendInputRequest}
                  />
                ))}
                {/* start 底部add button*/}
                <div className="addcell-btn addcell-bottom-btn">
                  <ToolAddCell
                    key="bottom"
                    responsive={false}
                    onAddCell={onAddCell}
                    path={path}
                    index={cells.length}
                    className="add-cell-btn"
                  />
                </div>
                {/* end 底部add button*/}
              </div>
            </div>
          ) : (
            <div
              style={{
                textAlign: "center",
                height: "100%",
                paddingTop: "100px",
              }}
            >
              <Spin size="large" />
            </div>
          )}
        </div>
      </NotebookComponentContext.Provider>
    </div>
  )
}

export default observer(React.forwardRef((Notebook)))
