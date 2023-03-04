import React, { useCallback, useEffect, useRef, useState } from "react"
import { CaretRightOutlined, EllipsisOutlined } from "@ant-design/icons"
import intl from "react-intl-universal"
import { Button, Collapse, List, message, Popover, Tooltip } from "antd"
import kernelApi from "../../../services/kernelApi"
import {
  getNoteBookIndexFromPath,
  kernelExecuteStateAsync,
  updateCellProps,
} from "@/store/features/notebookSlice"
import { store } from "@/store"
import PubSub from "pubsub-js"
import { useDispatch } from "react-redux"
import PropTypes from "prop-types"
import { updateKernelList, removeKernel, removeAllKernel } from "@/store/features/kernelSlice"
import './FileTreeCollapse.less';

const { Panel } = Collapse


FileTreeCollapse.propTypes = {
  clickNotebookState:PropTypes.func.isRequired,
}

function FileTreeCollapse(props) {



  const dispatch = useDispatch()
  const [kernelShutDownState, setKernelShutDownState] = useState([]) //按钮loading态
  const [kernelStopState, setKernelStopState] = useState([]) //停止按钮loading态
  const [kernelSuspendState, setKernelSuspendState] = useState([]) // 挂起按钮loading状态
  const [collapseActiveKeys, setCollapseActiveKeys] = useState([])
  const [visible, setVisible] = useState(false)
  const [kernelData, setKernelData] = useState([])
  const intervalIdRef = useRef(null)

  /*kernel相关的逻辑 start*/
  const getKernelData = () => {
    const pathname = window.location.pathname;
    pathname.indexOf('workspace') > -1 && kernelApi.kernelState().then((response) => {
      setKernelData(response.data)
      dispatch(updateKernelList(response.data))
    })
  }

  useEffect(()=>{
    const subscriber = PubSub.subscribe("updateCollapseKernel",()=>{
      getKernelData()
    })
    return ()=>{
      PubSub.unsubscribe(subscriber)
    }
  },[])


  useEffect(() => {
    PubSub.publish("updateKernelData", kernelData)
  }, [kernelData])



  useEffect(() => {
    getKernelData()
    intervalIdRef.current = window.setInterval(() => {
      getKernelData()
    }, 5000)

    return () => {
      window.clearInterval(intervalIdRef.current)
    }
  }, [])


  useEffect(()=>{
    window.addEventListener('visibilitychange',()=>{
      const { visibilityState } = document
      if(visibilityState ==='hidden'){
        window.clearInterval(intervalIdRef.current)
      }else{
        getKernelData()
        intervalIdRef.current = window.setInterval(() => {
          getKernelData()
        }, 5000)
      }
    })

  },[])

  const genExtra = () => {
    return (
      <Popover
        content={
          <>
            <div
              className="kernel-panel-popover-item"
              onClick={() => {
                shutDownAllKernel("all")
              }}
            >
              {intl.get("CLOSE_ALL_PROCESS")}
            </div>
            <div
              className="kernel-panel-popover-item"
              onClick={() => {
                shutDownIdleKernel("idle")
              }}
            >
              {intl.get("CLOSE_IDLE_PROCESS")}
            </div>
          </>
        }
        trigger="click"
        visible={visible}
        placement="bottomRight"
        onVisibleChange={handleVisibleChange}
      >
        <EllipsisOutlined
          style={{ fontSize: "1.4em" }}
          onClick={(ev) => {
            ev.stopPropagation()
            setVisible(true)
          }}
        />
      </Popover>
    )
  }

  //运行中panel，右上角下拉菜单相关逻辑
  const handleVisibleChange = (visible) => {
    setVisible(visible)
  }

  // kernel 列表操作按钮
  const kernelAction = (item, i) => {
    // if (showKernelAction[i]) {
    let buttonList = []
    if (item.state === "pause") {
      buttonList.push(
        <Button
          key={`suspend-${i}`}
          type="link"
          id={`suspend-${i}`}
          onClick={() =>
            resumeKernel(
              item.kernelName,
              item.kernelIdentity,
              item.notebookPath,
              i,
              item.inode
            )
          }
          size="small"
          loading={kernelSuspendState[i]}
        >
          {intl.get("RESUMKERNEL")}
        </Button>
      )
    } else {
      buttonList.push(
        <Button
          key={`pause-${i}`}
          type="link"
          id={`pause-${i}`}
          onClick={() =>
            pauseKernel(
              item.kernelName,
              item.kernelIdentity,
              item.notebookPath,
              i,
              item.inode
            )
          }
          size="small"
          loading={kernelSuspendState[i]}
        >
          {intl.get("SUSPENDKERNEL")}
        </Button>
      )
    }
    buttonList.push(
      <Button
        key={`stop-${i}`}
        type="link"
        id={`stop-${i}`}
        onClick={() => {
          stopKernel(
            item.kernelIdentity,
            i,
            item.kernelName,
            item.notebookPath,
            item.inode
          )
        }}
        size="small"
        disabled={item.state === "idle" || item.state === "pause"}
        loading={kernelStopState[i]}
      >
        {intl.get("SHUTDOWNKERNEL")}
      </Button>
    )
    buttonList.push(
      <Button
        key={`shutdown-${i}`}
        type="link"
        id={`shutdown-${i}`}
        disabled={item.state === "pause"}
        onClick={() => {
          shutDownKernel(
            item.kernelIdentity,
            i,
            item.kernelName,
            item.notebookPath,
            false,
            item.inode
          )
        }}
        size="small"
        loading={kernelShutDownState[i]}
      >
        {intl.get("CLOSEKERNEL")}
      </Button>
    )
    return buttonList
  }
  /* end kernel相关的逻辑*/

  // 面板的展开折叠相关逻辑 start
  const collapseChange = useCallback((keys) => {
    const findResult = keys[keys.length - 1] === "dataset"
    if (findResult) {
      setCollapseActiveKeys(["dataset"])
    } else {
      setCollapseActiveKeys(keys.filter((item) => item !== "dataset"))
    }
  }, [])
  // end 面板的展开折叠相关逻辑

  const pauseKernel = (name, identity, path, index, inode) => {
    let suspendState = [...kernelSuspendState]
    suspendState[index] = true
    setKernelSuspendState(suspendState)
    kernelApi
      .kernelPause({
        name,
        identity,
        inode,
        path,
      })
      .then(() => {
        setTimeout(() => {
          getKernelData()
          suspendState[index] = false
          setKernelSuspendState(suspendState)
          dispatch(kernelExecuteStateAsync({ path }))
        }, 500)
      })
      .catch((err) => {
        console.log(err)
        message.error(err.toString())
        suspendState[index] = false
        setKernelSuspendState(suspendState)
      })
  }

  const resumeKernel = (name, identity, path, index, inode) => {
    let suspendState = [...kernelSuspendState]
    suspendState[index] = true
    setKernelSuspendState(suspendState)
    kernelApi
      .kernelResume({
        name,
        identity,
        inode,
        path,
      })
      .then(() => {
        setTimeout(() => {
          getKernelData()
          suspendState[index] = false
          setKernelSuspendState(suspendState)
        }, 500)
      })
      .catch((err) => {
        console.log(err)
        message.error(err.toString())
        suspendState[index] = false
        setKernelSuspendState(suspendState)
      })
  }

  //关闭所有内核
  const shutDownAllKernel = () => {
    kernelData.map((item, i) => {
      shutDownKernel(
        item.kernelIdentity,
        i,
        item.kernelName,
        item.notebookPath,
        false,
        item.inode
      )
    })
    setVisible(false)
  }
  //关闭所有空闲内核
  const shutDownIdleKernel = () => {
    kernelData.map((item, i) => {
      if (item.state === "idle") {
        shutDownKernel(
          item.kernelIdentity,
          i,
          item.kernelName,
          item.notebookPath,
          false,
          item.inode
        )
      }
    })
    setVisible(false)
  }

  const shutDownKernel = (id, idx, kernelname, path, type, inode) => {
    const params = {
      identity: id,
      path,
      name: kernelname,
      restart: type,
      inode,
    }
    //设置按钮状态
    kernelShutDownState[idx] = 1
    setKernelShutDownState(Object.assign([], kernelShutDownState))
    kernelApi
      .shutdown(params)
      .then(function () {
        setTimeout(function () {
          deleteKernelFromData(path)
          // 暂时由前端手动控制cellProp对象的状态
          setTimeout(() => {
            resetKernel(path)
          }, 300)
        }, 500)
        dispatch(removeKernel({ path }))
      })
      .catch(function (error) {
        setTimeout(function () {
          // deleteKernelFromData(error.message)
          message.error(error.toString())
          kernelShutDownState[idx] = undefined
          setKernelShutDownState([...kernelShutDownState])
        }, 500)
      })
  }

  const deleteKernelFromData = (notebookPath) => {
    kernelData.some((item, i) => {
      if (notebookPath.includes(item.notebookPath)) {
        kernelData.splice(i, 1)
        kernelShutDownState[i] = 0
        setKernelData(Object.assign([], kernelData))
        setKernelShutDownState(Object.assign([], kernelShutDownState))
        return true
      }
    })
    kernelData.length <= 1 && setKernelShutDownState([])
  }

  const resetKernel = (path) => {
    const notebookList = store.getState().notebook.notebookList
    const index = getNoteBookIndexFromPath(path, notebookList)
    if (index === -1) return
    const cellProps = notebookList[index].cellProps
    let nextCellProps = {}
    for (const key in cellProps) {
      nextCellProps[key] = { ...cellProps[key], state: "ready" }
    }
    dispatch(updateCellProps({ path, cellProps: nextCellProps }))
  }

  const stopKernel = (id, idx, kernelname, path, inode) => {
    const params = {
      session: "bbb5b78a-6001-415b-a1f9-45037d6a3045",
      kernel: kernelname,
      identity: id,
      batchId: new Date().getTime(),
      inode,
      path,
    }

    const kernelBtnState = [...kernelStopState]
    //设置按钮状态
    kernelBtnState[idx] = 1
    setKernelStopState(kernelBtnState)
    kernelApi
      .executeInterrupt(params)
      .then(function () {
        setTimeout(function () {
          setKernelStopState([])
          getKernelData()
        }, 500)
      })
      .catch(function (error) {
        console.log(error)
      })
  }

  return (
    <Collapse
      onChange={collapseChange}
      activeKey={collapseActiveKeys}
      bordered={false}
      expandIcon={({ isActive }) => (
        <CaretRightOutlined rotate={isActive ? 90 : 0} />
      )}
      className='workspace-run-list'
    >
      <Panel
        header={intl.get("ACTIVITY")}
        key="1"
        extra={genExtra()}
        style={{ whiteSpace: "nowrap", width: '100%' }}
      >
        <List
          size="small"
          style={{ height: 310, overflow: "auto" }}
          dataSource={kernelData}
          rowKey={'notebookPath'}
          renderItem={(item, i) => {
            const fileArr = item.notebookPath.split("/")
            const fileName = fileArr[fileArr.length-1]

            return (
              <List.Item actions={kernelAction(item, i)}>
                <List.Item.Meta
                  style={{
                    whiteSpace: "nowrap",
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                    cursor: "pointer",
                  }}
                  onClick={() => {
                    const strList = item.notebookPath.split("/")
                    const fileName = strList[strList.length - 1]
                    const newItem = { ...item, fileName }
                    props.clickNotebookState(newItem)
                  }}
                  title={
                    <span>
                    <i
                      className={
                        item.state === "idle"
                          ? "kernel-idle"
                          : item.state === "pause"
                            ? "kernel-pause"
                            : "kernel-busy"
                      }
                    >
                      {item.state === "idle"
                        ? intl.get("IDLE")
                        : item.state === "pause"
                          ? intl.get("SUSPENDKERNEL")
                          : intl.get("BUSY")}
                    </i>
                    <Tooltip
                      placement={"topLeft"}
                      title={item.notebookPath}
                      mouseEnterDelay={0.5}
                    >
                      <span>{fileName}</span>
                    </Tooltip>
                  </span>
                  }
                />
              </List.Item>
            )
          }}
        ></List>
      </Panel>
    </Collapse>
  )
}

export default FileTreeCollapse
