import React, { useState, useRef } from "react"
import { useDispatch } from "react-redux"
import intl from "react-intl-universal"
import MarkdownEditor from "@uiw/react-markdown-editor"

import { Tooltip, Button, Col, Row } from "antd"
import RightTopBar from '../cellTools/RightTopBar';

import Icons from "../../../Icons/Icons"
import "./markdownCell.less"
import {
  contentDelCell,
  delCell,
} from "../../../../store/features/notebookSlice"


const MarkdownCell = (props) => {
  const { path, index, cellId, focus, onFocus, onBlur, value, cellProps, stopCell, runCurrentCellAndAbove, runCurrentCellAndBelow } = props
  const [theme, setTheme] = useState("lightTheme")
  const [visible, setVisiable] = useState(true)
  const markdownCellRef = useRef()
  const [z_index, setZ_index] = useState(7)
  const dispatch = useDispatch()

  const onClick = () => {
    if (!focus) {
      onFocus(cellId, markdownCellRef.current.editor)
    }
  }

  const handleBlur = () => {
    onBlur(cellId, markdownCellRef.current.editor.getValue())
  }

  const toggleVisible = () => {
    setVisiable(!visible)
  }

  // 删除 cell
  const deleteCell = () => {
    dispatch(contentDelCell({ path, index, cellId, bol: false })).then(() =>
      dispatch(delCell({ path, index }))
    )
  }

  const mechodSetZ_index = (num) => {
    setZ_index(num)
  }

  return (
    <div className={theme} onClick={onClick}>
      <Row className="code-cell">
        <Col className="sider-left-controlbar">
          <div className="editor-cell-statebar" onClick={toggleVisible}></div>
        </Col>
        <Col span={24} className="code-wrapper">
          <Row
            className="code-editor-topbar"
            onMouseEnter={() => setZ_index(100)}
            onMouseLeave={() => setZ_index(7)}
            style={{ display: !focus ? "none" : "flex", zIndex: z_index }}
          >
            <Col className="code-editor-topbar-actions">
              {/* <Tooltip placement="bottom" title={intl.get("DELETECELL")}>
                <Button
                  icon={<Icons.BHDeleteIcon />}
                  size="small"
                  type="text"
                  onClick={() => deleteCell()}
                ></Button>
              </Tooltip> */}
              <RightTopBar
                cellProps={cellProps}
                path={path}
                cellId={cellId}
                index={index}
                stopCell={stopCell}
                runCurrentCellAndAbove={runCurrentCellAndAbove}
                runCurrentCellAndBelow={runCurrentCellAndBelow}
                mechodSetZ_index={mechodSetZ_index}
              />
            </Col>
          </Row>
          <Row
            className={!focus ? "md-editor-preview-only anticon" : "anticon"}
            id={"markdown-" + cellId}
            style={{ display: visible ? "inline" : "none" }}
          >
            <MarkdownEditor
              value={value}
              onBlur={handleBlur}
              visible={true} //开启预览
              options={{ lineNumbers: false }}
              ref={markdownCellRef}
              initialMode={false} // 判断是否是 "插件初始化" 模式
            />
          </Row>
          <Row
            style={{
              cursor: "pointer",
              lineHeight: "50px",
              paddingLeft: "25px",
              color: "#999",
              display: visible ? "none" : "",
            }}
            onClick={toggleVisible}
          >
            {intl.get("EXPAND_TO_VIEW")}
          </Row>
        </Col>
      </Row>
    </div>
  )
}

export default React.memo(MarkdownCell, (prevProps, nextProps) => {
  return prevProps.value === nextProps.value && prevProps.focus === nextProps.focus && prevProps.index === nextProps.index
});
