import { useEffect, useState, createContext, useContext } from "react"
import { useDispatch } from "react-redux"
import { contentUpdateCellSource } from "../../../../store/features/notebookSlice"
import sneltoets from '@idp/global/sneltoets';
import { useUpdateEffect } from "ahooks"
import { Col, Row } from "antd"
import RightTopBar from "../cellTools/RightTopBar"
import VisualizationContent from "./VisualizationContent"
import { NotebookComponentContext } from "../../Notebook"

export const VisualizationCellContext = createContext({})

function VisualizationCell(props) {
  const {
    path,
    cellId,
    index,
    source,
    executionTime,
    metadata,
    focus,
    runCell,
    runCellAndGotoNext,
    stopCell,
    onFocus,
    onBlur,
    outputs,
    cellProp,
    runCurrentCellAndAbove,
    runCurrentCellAndBelow,
  } = props

  const { cells, cellProps } = useContext(NotebookComponentContext)

  const [hideInput, setHideInput] = useState(false)
  const [toggleValue, setToggleValue] = useState(false)
  const [nextToggleValue, setNextToggleValue] = useState(false)
  const [value, setValue] = useState(source)
  const dispatch = useDispatch()

  useEffect(() => {
    setHideInput(sneltoets.collapseAllInput)
  }, [sneltoets.collapseAllInput])

  useUpdateEffect(() => {
    // 强制刷新 Command + Enter
    runCell(cellId)
    // 当runCell时 保存对应的数据
    dispatch(contentUpdateCellSource({ path, cellId }))
  }, [toggleValue])
  useUpdateEffect(() => {
    // 强制刷新 Shift + Enter
    runCellAndGotoNext(cellId)
    // 当runCell时 保存对应的数据
    dispatch(contentUpdateCellSource({ path, cellId }))
  }, [nextToggleValue])

  return (
    <VisualizationCellContext.Provider
      value={{ cellId, metadata, outputs, runCell, cellProp, path }}
    >
      <Row className="code-cell" tabIndex={-1} onClick={() => onFocus(cellId)} onBlur={() => onBlur(cellId, '')}>
        <Col className="sider-left-controlbar">
          <div
            className="editor-cell-statebar"
            onClick={() => setHideInput(!hideInput)}
          >
            1
          </div>
        </Col>
        <Col span={24} className="code-wrapper">
          <Row
            className="code-editor-topbar"
            style={{
              display: focus ? "" : "none",
            }}
          >
            <RightTopBar
              outputs={outputs}
              path={path}
              cellId={cellId}
              index={index}
              stopCell={stopCell}
              cells={cells}
              cellProps={cellProps}
              runCurrentCellAndAbove={runCurrentCellAndAbove}
              runCurrentCellAndBelow={runCurrentCellAndBelow}
            />
          </Row>
          <Row className="tool-placeholder">
            <div
              className="execution-time-text"
              style={{
                display: focus ? "" : "none",
              }}
            >
              {executionTime}
            </div>
          </Row>
          <Row>{hideInput ? null : <VisualizationContent />}</Row>
          <Row
            className="show-cell-btn"
            style={{
              display: hideInput ? "block" : "none",
            }}
            onClick={() => setHideInput(!hideInput)}
          >
            {`{...}`}
          </Row>
        </Col>
      </Row>
    </VisualizationCellContext.Provider>
  )
}

export default VisualizationCell
