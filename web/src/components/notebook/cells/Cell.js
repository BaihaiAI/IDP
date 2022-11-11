import { useImperativeHandle, useEffect, useState, useCallback, useMemo } from "react"
import { useDispatch } from "react-redux"
import intl from "react-intl-universal"
import { Row, Col, message } from "antd"

import ToolRunCell from "./cellTools/ToolRunCell"
import { CodeCell } from "./codeCell/CodeCell"
import { SqlCell } from "./sqlCell/SqlCell"
import MarkdownCell from "./markdownCell/MarkdownCell"
import ToolAddCell from "./cellTools/ToolAddCell"
import {
  updateCellProp,
  updateCellSource,
  contentUpdateCellSource,
} from "../../../store/features/notebookSlice"
import kernelApi from "../../../services/kernelApi"
import OutputCell from "./outputCell/OutputCell"
import "./cell.less"
import VisualizationCell from "./visualizationCell/VisualizationCell"
import { store } from "../../../store"

const arrToString = (arr) => {
  let str = ""
  for (const item of arr) {
    str += item
  }
  return str
}

let saveTimer = null

const Cell = (props) => {
  const {
    path,
    data,
    cellProp,
    cellProps,
    index,
    isInMiddle,
    runCell,
    doRunCell,
    runCellAndGotoNext,
    toolbarfixed,
    lspStore,
    lspWebsocket,
    onAddCell,
    cellRef,
    findFocusCellIdAndType,
    resetCellPosition,
    handleScroll,
    metadata,
    runCurrentCellAndAbove,
    runCurrentCellAndBelow,
    content,
    cells,
    sendInputRequest,
  } = props

  const cellId = data.metadata.id;
  const cellType = data["cell_type"];
  const source = arrToString(data["source"]);

  const dispatch = useDispatch()

  // 提供给菜单栏调用
  useImperativeHandle(cellRef, () => ({
    stopCell: (cellId) => stopCell(cellId),
  }))

  const outputIsError = (outputs) => {
    if (outputs) {
      for (const output of outputs) {
        if ("ename" in output) {
          return true
        }
      }
    }
    return false
  }

  const formatSource = (value) => {
    let source = []
    const arr = value.split("\n")
    for (let i = 0; i < arr.length - 1; i++) {
      source.push(`${arr[i]}\n`)
    }
    if (arr.length > 0) {
      source.push(arr[arr.length - 1])
    }
    return source
  }

  const stopCell = (cellId) => {
    const params = {
      session: "bbb5b78a-6001-415b-a1f9-45037d6a3045",
      path,
      batchId: new Date().getTime(),
      inode: metadata.inode,
    }
    return kernelApi
      .executeInterrupt(params)
      .then(function (response) {
        // 注释掉该代码 因为 websocket 消息里 会更新cellProps 而且比在此手动更新的信息 更准确
        // dispatch(updateCellProp({ cellId, cellProp: { state: 'ready' } }));
        message.success(intl.get("KERNEL_STOP_SUCCEEDED"))
      })
      .catch(function (err) {
        if (err.message === "Request kernel already terminated or not exist.") {
          // message.success(intl.get('KERNEL_STOP_SUCCEEDED'));
        } else {
          message.error(intl.get("KERNEL_STOP_FAILED"))
        }
        dispatch(updateCellProp({ path, cellId, cellProp: { state: "ready" } }))
      })
  }

  const handleEditorBlur = (cellId, value) => {
    dispatch(
      updateCellSource({
        path,
        cellId,
        source: formatSource(value),
      })
    )
    dispatch(contentUpdateCellSource({ path, cellId }))
    saveTimer && clearInterval(saveTimer)
    saveTimer = null
  }

  const handleEditorFocus = (cellId, instance) => {
    // 开启定时保存
    if (!saveTimer && instance) {
      saveTimer = setInterval(() => {
        dispatch(
          updateCellSource({
            path,
            cellId,
            source: formatSource(instance.getValue()),
          })
        )
        dispatch(contentUpdateCellSource({ path, cellId }))
      }, 5000)
    }

    const oldFocusCell = findFocusCellIdAndType()
    if (oldFocusCell.focusCellId !== cellId) {
      props.resetCellPosition(oldFocusCell.focusCellId)
    }

    let nextCellProp = instance ? { focus: true, instance } : { focus: true }
    dispatch(
      updateCellProp({
        path,
        cellId,
        cellProp: nextCellProp,
      })
    )

    const focusCell = findFocusCellIdAndType()
    props.handleScroll(focusCell.focusCellId, focusCell.focusCellType)
  }

  // format 执行时间
  const formatExecutionTime = (time) => {
    const duration = Number(time)
    if (isNaN(duration)) return time
    let executionTime = ""
    if (duration < 60 * 1000) {
      const time = (duration / 1000).toFixed(2)
      executionTime = `${time} s`
    } else if (duration >= 60 * 1000 && duration < 3600 * 1000) {
      const min = Math.floor(duration / (60 * 1000))
      const s = Math.floor((duration % (60 * 1000)) / 1000)
      executionTime = `${min} min ${s} s`
    } else if (duration >= 3600 * 1000) {
      const hour = Math.floor(duration / (3600 * 1000))
      const min = Math.floor((duration % (3600 * 1000)) / (60 * 1000))
      executionTime = `${hour} hour ${min} min`
    }
    return executionTime
  }

  const executionTime = useMemo(() => {
    if (data["execution_time"] === null) {
      return data["execution_time"]
    }
    return formatExecutionTime(data["execution_time"])
  }, [data["execution_time"]])

  const cellEditor = () => {
    switch (cellType) {
      case "code":
        return (
          <CodeCell
            outputs={data["outputs"]}
            cellProps={cellProps}
            path={path}
            cellId={cellId}
            index={index}
            source={source}
            executionTime={executionTime}
            metadata={data["metadata"]}
            focus={cellProp.focus}
            lspStore={lspStore}
            lspWebsocket={lspWebsocket}
            runCell={runCell}
            doRunCell={doRunCell}
            runCellAndGotoNext={runCellAndGotoNext}
            stopCell={stopCell}
            onFocus={handleEditorFocus}
            onBlur={handleEditorBlur}
            formatSource={formatSource}
            resetCellPosition={resetCellPosition}
            handleScroll={handleScroll}
            findFocusCellIdAndType={findFocusCellIdAndType}
            runCurrentCellAndAbove={runCurrentCellAndAbove}
            runCurrentCellAndBelow={runCurrentCellAndBelow}
            content={content}
            cells={cells}
            onAddCell={onAddCell}
          />
        )
      case "sql":
        return (
          <SqlCell
            outputs={data["outputs"]}
            cellProps={cellProps}
            path={path}
            cellId={cellId}
            index={index}
            source={source}
            executionTime={executionTime}
            metadata={data["metadata"]}
            focus={cellProp.focus}
            lspStore={lspStore}
            lspWebsocket={lspWebsocket}
            runCell={runCell}
            doRunCell={doRunCell}
            runCellAndGotoNext={runCellAndGotoNext}
            stopCell={stopCell}
            onFocus={handleEditorFocus}
            onBlur={handleEditorBlur}
            formatSource={formatSource}
            runCurrentCellAndAbove={runCurrentCellAndAbove}
            runCurrentCellAndBelow={runCurrentCellAndBelow}
          />
        )
      case "markdown":
        return (
          <MarkdownCell
            key={cellId}
            path={path}
            cellId={cellId}
            index={index}
            value={source}
            focus={cellProp.focus}
            onBlur={handleEditorBlur}
            onFocus={handleEditorFocus}

            cellProps={cellProps}
            stopCell={stopCell}
            runCurrentCellAndAbove={runCurrentCellAndAbove}
            runCurrentCellAndBelow={runCurrentCellAndBelow}
          />
        )
      case "visualization":
        return (
          <VisualizationCell
            path={path}
            cellId={cellId}
            index={index}
            source={source}
            executionTime={executionTime}
            metadata={data["metadata"]}
            focus={cellProp.focus}
            lspStore={lspStore}
            lspWebsocket={lspWebsocket}
            runCell={runCell}
            doRunCell={doRunCell}
            runCellAndGotoNext={runCellAndGotoNext}
            stopCell={stopCell}
            onFocus={handleEditorFocus}
            onBlur={handleEditorBlur}
            formatSource={formatSource}
            outputs={data.outputs}
            cellProp={cellProp}
            findFocusCellIdAndType={findFocusCellIdAndType}
            resetCellPosition={resetCellPosition}
            handleScroll={handleScroll}
            runCurrentCellAndAbove={runCurrentCellAndAbove}
            runCurrentCellAndBelow={runCurrentCellAndBelow}
          />
        )
      default:
        return <div></div>
    }
  }

  return (
    <div
      key={cellId}
      className={
        outputIsError(data["outputs"])
          ? "demo-wrapper cell-output-error"
          : "demo-wrapper"
      }
    >
      <div
        id={"cellbox-" + cellId}
        className={
          cellProp.focus
            ? cellType === "markdown"
              ? "demo-inner mk-demo-inner cell-active"
              : "demo-inner cell-active"
            : cellType === "markdown"
              ? "demo-inner mk-demo-inner"
              : "demo-inner"
        }
      >
        <Row className="demo-cell" key={`input-${cellId}`}>
          {/* start 侧边运行按钮区域*/}
          <Col span={1} className="cell-side-panel">
            <ToolRunCell
              outputs={data["outputs"]}
              outputIsError={outputIsError(data["outputs"])}
              cellProp={cellProp}
              cellId={cellId}
              cellType={cellType}
              executionCount={data["execution_count"]}
              executionTime={data["execution_time"]}
              focus={cellProp.focus}
              isScrollFixed={toolbarfixed}
              cellState={cellProp.state}
              stopCell={() => stopCell(cellId)}
              runCell={() => runCell(cellId)}
            />
          </Col>
          {/* end 侧边运行按钮区域*/}

          {/* start 右侧可编辑区域*/}
          <Col span={24} className="code-cell-wrapper">
            {cellEditor()}
          </Col>
          {/* end 右侧可编辑区域*/}
        </Row>
        {/* start 运行结果展现模块*/}

        {cellType === "markdown" || cellType === "visualization" ? (
          <div />
        ) : (
          <OutputCell path={path} cellProp={cellProp} key={cellId} cellId={cellId} outputs={data["outputs"]} sendInputRequest={sendInputRequest} />
          )}
        {/* end 运行结果展现模块*/}
      </div>
      {isInMiddle ? (
        <Row className="addcell-btn addcell-middl-btn">
          <ToolAddCell
            className="add-cell-btn"
            path={path}
            index={index + 1}
            responsive={true}
            isTop={false}
            onAddCell={onAddCell}
          />
        </Row>
      ) : (
        <div></div>
      )}
    </div>
  )
}

export default Cell
