import React, { useState, useEffect, useRef, useContext } from 'react'
import { useSelector, useDispatch } from 'react-redux';
import { useUpdateEffect } from 'ahooks';
import intl from "react-intl-universal"
import cookie from "react-cookies";
import { UnControlled as CodeMirror } from 'react-codemirror2';
import PubSub from "pubsub-js"
import contentApi from '../../../../services/contentApi';

import { Row, Col, Typography } from 'antd';

import {
  contentAddCell,
  updateCellProp,
  updateCellSource
} from '../../../../store/features/notebookSlice';

import codemirror from "codemirror";
import "codemirror/lib/codemirror";
import "codemirror/keymap/sublime";
import "codemirror/theme/xq-light.css";
import "codemirror/addon/hint/show-hint";
import "codemirror/mode/python/python";
import "codemirror/addon/lint/lint.css";
import "codemirror/addon/lint/quickfix";
import "codemirror/addon/comment/comment";
import "codemirror/addon/edit/closebrackets";

import { extraKeys } from '../../lib/extraKeys';
import RightTopBar from '../cellTools/RightTopBar';
import sneltoets from '@idp/global/sneltoets';
import packageApi from '../../../../services/packageApi';
import './CodeCell.less'
import { useNotebookItem } from '../../../../utils/hook/useActiveCellProps';
// import { NotebookComponentContext } from "../../Notebook"
import { addNewFile } from '../../../../store/features/filesTabSlice';

// 为了自动安装包和恢复版本时更新内容
let refreshSource = false;
export const setRefreshSource = (flag) => {
  refreshSource = flag
}
export const CodeCell = (props) => {
  const {
    path,
    cellId,
    index,
    source,
    executionTime,
    focus,
    lspStore,
    lspWebsocket,
    runCell,
    doRunCell,
    runCellAndGotoNext,
    stopCell,
    onFocus,
    onBlur,
    formatSource,
    resetCellPosition,
    handleScroll,
    findFocusCellIdAndType,
    cellProps,
    runCurrentCellAndAbove,
    runCurrentCellAndBelow,
    content,
    // cells,
    onAddCell,
    outputs
  } = props;
  const dispatch = useDispatch();
  // const { cells } = useContext(NotebookComponentContext)
  const notebook = useNotebookItem(path)
  const cells = notebook ? notebook.cells : []

  const [value, setValue] = useState(source);
  const [hideInput, setHideInput] = useState(false);
  const [editorInstance, setEditorInstance] = useState();
  const [toogleValue, setToogleValue] = useState(false);
  const [sToogleValue, setSToogleValue] = useState(false);

  useEffect(() => {
    const subscriber = PubSub.subscribe('changeCompletionItemDom', (msg, data) => {
      const dom = document.getElementById(`cm-complete-0-${data.index}`)
      if (dom && !dom.className.includes('has-item-info')) {
        let showValue
        if (data.sliceValue.length > 100) {
          showValue = data.sliceValue.slice(0, 100) + '...'
        } else {
          showValue = data.sliceValue
        }
        console.log(showValue, 'hfggf')
        dom.className += ' has-item-info'
        dom.innerHTML += `<span class="item-info">${showValue}</span>`
      }
    })
    return () => {
      PubSub.unsubscribe(subscriber)
    }
  }, [])

  useEffect(() => {
    setHideInput(sneltoets.collapseAllInput);
  }, [sneltoets.collapseAllInput]);

  useEffect(() => {
    // 如果是自动安装缺失包，自动刷新
    if (refreshSource) {
      refreshSource = false;
      setValue(source);
    }
  }, [source])

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
          // console.log(cellProps[key].posLine)
          instance.focus();
          if (cellProps[key].posLine) {
            instance.setCursor({ line: cellProps[key].posLine - 1, ch: 0 })
          }
        }
        break;
      }
    }
  }

  let notebookItem = useNotebookItem(path)
  const editorChange = (editor, data, value, cellId) => {
    // !打头的pip忽略
    if (value.startsWith('!')) {
      return;
    }
    if (lspWebsocket.didOpenFile[path]) {
      // if (!data.origin) return; // 注释掉这行代码，因为自动修复后仍提示错误；之前加这个判断有可能是因为当全屏切换多个屏幕时，清空了所有内容。
      let text = data.text[0];
      for (let i = 1; i < data.text.length; i++) {
        text = `${text}\n${data.text[i]}`;
      }
      lspWebsocket.didChange(path, cellId, data, text);
    } else {  // 如果没有打开文件，则先发送didOpen消息
      let cells = []
      for (let cell of notebookItem.cells) {
        let newCell = { ...cell }
        if (cell.metadata.id === cellId) {
          newCell["source"] = formatSource(value);
        }
        cells.push(newCell)
      }
      const content = JSON.stringify({ ...notebookItem.notebookJson, cells })
      lspWebsocket.didOpen(path, content, lspStore)
    }
  }

  let isCompleted = false;  // 是否完成补全

  const getKeywordItem = (info, index) => {
    lspWebsocket.completionItemRequest(path, info,index)
  }


  const editorInputRead = (instance, cellId) => {
    // !打头的pip忽略
    const {line} = instance.getCursor();
    const currentLineValue = instance.getLine(line);
    if (currentLineValue.startsWith('!')) return;

    isCompleted = false;
    let cursor = instance.getCursor()

    // 往lspserver发completion message id,uri,line,character
    lspWebsocket.completionRequest(
      path,
      cellId,
      cursor.line,
      cursor.ch
    )

    lspWebsocket.completeHint.showHint = (messageId) => {
      !isCompleted && messageId === lspWebsocket.completeHint.messageId &&
        instance.showHint()
      isCompleted = true
    }

    setTimeout(() => {
      if (!isCompleted) {
        instance.showHint()
        // isCompleted = true
      }
    }, 500)
  }

  // python的hint
  const pythonKeywords = ["as", "assert", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally",
    "for", "from", "global", "if", "import",
    "lambda", "pass", "raise", "return",
    "try", "while", "with", "yield", "in",
    "abs", "all", "any", "bin", "bool", "bytearray", "callable", "chr",
    "classmethod", "compile", "complex", "delattr", "dict", "dir", "divmod",
    "enumerate", "eval", "filter", "float", "format", "frozenset",
    "getattr", "globals", "hasattr", "hash", "help", "hex", "id",
    "input", "int", "isinstance", "issubclass", "iter", "len",
    "list", "locals", "map", "max", "memoryview", "min", "next",
    "object", "oct", "open", "ord", "pow", "property", "range",
    "repr", "reversed", "round", "set", "setattr", "slice",
    "sorted", "staticmethod", "str", "sum", "super", "tuple",
    "type", "vars", "zip", "__import__", "NotImplemented",
    "Ellipsis", "__debug__"
  ]
  const handleShowHint = (cmInstance, hintOptions) => {
    let cursor = cmInstance.getCursor()
    let cursorLine = cmInstance.getLine(cursor.line)
    let end = cursor.ch
    let start = end

    let token = cmInstance.getTokenAt(cursor)
    // todo
    console.log(token, 'eeeee')
    if ("" === token.string) return

    let list = []
    const kindValueList = []
    const originDataList = []
    for (let item of lspStore.lspKeywords.values()) {
      if (token.string === ".") {
        //when dot been typed , show the all hint
        list.push(item.label)
        kindValueList.push(item.kind)
        originDataList.push(item)
      } else {
        if (item.label.startsWith(token.string)) {
          list.push(item.label)
          kindValueList.push(item.kind)
          originDataList.push(item)
        }
      }
    }

    if ((token.type === 'property' || token.string === '.') && list.length === 0) {
      return
    }

    // 如果lsp没有提供hint，则使用python的hint
    if (list.length === 0) {
      list = pythonKeywords.filter(item => item.startsWith(token.string))
      pythonKeywords.forEach(item => {
        kindValueList.push(14)
      })
    }

    lspStore.lspKeywords.clear()
    if (token.string === ".") {
      return {
        originDataList,
        keyword: token.string,
        kindValueList,
        list,
        from: { ch: token.start + 1, line: cursor.line },
        to: { ch: token.end, line: cursor.line },
      }
    } else {
      return {
        originDataList,
        keyword: token.string,
        kindValueList,
        list,
        from: { ch: token.start, line: cursor.line },
        to: { ch: token.end, line: cursor.line },
      }
    }
  }

  const remoteValidator = (text, updateLinting, options, cm, codemirror) => {
    let founds = {}
    if (lspStore.lspDiagnostics.size) {
      //初始 无值
      //判断是否为当前实例
      //if(this.ins[this.state.currentCellId]  != cm)  return;
      //判断数据是否更新
      lspStore.lspDiagnostics.forEach(function (item) {
        if (!founds[item.editorId]) founds[item.editorId] = []
        if (item.editorId == cm.options.cellId) {
          founds[item.editorId].push({
            from: codemirror.Pos(
              item.range.start.line,
              item.range.start.character
            ),
            to: codemirror.Pos(item.range.end.line, item.range.end.character),
            message: item.message, //added
            severity: item.severity == 1 ? "error" : "warning",
            data: item,
            // "error", "warning"
          })
        }
      })
      founds[cm.options.cellId] && updateLinting(founds[cm.options.cellId])
    }
  }

  // 自动安装缺失包
  const quickfixInstallPackage = (packageName, version) => {
    const oldFocusCell = findFocusCellIdAndType()
    dispatch(contentAddCell({
      path,
      index: index === 0 ? 0 : index,
      cellType: 'code',
      cells,
    })).unwrap().then((res) => {
      const { data } = res
      refreshSource = true;
      const cell = { ...data };
      cell['source'] = 'latest' === version ? [`!pip install ${packageName}`] : [`!pip install ${packageName}==${version}`];
      dispatch(updateCellSource({
        path,
        cellId: cell.metadata.id,
        source: cell['source'],
      }));
      doRunCell(cell, false);

      const focusCell = findFocusCellIdAndType()
      resetCellPosition(oldFocusCell.focusCellId)
      handleScroll(focusCell.focusCellId, focusCell.focusCellType)

    });
  }

  // 自动判断包名
  const quickfixGetPackageName = (packageName) => {
    const nameMap = {
      'cv2': 'opencv-python',
      'PIL': 'Pillow',
      'sklearn': 'scikit-learn',
      'psycopg2': 'psycopg2-binary',
      'pycrfsuite': 'python-crfsuite',
      'bayes_opt': 'bayesian-optimization',
    }
    return nameMap[packageName] || packageName
  }

  // 提取包名
  const quickfixExtractPackageName = (message) => {
    let packageName = message.slice(message.indexOf('"') + 1, message.lastIndexOf('"'));
    if (packageName.indexOf('.') > 0) {
      packageName = packageName.substring(0, packageName.indexOf('.'))
    }
    return quickfixGetPackageName(packageName)
  }

  // 搜索包版本
  const quickfixSearchPackage = (packageName, callback) => {
    packageApi.searchV3({ packageName }).then((res) => {
      if (res && res.data.records.length > 0) {
        callback(res.data.records[0], 'install');
      } else {
        callback({ packageName: packageName, versions: ['latest'] }, 'install');
      }
    }).catch((err) => {
      console.log(err)
      callback({ packageName: packageName, versions: ['latest'] }, 'install');
    })
  }

  const quickfixCallback = (cm, content, pos, callback) => {
    // 判断是不是缺包
    if (Array.isArray(pos)) {
      for (let i = 0; i < pos.length; i++) {
        const item = pos[i]
        if (item.data.code === 'reportMissingModuleSource' || item.data.code === 'reportMissingImports') {
          const packageName = quickfixExtractPackageName(item.data.message)
          quickfixSearchPackage(packageName, callback)
          return
        }
      }
    } else if (!Array.isArray(pos) && (pos.data.code === 'reportMissingModuleSource' || pos.data.code === 'reportMissingImports')) {
      const packageName = quickfixExtractPackageName(pos.data.message)
      quickfixSearchPackage(packageName, callback)
      return;
    }

    let timer = null
    let uri = lspWebsocket.path2uri(cm.options.path)
    let id = lspWebsocket.getVdoc(uri).nextQueryId()
    let timeout = null
    cookie.save("quickfixId", id) //用于lsp处理空态
    lspWebsocket.codeActionRequest(
      lspWebsocket.getVdoc(uri),
      id,
      uri,
      Array.isArray(pos) ? pos.map(item => item.data) : pos["data"]
    )

    timer = setInterval(function () {
      if (
        lspStore.lspQuickFixWords &&
        lspStore.lspQuickFixWords.result.length
      ) {
        callback(lspStore.lspQuickFixWords, 'code')
        clearInterval(timer)
        clearTimeout(timeout)
      }
    }, 500)
    timeout = setTimeout(function () {
      if (timer) clearInterval(timer)
      callback("&nbsp No quick fixes available!", 'code')
    }, 2000)
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
          <CodeMirror
            style={{ paddingTop: '15px' }}
            key={`${path}/${cellId}`}
            value={value}
            editorDidMount={(instance, value) =>
              editorDidMount(
                cellId,
                instance,
                value
              )
            }
            onFocus={(ins) => {
              console.log('foucus')
              onFocus(cellId, ins)
            }}
            onBlur={(ins) => {
              console.log('blur')
              onBlur(cellId, ins.getValue())
            }}
            onChange={(editor, data, value) =>
              editorChange(
                editor,
                data,
                value,
                cellId
              )
            }
            onInputRead={(instance) =>
              editorInputRead(instance, cellId)
            }
            onCursor={highlightcurrentLine}
            scrollbarStyle="null"
            options={{
              cellId: cellId,
              path: path,
              matchBrackets: true,
              autoCloseBrackets: true,
              theme: 'xq-light',
              mode: "python",
              keyMap: 'sublime',
              lineWrapping: true,
              lineNumbers: sneltoets.lineNumbers,
              indentUnit: 4,  // 缩进的空格数
              addModeClass: true,
              autofocus: true,
              quickfix: {
                getAnnotations: remoteValidator,
                quickfix_callback: quickfixCallback,
                installPackage: quickfixInstallPackage,
                async: true,
                lintOnChange: true,
              },
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
              saveVersion: () => {
                console.log('11111')
                const params = {
                  path,
                  label: intl.get("SAVE_VERSION_AUTO"),
                }
                contentApi.snapshot(params).catch((error) => { })
              },
              codeMirrorBlur: (ins) => { },
              currentCellBottomAddNewCell: () => {
                cells.forEach((prop, i) => {
                  if (cellProps[prop.metadata.id].focus) {
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
              extraKeys,
              getKeywordItem,
              highlightSelectionMatches: {},
              hintOptions: {
                completeSingle: false,
                alignWithWord: true,
                hint: handleShowHint,
              },
              gotoLibrary: (item) => {
                const path = item.uri
                dispatch(addNewFile({
                  path,
                  name: path.substring(path.lastIndexOf('/') + 1),
                  suffix: path.substring(path.lastIndexOf('.') + 1),
                  posLine: item.range.start.line,
                  posCh: item.range.start.character,
                }));
              }
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
// export default React.memo(CodeCell);
