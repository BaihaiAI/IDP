import { useEffect, useState, useContext, useMemo, useRef } from "react"
import { useDispatch } from "react-redux"
import { updateCellMetadata } from "../../../../store/features/notebookSlice"
import intl from "react-intl-universal"
import sneltoets from '@idp/global/sneltoets';
import { useUpdateEffect } from "ahooks"
import { Col, Row, Select } from "antd"
import RightTopBar from "../cellTools/RightTopBar"
import { NotebookComponentContext } from "../../Notebook"
import { useSetState } from 'ahooks'
import './explorationCell.less'


function ExplorationCell(props){
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
  const { cells, cellProps, variableList } = useContext(NotebookComponentContext)
  const [hideInput, setHideInput] = useState(false)
  const selectParentRef = useRef()
  const [visible, setVisible] = useState(!sneltoets.collapseAllOutput);

  const [metaDataInfo, setMetaDataInfo] = useSetState({
    df_name: metadata.df_name || "",
  })
  const dispatch = useDispatch()


  useEffect(() => {
    const {df_name} = metadata
    setMetaDataInfo({
      df_name: df_name || '',
    })
  }, [metadata])

  const dataFrameVariableList = useMemo(() => {
    return variableList.filter((item) => item.type === "dataframe")
  }, [variableList])

  useEffect(() => {
    setHideInput(sneltoets.collapseAllInput)
    
  }, [sneltoets.collapseAllInput])


  
  const dataFrameNameChange = (value) => {
    console.log(value)
    setMetaDataInfo({
      df_name: value || "",
    })
    const newMetaData = {
      ...metadata,
      df_name: value || "",
    }

    dispatch(updateCellMetadata({ path, cellId, metadata: newMetaData }))
  }

  const toggleVisible = () => {
    setVisible(!visible);
  };

  const parseOutputs = () => {
    let html = '';
    for (const output of outputs) {
      if ('data' in output) {
        const outData = output['data'];
        if ('text/html' in outData) {
          html = outData['text/html'];
        }
      }
    }
    return html;
  }

  return (
    <Row className="code-cell" tabIndex={-1} 
        onClick={() => onFocus(cellId)} 
        onBlur={() => onBlur(cellId, '')}
    >
        <Col className="sider-left-controlbar">
          <div className="editor-cell-statebar" onClick={() => setHideInput(!hideInput)}></div>
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
          {/* <Row>{hideInput ? null : <VisualizationContent />}</Row> */}
          <Row>
            {hideInput? null : (
              <div className={"exploration-container"}>
                <div className="content">
                  <div className="table" ref={selectParentRef}>
                    <span>变量:</span>
                    <Select
                      // getPopupContainer={()=>selectParentRef.current}
                      value={metaDataInfo.df_name}
                      style={{ minWidth: "100px" }}
                      bordered={false}
                      onSelect={dataFrameNameChange}
                    >
                      {dataFrameVariableList.map((item) => {
                        return (
                          <Select.Option key={item.name} value={item.name}>
                            {item.name}
                          </Select.Option>
                        )
                      })}
                    </Select>
                  </div>
                </div>
              </div>
            )}
          </Row>
          <Row
            className="show-cell-btn"
            style={{
              display: hideInput ? "block" : "none",
            }}
            onClick={() => setHideInput(!hideInput)}
          >
            {`{...}`}
          </Row>
        <Row className="demo-cell output-cell"
          style={{
            display: (outputs && outputs.length !== 0) ? '' : 'none',
          }}>
          <Col className="cell-side-panel" span={1}></Col>
          <Col span={24} className="code-cell-wrapper">
            <Row className="code-cell">
              <Col className="sider-left-controlbar">
                <div className="editor-cell-statebar" onClick={toggleVisible}></div>
              </Col>
              <Col span={24}>
                <div
                  className="output-cell-show"
                  style={{ display: visible ? 'none' : 'block' }}
                  onClick={toggleVisible}
                >
                  {intl.get('SHOW_OUTPUTS')}
                </div>

                <div style={{ padding: "10px", overflowX: 'scroll', height: '415px', maxHeight: 415, display: visible ? '' : 'none' }}>
                  <iframe style={{width: '100%', height: '100%'}} src={`data:text/html;charset=utf-8,${encodeURIComponent(parseOutputs())}`}></iframe>
                </div>
              </Col>
            </Row>
          </Col>
        </Row>
        </Col>
      </Row>
  )
}

export default ExplorationCell;