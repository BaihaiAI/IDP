import React, { useState, useEffect, useMemo, memo } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import intl from 'react-intl-universal';
import Ansi from "ansi-to-react";
import TextPlain from "./TextPlain";
import './outputCell.less'
import { Col, Row, message,Input} from 'antd';
import { store } from "../../../../store"
import { selectActivePath } from '../../../../store/features/filesTabSlice';
import sneltoets from '@idp/global/sneltoets';
import { observer } from 'mobx-react';

const OutputCell = (props) => {
    const { path, cellId, cellProp,sendInputRequest } = props;
    const [visible, setVisible] = useState(!sneltoets.collapseAllOutput);

    const [outputs, setOutputs] = useState(props.outputs);
    // const activePath = useSelector(selectActivePath);

    useEffect(() => {
      const notebookList = store.getState().notebook.notebookList
      for (const notebook of notebookList) {
        if (notebook.path !== path) continue
        for (const cell of notebook.cells) {
          if (cell.metadata.id === cellId) {
            setOutputs(cell.outputs)
          }
        }
      }
    }, [props.outputs])

    useEffect(() => {
        setVisible(!sneltoets.collapseAllOutput);
    }, [sneltoets.collapseAllOutput])


    const toggleVisible = () => {
        setVisible(!visible);
    };
    const errorLineClick = (errorInfo) => {
        const ins = cellProp['instance'];
        const line = errorInfo.errorLine;
        if (ins && line <= ins.display.view.length) {
            const dom = ins.display.view[line - 1]['text'];
            ins.focus();
            dom.scrollIntoView({ behavior: 'smooth', block: 'start' });
            ins.setCursor({ line: line - 1, ch: 100 });
            ins.doc.addLineClass(line, "background", "highlight-line")
            ins.doc.addLineClass(line, "wrap", "highlight-wrap")
            ins.doc.addLineClass(line, "gutter", "highlight-gutter")
            ins.state.activeLine = line
        } else {
            message.error(intl.get("ERROR_NOTIN_FILE"));
        }
    };

    // 将html文本拆成纯html代码和脚本代码
    const formatHtml = (html, scriptArr, htmlArr, hasScript) => {
        if (html) {
            const sIndex = html.indexOf('<script');
            const eIndex = html.indexOf('</script>');
            if (hasScript) {
                if (eIndex >= 0) {
                    const script = html.slice(0, eIndex + 9);
                    scriptArr.push(script);
                    return formatHtml(html.slice(eIndex + 9), scriptArr, htmlArr, false);
                } else {
                    scriptArr.push(html);
                    return formatHtml('', scriptArr, htmlArr, true);
                }
            } else {
                if (sIndex >= 0) {
                    const htmlBefore = html.slice(0, sIndex);
                    if (htmlBefore) htmlArr.push(htmlBefore);

                    const htmlAfter = html.slice(sIndex);
                    const nEIndex = htmlAfter.indexOf('</script>');
                    if (nEIndex >= 0) {
                        const script = htmlAfter.slice(0, nEIndex + 9);
                        scriptArr.push(script);
                        return formatHtml(htmlAfter.slice(nEIndex + 9), scriptArr, htmlArr, false);
                    } else {
                        scriptArr.push(htmlAfter);
                        return formatHtml('', scriptArr, htmlArr, true);
                    }
                } else {
                    htmlArr.push(html);
                    return formatHtml('', scriptArr, htmlArr, false);
                }
            }
        } else {
            return {
                scriptArr,
                htmlArr,
                hasScript,
            }
        }
    }

    // 合并脚本代码
    const mergeScript = (nextScriptArr, scriptArr) => {
        let scriptStr = '';
        for (const script of scriptArr) {
            scriptStr += script;
            if (script.indexOf('</script>') >= 0) {
                nextScriptArr.push(scriptStr);
                scriptStr = '';
            }
        }
        return nextScriptArr;
    }
    const regProgress = () => {
      /*
      "\r  1/938 [..............................] - ETA: 21s - loss: 0.0052 - accuracy: 1.0000\n"
      "\r  0%|          | 33792/170498071 [00:01<2:08:19, 22139.52it/s]\n"
      "\rDownloading:   0%|          | 0.00/263k [00:00<?, ?B/s]\n"
      */
      return new RegExp('^\r[ ]*(([0-9]+/[0-9]+)|([0-9]+%)|(Downloading.*[0-9]+%))')
    }

    // 判断进度条完成
    const isProgressFinish = (line) => {
      if (line.indexOf('\r\n') !== -1 || line.indexOf('\n\n') !== -1 || line.indexOf('100%') !== -1) return true;
      // "\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\b\r938/938 [==============================] - 3s 3ms/step - loss: 0.0601 - accuracy: 0.9815 - val_loss: 0.0586 - val_accuracy: 0.9800\n"
      if ((new RegExp('\r[0-9]+/[0-9]+')).test(line)) {
        const arr = line.match(/[0-9]+/g)
        return line.length >= 2 && arr[0] === arr[1]
      }
      return false
    }
    const parseOutputs = useMemo(() => {
        if (undefined === outputs || outputs.length === 0) {
            return "";
        }
        // Handle "data" type cells
        let stateValue = '';
        let textPlain = '';
        let textHtml = '';
        let stdout = '';
        let stdoutProgressHeader = '';
        let stdoutProgress = '';
        let isStdoutcomplete = true;
        let stdouts = [];
        let errors = '';
        let headerEnameErrors = '';
        let headerEvalueErrors = '';
        let imgList = [];
        let imgData = 'data:image/png;base64,';

        //booleans
        let stdoutFound = false
        let textPlainFound = false
        let textHtmlFound = false
        let errorFound = false
        let imgFound = false
        let inputRequestFound = false


      //maxlines for each output type
        let linesStdout = 3
        let linesTextPlain = 3
        let linesErrorTrace = 3
        let scripts = [];
        const ingressReg = regProgress()
      // console.log(outputs);
        for (const outs of outputs) {
            if ("data" in outs) {
                const outputType = outs['output_type'];
                const outData = outs["data"];
                if (outputType !== 'display_data' && "text/plain" in outData) {
                    let arr = outData['text/plain'];
                    for (const text of arr) {
                        textPlain += text;
                        textPlainFound = true;
                    }
                }
                if ("text/html" in outData) {
                    let arr = outData['text/html'];
                    let scriptArr = [];
                    let htmlArr = [];
                    let hasScript = false;
                    for (const text of arr) {
                        const scriptAndHtmlArr = formatHtml(text, scriptArr, htmlArr, hasScript);
                        scriptArr = [...scriptAndHtmlArr.scriptArr];
                        htmlArr = [...scriptAndHtmlArr.htmlArr];
                        hasScript = scriptAndHtmlArr.hasScript;
                    }
                    scripts = mergeScript(scripts, scriptArr);
                    for (const html of htmlArr) {
                        textHtml += html;
                    }
                    textHtmlFound = true;
                }
                if ("image/png" in outData) {
                    imgList.push(imgData + outData["image/png"]);
                    imgFound = true
                }
                if ('state' in outData) {
                    stateValue = outData['state']['value'];
                }
            }
            if ("name" in outs) {
                for (const text of outs["text"]) {
                    if (isStdoutcomplete) {
                        // 安装包时输出太多，做下合并
                        // if (text.startsWith('\u001b[?25l\r\u001b[K')) {
                        if (text.startsWith('\u001b[?25l')) {
                            stdoutProgressHeader = text;
                            isStdoutcomplete = false;
                        } else if (ingressReg.test(text)) {
                          stdoutProgress = text;
                          isStdoutcomplete = false;
                        } else {
                            stdout += text;
                        }
                    } else {
                        // 安装包时输出太多，做下合并
                        stdoutProgress = text;
                        if (isProgressFinish(text)) {
                            stdout = stdout + stdoutProgressHeader + stdoutProgress;
                            stdoutProgressHeader = '';
                            stdoutProgress = '';
                            isStdoutcomplete = true;
                        }
                    }
                }
                stdoutFound = true;
                linesStdout = outs["text"].length;
                if (isStdoutcomplete) {
                    stdouts.push(stdout);
                    stdout = '';
                }
            }
            // Check for errors
            if ("ename" in outs) {
                headerEnameErrors = outs['ename'];
                headerEvalueErrors = outs["evalue"];
                for (const trace of outs["traceback"]) {
                    errors += trace
                }
                errorFound = true
                linesErrorTrace = outs["traceback"].length
            }
            // input request
            if (outs['output_type'] && outs['output_type'] === 'input_request') {
              inputRequestFound = true
            }
        }

        stdout = stdout + stdoutProgressHeader + stdoutProgress;
        if (stdout !== '') {
            stdouts.push(stdout);
        }
        // 此处应该开发成一个模板组装器
        const returnTemplate = (
            <div>
              <div style={{ marginTop: "-1px", display: stdoutFound || inputRequestFound ? '' : 'none' }}>
                    {
                        stdouts.map((item, i) => (
                            <div key={i}>
                                <Ansi
                                    key={i}
                                    useClasses
                                    linkify={true}
                                    className={sneltoets.autoWarpOutput ? "ansi-warp-span" : "ansi-black-span"}
                                >{item}</Ansi>
                            </div>
                        ))
                    }
                    {inputRequestFound && <Input onPressEnter={(e) => {
                      sendInputRequest(cellId, e.target.value)
                      e.target.hidden = true
                      e.target.value = ''
                    }} />}
                </div>
                <div style={{ display: textPlainFound && !textHtmlFound ? '' : 'none' }}>
                    <pre className={sneltoets.autoWarpOutput ? "ansi-warp-span" : "ansi-black-span"}>
                        <span>{textPlain}</span>
                    </pre>
                </div>
                <div style={{ display: textHtmlFound ? '' : 'none' }}>
                    <TextPlain cellId={cellId} textPlain={textHtml} scripts={scripts} />
                    <div dangerouslySetInnerHTML={{ __html: `<div id="state-${cellId}">${stateValue}<div>` }}></div>
                </div>
                <div style={{ marginTop: "-1px", display: imgFound ? '' : 'none' }}>
                    {
                        imgList.map((img, i) => (
                            <img
                                key={i}
                                alt=""
                                src={img}
                                style={{
                                    maxWidth: '100%',
                                    display: imgFound ? '' : 'none',
                                    backgroundColor: 'white'
                                }} />
                        ))
                    }
                </div>
                {/* 出现三个Ansi是避免出现内容不匹配的问题，暂时只能这样解决，后期若若是有好的解决方案，就替换了 */}
                <div style={{ marginTop: "-1px", display: errorFound ? '' : 'none' }}>
                    <div style={{ color: "red", fontWeight: "bold" }}>Error:</div>
                    <span style={headerEnameErrors ? { marginRight: '15px' } : { display: 'none' }}>
                        <Ansi
                            key="headerEnameErrors"
                            className={sneltoets.autoWarpOutput ? "ansi-warp-span" : "ansi-black-span"}
                            linkify={true}
                        >
                            {headerEnameErrors + ":"}
                        </Ansi>
                    </span>
                    <span style={headerEvalueErrors ? {} : { display: 'none' }}>
                        <Ansi
                            key="headerEvalueErrors"
                            className={sneltoets.autoWarpOutput ? "ansi-warp-span" : "ansi-black-span"}
                            linkify={true}
                        >
                            {headerEvalueErrors}
                        </Ansi>
                        <br />
                    </span>
                    <Ansi
                        key="errors"
                        className={sneltoets.autoWarpOutput ? "ansi-warp-span" : "ansi-black-span"}
                        clickHandle={errorLineClick}
                        linkify={true}
                    >
                        {errors}
                    </Ansi>
                </div>
            </div>
        )
        return returnTemplate;
    }, [cellProp, outputs, cellId, sneltoets.autoWarpOutput]);

    return (
        <>
            <Row className="demo-cell output-cell"
                style={{
                    display: (outputs && outputs.length !== 0) ? '' : 'none'
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

                            <div style={{ padding: "10px", overflowX: 'scroll', maxHeight: 415, display: visible ? '' : 'none' }}>
                                {parseOutputs}
                            </div>
                        </Col>
                    </Row>
                </Col>
            </Row>
        </>
    );
}

export default memo(OutputCell);
