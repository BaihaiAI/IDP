import React, { useState, useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import intl from "react-intl-universal";
import { useUpdateEffect } from 'ahooks';
import { UnControlled as CodeMirror } from 'react-codemirror2';

import { Row, Col, Input} from 'antd';

import {
  updateCellMetadata,
  updateCellProp,
  updateCellSource,
  contentUpdateCellSource
} from '../../../../store/features/notebookSlice';

import codemirror from "codemirror";
import "codemirror/lib/codemirror";
import "codemirror/keymap/sublime";
import "codemirror/theme/xq-light.css";
import "codemirror/addon/hint/show-hint";
import 'codemirror/addon/hint/sql-hint';
import "codemirror/addon/lint/lint.css";
import "codemirror/addon/comment/comment";
import "codemirror/addon/display/autorefresh";
import 'codemirror/mode/sql/sql';

import { extraKeys } from '../../lib/extraKeys';
import RightTopBar from '../cellTools/RightTopBar';
import ToolImpl from '@/idp/lib/tool/impl/toolImpl';
import './SqlCell.less'
import DataSourceSelect from './DataSourceSelect';


// 为了恢复版本时更新内容
let refreshSource = false;
export const setRefreshSource = (flag) => {
  refreshSource = flag
}

export const SqlCell = (props) => {
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
    formatSource,
    cellProps,
    runCurrentCellAndAbove,
    runCurrentCellAndBelow,
    outputs
  } = props;
  const dispatch = useDispatch();

  const [hideInput, setHideInput] = useState(false);
  const [editorInstance, setEditorInstance] = useState();
  const [toogleValue, setToogleValue] = useState(false);
  const [sToogleValue, setSToogleValue] = useState(false);
  const [value, setValue] = useState(source);

  useEffect(() => {
    // 回复版本时需要刷新source
    if (refreshSource) {
      refreshSource = false;
      setValue(source);
    }
  }, [source])

  useEffect(() => {
    setHideInput(ToolImpl.collapseAllInput);
  }, [ToolImpl.collapseAllInput]);

  useUpdateEffect(() => {
    // 强制刷新 Command + Enter
    runCell(cellId);
  }, [toogleValue])
  useUpdateEffect(() => {
    // 强制刷新 Shift + Enter
    runCellAndGotoNext(cellId)
  }, [sToogleValue])

  // 初始化codemirror时，保存codemirror实例
  const editorDidMount = (cellId, instance, value) => {
    setEditorInstance(instance);
    for (const key in cellProps) {
      if (key === cellId) {
        dispatch(updateCellProp({
          path,
          cellId: cellId,
          cellProp: { instance },
        }));
        // focus后能自动出现光标
        if (cellProps[key].focus) {
          instance.focus();
          if (cellProps[key].posLine) {
            instance.setCursor({ line: cellProps[key].posLine - 1, ch: 0 })
          }
        }
        break;
      }
    }
  }

  //高亮光标所在行
  const highlightcurrentLine = (cm, pos) => {
    if (cm.state.activeLine !== undefined) {
      cleanHighlightLine(cm, cm.state.activeLine)
    }
    cm.doc.addLineClass(pos.line, "background", "highlight-line")
    cm.doc.addLineClass(pos.line, "wrap", "highlight-wrap")
    cm.doc.addLineClass(pos.line, "gutter", "highlight-gutter")
    cm.state.activeLine = pos.line
  }
  const cleanHighlightLine = (cm) => {
    const ins = cm && cm.instance ? cm.instance : cm
    if (ins && ins.state && ins.state.activeLine !== undefined) {
      const line = ins.state.activeLine
      ins.doc.removeLineClass(line, "background", "highlight-line")
      ins.doc.removeLineClass(line, "gutter", "highlight-gutter")
      ins.doc.removeLineClass(line, "wrap", "highlight-wrap")
      ins.state.activeLine = undefined
    }
  }

  // 另存为的名字
  const onInputChange = (e) => {
    dispatch(updateCellMetadata({
      path,
      cellId, metadata: {
        dfName: e.target.value
      }
    }));
  }

  return (
    <Row className="code-cell">
      <Col className="sider-left-controlbar">
        <div
          className="editor-cell-statebar"
          onClick={() =>
            setHideInput(!hideInput)
          }
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
            cellProps={cellProps}
            path={path}
            cellId={cellId}
            index={index}
            stopCell={stopCell}
            runCurrentCellAndAbove={runCurrentCellAndAbove}
            runCurrentCellAndBelow={runCurrentCellAndBelow}
          />
        </Row>

        <Row
          className={
            hideInput
              ? "code-box code-box-hide"
              : "code-box"
          }
        >
          <div className="tool-placeholder">
            <div className="execution-time-text" style={{
              display: focus ? "" : "none",
            }}>
              {executionTime}
            </div>
          </div>
          <div className='tool-row' style={{marginTop: '0px'}} >
            <span className="sql-lable">{intl.get('NOTEBOOK_SQL_DATASOURCE')}</span>
            <div className='tool-col'>
              <DataSourceSelect
                path={path}
                key={cellId}
                cellId={cellId}
                placeholder={intl.get('NOTEBOOK_SQL_DATASOURCE_TIPS')}
                dataSource={metadata ? metadata.dataSource : ''}
              />
            </div>
            <span className="sql-lable">{intl.get('NOTEBOOK_SQL_VARIABLE')}</span>
            <div className='tool-col-saved'>
              <Input placeholder={intl.get('NOTEBOOK_SQL_VARIABLE_TIPS')}
                bordered={false}
                value={metadata ? metadata.dfName : ''}
                onChange={onInputChange}
                onBlur={() => {
                  dispatch(contentUpdateCellSource({ path, cellId }))
                }}
              />
            </div>
          </div>

          <CodeMirror
            key={`${path}/${cellId}`}
            value={value}
            editorDidMount={(instance, value) =>
              editorDidMount(
                cellId,
                instance,
                value
              )
            }
            onInputRead={(instance, change) => {
              if (change.text.toString() !== ' ') {
                instance.showHint()
              }
            }}
            onFocus={(ins) => {
              onFocus(cellId, ins)
            }}
            onBlur={(ins) => {
              onBlur(cellId, ins.getValue())
            }}
            onCursor={highlightcurrentLine}
            scrollbarStyle="null"
            options={{
              cellId: cellId,
              path: path,
              matchBrackets: true,
              autoCloseBrackets: true,
              theme: 'xq-light',
              mode: "text/x-mysql",
              keyMap: 'sublime',
              lineWrapping: true,
              lineNumbers: ToolImpl.lineNumbers,
              indentUnit: 4,  // 缩进的空格数
              addModeClass: true,
              autofocus: true,
              autoRefresh: true,
              runCell: () => {
                dispatch(updateCellSource({
                  path,
                  cellId: cellId,
                  source: formatSource(editorInstance.getValue()),
                }));
                setToogleValue((toogleValue) => !toogleValue)
              },
              runCellAndGotoNext: () => {
                dispatch(updateCellSource({
                  path,
                  cellId: cellId,
                  source: formatSource(editorInstance.getValue()),
                }));
                setSToogleValue(sToogleValue => !sToogleValue)
              },
              extraKeys,

              highlightSelectionMatches: {},
              hintOptions: {
                completeSingle: false,
                alignWithWord: true,
              },
            }}
          />

        </Row>
        <Row
          className="show-cell-btn"
          style={{
            display: hideInput
              ? "block"
              : "none",
          }}
          onClick={() =>
            setHideInput(!hideInput)
          }
        >{`{...}`}
        </Row>
      </Col>
    </Row>
  );
}
// export default React.memo(SqlCell);
