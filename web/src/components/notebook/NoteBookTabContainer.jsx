import React, {
  useImperativeHandle,
  useMemo,
  useState,
  useRef,
  useEffect,
} from "react"
import { Tabs, Tooltip } from "antd"
import "./NoteBookTabContainer.less"
import { useDispatch, useSelector } from "react-redux"
import {
  contentSnapshot,
  contentUpdateAllCellSource,
  resetNotebookState,
  selectNotebookList,
  updateNotebookListFromTabListAsync,
  clearPopupList,
  resolveClearPopupList,
} from "../../store/features/notebookSlice"
import {
  changeActivePath,
  clearFileList,
  clearLeftOrRightFileList,
  clearOtherAllFileList,
  deleteFile,
  selectActivePath,
  selectTabList,
  updateFileDeleteFlag,
} from "../../store/features/filesTabSlice"
import Notebook from "./Notebook"
import TextEditor from "../TextEditor"
import CsvMode from "../csv/CsvMode"
import SvgEditor from "../editor/svg"
import { useNotebookItem } from "../../utils/hook/useActiveCellProps"
import { changeHistoryFileOpenOrClose } from "../../utils/storage"
import { projectId } from "../../store/cookie"
import { Item, Menu, useContextMenu } from "react-contexify"
import handClickSvg from "../../assets/handClick.svg"
import PubSub from "pubsub-js"
import classNames from "classnames"
import intl from "react-intl-universal"
import { PythonEditor } from "../editor/python"
import ZipView from "@components/zipview/zipview"
import { LockOutlined } from "@ant-design/icons"
import { PythonLibEditor } from "../editor/pythonLib"
import MarkdownFile from '../markdownFile/markdownFile'
import { observer } from "mobx-react"

import IdpTerminal from '@/idp/lib/terminal';

const { TabPane } = Tabs

const FancyRenderFile = React.forwardRef((props, ref) => {
  const { item, workSpaceHeight, sourceVeiw } = props
  let detailItem
  detailItem = useNotebookItem(item.path)
  if (item.suffix === "ipynb" || item.suffix === 'idpnb') {
    return (
      <Notebook
        key={item.path}
        deleteflag={item.deleteFlag}
        {...detailItem}
        ref={ref}
        workSpaceHeight={workSpaceHeight}
        sourceVeiw={sourceVeiw}
      />
    )
  } else if (item.contentType === 'text') {
    switch (item.suffix) {
      case "py":
        return (
          <PythonEditor
            key={item.path}
            deleteflag={item.deleteFlag}
            workSpaceHeight={workSpaceHeight}
            path={item.path}
            posLine={item.posLine}
            item={item}
          />
        )
      case "csv":
        return (
          <CsvMode
            ref={ref}
            key={item.path}
            item={item}
            deleteflag={item.deleteFlag}
            workSpaceHeight={workSpaceHeight}
            sourceVeiw={sourceVeiw}
          />
        )
      case "svg":
        return (
          <SvgEditor
            ref={ref}
            key={item.path}
            item={item}
            deleteflag={item.deleteFlag}
            workSpaceHeight={workSpaceHeight}
            sourceVeiw={sourceVeiw}
          />
        )
      case "md":
        return (
          <MarkdownFile
            ref={ref}
            key={item.path}
            item={item}
            deleteflag={item.deleteFlag}
            workSpaceHeight={workSpaceHeight}
            sourceVeiw={sourceVeiw}
          />
        )
      default:
        return (
          <TextEditor
            ref={ref}
            key={item.path}
            {...item}
            deleteflag={item.deleteFlag}
            workSpaceHeight={workSpaceHeight}
            sourceVeiw={sourceVeiw}
            item={item}
          />
        )
    }
  } else if (item.contentType === 'zip') {
    return (
      <ZipView
        ref={ref}
        key={item.path}
        item={item}
        deleteflag={item.deleteFlag}
        workSpaceHeight={workSpaceHeight}
        sourceVeiw={sourceVeiw}
      />
    )
  }
})

function NoteBookTabContainer(props, ref) {
  const { workSpaceHeight, sourceVeiw } = props
  useImperativeHandle(ref, () => ({
    updateDeleteFlag(targetKey) {
      return Promise.resolve(dispatch(updateFileDeleteFlag(targetKey)))
    },

    removeTab(targetKey) {
      return remove(targetKey)
    },
    fun() {
      if (activeKey.endsWith("ipynb") || activeKey.endsWith("idpnb")) {
        // console.log(renderFileRef.current[`${activeKey}`].switchRecalculation())
        renderFileRef.current[`${activeKey}`].switchRecalculation()
      } else {
        console.log(activeKey)
      }
    },
    runCell: () =>
      (activeKey.endsWith("ipynb") || activeKey.endsWith("idpnb")) &&
      renderFileRef.current[`${activeKey}`].runCell(),
    runPreCell: () =>
      (activeKey.endsWith("ipynb") || activeKey.endsWith("idpnb")) &&
      renderFileRef.current[`${activeKey}`].runPreCell(),
    runNextCell: () =>
      (activeKey.endsWith("ipynb") || activeKey.endsWith("idpnb")) &&
      renderFileRef.current[`${activeKey}`].runNextCell(),
    runAllCell: () =>
      (activeKey.endsWith("ipynb") || activeKey.endsWith("idpnb")) &&
      renderFileRef.current[`${activeKey}`].runAllCell(),
    stopCell: () =>
      (activeKey.endsWith("ipynb") || activeKey.endsWith("idpnb")) &&
      renderFileRef.current[`${activeKey}`].stopCell(),
    stopAllCell: () =>
      (activeKey.endsWith("ipynb") || activeKey.endsWith("idpnb")) &&
      renderFileRef.current[`${activeKey}`].stopAllCell(),
    restartKernel: () =>
      (activeKey.endsWith("ipynb") || activeKey.endsWith("idpnb")) &&
      renderFileRef.current[`${activeKey}`].restartKernel(),
    updateDeleteflag: (value) =>
      (activeKey.endsWith("ipynb") || activeKey.endsWith("idpnb")) &&
      renderFileRef.current[`${activeKey}`].updateDeleteflag(value),
  }))

  const [rightSelectPath, setRightSelectPath] = useState("")
  const activeKey = useSelector(selectActivePath)
  const tabList = useSelector(selectTabList)
  const notebookList = useSelector(selectNotebookList)
  const dispatch = useDispatch()
  const renderFileRef = useRef({})
  const [kernelData, setKernelData] = useState([])

  useEffect(() => {
    const updateKernelDataSubscriber = PubSub.subscribe(
      "updateKernelData",
      (msg, data) => {
        setKernelData(data)
      }
    )
    return () => {
      PubSub.unsubscribe(updateKernelDataSubscriber)
    }
  }, [])

  const libFile = (item) => {
    return {
      title: (
        <div>
          <LockOutlined style={{ fontSize: 12, marginRight: 5 }} />
          <span>
            {item.name}
          </span>
        </div>
      ),
      content: () => (
        <PythonLibEditor
          key={item.path}
          {...item}
          workSpaceHeight={workSpaceHeight}
          sourceVeiw={sourceVeiw}
        />
      ),
      key: item.path,
    }
  }

  const panes = useMemo(() => {
    return tabList.map((item) => {
      console.log(item.path)
      if (item.path.startsWith('file:///')) return libFile(item)
      const isIpynb = item.suffix === "ipynb" || item.suffix === "idpnb"
      let iconStatus
      if (isIpynb) {
        const findResult = kernelData.find(
          (KernelItem) => KernelItem.notebookPath === item.path
        )
        if (findResult) {
          iconStatus = findResult.state
        }
      }

      return {
        title: (
          <span>
            {iconStatus ? (
              <span
                className={classNames("circle-icon", { [iconStatus]: true })}
              ></span>
            ) : null}{" "}
            {item.name}
          </span>
        ),
        content: () => (
          <FancyRenderFile
            item={item}
            notebookList={notebookList}
            workSpaceHeight={workSpaceHeight}
            sourceVeiw={sourceVeiw}
            ref={(element) => {
              renderFileRef.current[item.path] = element
            }}
          />
        ),
        key: item.path,
      }
    })
  }, [tabList, kernelData, workSpaceHeight])

  const onChange = (activeKey) => {
    const theFileType = activeKey.slice(activeKey.lastIndexOf(".") + 1);
    const rightBarOpenStatus = IdpTerminal.rightBarOpenStatus;
    if (theFileType === 'ipynb' || theFileType === 'idpnb') {
      IdpTerminal.setTerminalVisabled(true);
      IdpTerminal.setRightSidePanelWidth(rightBarOpenStatus ? -300 : 0, false);
      IdpTerminal.setNext(IdpTerminal.next);
      IdpTerminal.updateWorkspaceTabBarClickFile(activeKey);
    } else if(theFileType === 'py') {
      IdpTerminal.setTerminalVisabled(true);
      IdpTerminal.setRightSidePanelWidth(0, false);
      IdpTerminal.setNext(IdpTerminal.next);
      IdpTerminal.updateWorkspaceTabBarClickFile(activeKey);
    }  else {
      IdpTerminal.setTerminalVisabled(false);
    }
    dispatch(changeActivePath(activeKey))
    // 切换时清空 回撤数组
    dispatch(clearPopupList())
    dispatch(resolveClearPopupList())
  }
  const onEdit = (targetKey, action) => {
    switch (action) {
      case "remove":
        remove(targetKey)
        break
    }
  }

  const remove = async (targetKey) => {
    let newActiveKey = activeKey
    let lastIndex
    panes.forEach((pane, i) => {
      if (pane.key === targetKey) {
        lastIndex = i - 1
      }
    })
    const newPanes = panes.filter((pane) => pane.key !== targetKey)
    if (newPanes.length && activeKey === targetKey) {
      if (lastIndex >= 0) {
        newActiveKey = newPanes[lastIndex].key
      } else {
        newActiveKey = newPanes[0].key
      }
    } else if (newPanes.length === 0) {
      newActiveKey = ''
    }

    const item = tabList.find((item) => item.path === targetKey)
    dispatch(deleteFile(targetKey))
    changeHistoryFileOpenOrClose({
      status: "close",
      projectId,
      path: targetKey,
    })
    dispatch(changeActivePath(newActiveKey))

    if (item.suffix === "ipynb" || item.suffix === "idpnb") {
      if (!item.deleteFlag) {
        dispatch(contentUpdateAllCellSource({ path: targetKey }))
        dispatch(contentSnapshot({ path: targetKey }))
      }
      dispatch(resetNotebookState(targetKey))
    }
    const theFileType = newActiveKey.slice(newActiveKey.lastIndexOf(".") + 1);
    if (theFileType === 'ipynb' || theFileType === 'idpnb' ||  theFileType === 'py' ) {
      IdpTerminal.setTerminalVisabled(true);
      IdpTerminal.setNext(IdpTerminal.next);
      IdpTerminal.updateWorkspaceTabBarClickFile(newActiveKey);
    } else {
      IdpTerminal.setTerminalVisabled(false);
    }
    return newActiveKey
  }

  const MENU_ID = "MENU_ID"

  const { show } = useContextMenu({
    id: MENU_ID,
  })

  function handleContextMenu(event, item) {
    event.preventDefault()
    const { key } = item
    show(event, {
      props: {
        key,
      },
    })
    setRightSelectPath(key)
  }
  const closeAllTab = ({ event, props }) => {
    dispatch(clearFileList())
    dispatch(updateNotebookListFromTabListAsync())
  }

  const closeOtherAllTab = ({ event, props }) => {
    dispatch(clearOtherAllFileList(rightSelectPath))
    dispatch(updateNotebookListFromTabListAsync())
  }

  const closeLeftOrRightTab = (type) => {
    return ({ event, props }) => {
      dispatch(clearLeftOrRightFileList({ path: rightSelectPath, type }))
      dispatch(updateNotebookListFromTabListAsync())
    }
  }
  const fileTreeHandleClick = (key) => {
    return (event) => {
      event.stopPropagation()
      PubSub.publish("updateSelectKeys", key)
    }
  }
  // 切换时 清空 撤回cell数组
  // const clearWithdrawList = () => {

  // }
  return (
    <div id={"notebook-tab-container"}>
      <Tabs
        hideAdd
        type="editable-card"
        onChange={onChange}
        activeKey={activeKey}
        onEdit={onEdit}
      >
        {panes.map((pane) => (
          <TabPane
            tab={
              <Tooltip
                mouseEnterDelay={0.8}
                title={
                  <span>
                    {pane.key}{" "}
                    <img
                      onClick={fileTreeHandleClick(pane.key)}
                      style={{
                        verticalAlign: "middle",
                        marginLeft: 8,
                        cursor: "pointer",
                      }}
                      src={handClickSvg}
                      alt=""
                    />
                  </span>
                }
                placement="bottom"
              >
                <span
                  onMouseDown={(event)=>{
                    event.preventDefault()
                    event.stopPropagation()
                    if(event.button ===1){
                      remove(pane.key)
                    }
                  }}
                  onContextMenu={(event) => {
                    handleContextMenu(event, pane)
                  }}
                >
                  {pane.title}
                </span>
              </Tooltip>
            }
            key={pane.key}
            closable={pane.closable}
          >
            {pane.content()}
          </TabPane>
        ))}
      </Tabs>

      <Menu style={{ zIndex: 1000 }} id={MENU_ID}>
        <Item
          onClick={({ event, props }) => {
            remove(rightSelectPath)
          }}
        >
          {intl.get("CLOSE")}
        </Item>
        <Item onClick={closeAllTab}>{intl.get("CLOSE_ALL_TABS")}</Item>
        <Item onClick={closeLeftOrRightTab("left")}>
          {intl.get("CLOSE_ALL_TABS_ON_THE_LEFT")}
        </Item>
        <Item onClick={closeLeftOrRightTab("right")}>
          {intl.get("CLOSE_ALL_TABS_ON_THE_RIGHT")}
        </Item>
        <Item onClick={closeOtherAllTab}>{intl.get("CLOSE_OTHER_TABS")}</Item>
      </Menu>
    </div>
  )
}

export default observer(React.forwardRef(NoteBookTabContainer))
