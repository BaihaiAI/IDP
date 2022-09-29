import React, { useContext, useImperativeHandle, useState } from "react"
import { Button, Form, Input, Select } from "antd"
import "./DataFrame.less"
import { useForm } from "antd/es/form/Form"
import { VisualizationCellContext } from "./VisualizationCell"
import { useDispatch } from "react-redux"
import { updateCellMetadata } from "../../../../store/features/notebookSlice"
import Ansi from "ansi-to-react"
import TextPlain from "../outputCell/TextPlain"
import intl from "react-intl-universal"
import { useDebounceFn } from "ahooks"
import ToolImpl from '@/idp/lib/tool/impl/toolImpl';

const picType = "pic_type"
const title = "title"
// const xCol = "x_col"
// const yCol = "y_col"
// const colorCol = "color_col"

function DataFrame(props, ref) {
  const { style } = props
  const [form] = useForm()
  const { columnSelectList, showTableTab,path } = props

  const { cellId, metadata, outputs, cellProp } = useContext(
    VisualizationCellContext
  )
  const dispatch = useDispatch()

  useImperativeHandle(ref, () => ({
    getFormInfo: () => form.getFieldsValue(),
    setFormValue: form.setFieldsValue,
  }))

  const { run } = useDebounceFn(
    () => {
      handleChange()
    },
    {
      wait: 1500,
    }
  )

  const handleChange = () => {
    const formInfo = form.getFieldsValue()
    const newMetaData = { ...metadata, chart: formInfo }
    console.log(newMetaData)
    dispatch(updateCellMetadata({ path,cellId, metadata: newMetaData }))
  }

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
  const parseOutputs = (outputs, cellId) => {
    if (undefined === outputs || outputs.length === 0) {
      return ""
    }

    let stateValue = '';
    let textPlain = '';
    let textHtml = '';
    let stdout = '';
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
          stdout += text;
        }
        stdoutFound = true;
        linesStdout = outs["text"].length;
        stdouts.push(stdout);
        stdout = '';
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
                  className={ToolImpl.autoWarpOutput ? "ansi-warp-span" : "ansi-black-span"}
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
          <pre className={ToolImpl.autoWarpOutput ? "ansi-warp-span" : "ansi-black-span"}>
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
              className={ToolImpl.autoWarpOutput ? "ansi-warp-span" : "ansi-black-span"}
              linkify={true}
            >
              {headerEnameErrors + ":"}
            </Ansi>
          </span>
          <span style={headerEvalueErrors ? {} : { display: 'none' }}>
            <Ansi
              key="headerEvalueErrors"
              className={ToolImpl.autoWarpOutput ? "ansi-warp-span" : "ansi-black-span"}
              linkify={true}
            >
              {headerEvalueErrors}
            </Ansi>
            <br />
          </span>
          <Ansi
            key="errors"
            className={ToolImpl.autoWarpOutput ? "ansi-warp-span" : "ansi-black-span"}
            linkify={true}
          >
            {errors}
          </Ansi>
        </div>
      </div>
    )
    return returnTemplate;
  }

  const dims = [
    { name: 'x', label: intl.get("DIM_X"), hidden: false },
    { name: 'y', label: intl.get("DIM_Y"), hidden: false },
    { name: 'color', label: intl.get("DIM_COLOR"), hidden: false },
    { name: 'size', label: intl.get("DIM_SIZE"), hidden: true },
    // { name: 'hover_data', label: intl.get("DIM_HOVER_DATA"), hidden: true },
    { name: 'facet_col', label: intl.get("DIM_FACET_COL"), hidden: true },
    { name: 'facet_row', label: intl.get("DIM_FACET_ROW"), hidden: true },
    { name: 'text', label: intl.get("DIM_TEXT"), hidden: true },
  ]
  const needHiddenDims = () => {
    const chart = metadata.chart
    if (chart && (chart.size
      || chart.hover_data
      || chart.facet_col
      || chart.facet_row
      || chart.text)) {
      return false
    } else {
      return true
    }
  }
  const [hiddenDimMore, setHiddenDimMore] = useState(needHiddenDims())
  const showDims = () => {
    setHiddenDimMore(false)
  }
  const hiddenDims = () => {
    setHiddenDimMore(true)
  }
  
  return (
    <div className={"dataframe-container"} style={style}>
      {!showTableTab ? (
        <div className={"left"}>
          <Form form={form} labelAlign={"right"} layout={"vertical"}>
            <Form.Item label={intl.get("GRAPHICS")} name={picType}>
              <Select onChange={handleChange} size={"small"}>
                {/*<Select.Option key={"bar"} value={"bar"}>条形图</Select.Option>*/}
                <Select.Option key={"line"} value={"line"}>
                  {intl.get("LINE_CHART")}
                </Select.Option>
                <Select.Option key={"area"} value={"area"}>
                  {intl.get("AREA_CHART")}
                </Select.Option>
                <Select.Option key={"point"} value={"point"}>
                  {intl.get("SCATTER_PLOT")}
                </Select.Option>
              </Select>
            </Form.Item>
            <Form.Item label={intl.get("FIGURE_TITLE")} name={title}>
              <Input onChange={run} size={"small"} />
            </Form.Item>

            {/* <Form.Item label={`${intl.get("DIMENSION")}(X轴)`} name={xCol}>
              <Select onChange={handleChange} size={"small"}>
                {columnSelectList.map((item) => {
                  return (
                    <Select.Option key={item} value={item}>
                      {item}
                    </Select.Option>
                  )
                })}
              </Select>
            </Form.Item>

            <Form.Item label={`${intl.get("DIMENSION")}(Y轴)`} name={yCol}>
              <Select onChange={handleChange} size={"small"}>
                {columnSelectList.map((item) => {
                  return (
                    <Select.Option key={item} value={item}>
                      {item}
                    </Select.Option>
                  )
                })}
              </Select>
            </Form.Item>
            <Form.Item label={`${intl.get("DIMENSION")}(${intl.get("DIM_COLOR")})`} name={colorCol}>
              <Select onChange={handleChange} size={"small"}>
                {columnSelectList.map((item) => {
                  return (
                    <Select.Option key={item} value={item}>
                      {item}
                    </Select.Option>
                  )
                })}
              </Select>
            </Form.Item> */}
            {
              dims.map((dim) => {
                return (
                  <Form.Item key={dim.name} label={`${intl.get('DIMENSION')}(${dim.label})`} name={dim.name} hidden={dim.hidden && hiddenDimMore}>
                    <Select onChange={handleChange} size={"small"}>
                      {dim.name != 'x' && dim.name != 'y' ? <Select.Option key="empty" value="">{intl.get('DIM_EMPTY')}</Select.Option> : null}
                      {columnSelectList.map((item) => {
                        return (
                          <Select.Option key={item} value={item}>
                            {item}
                          </Select.Option>
                        )
                      })}
                    </Select>
                  </Form.Item>
                )
              })
            }
          </Form>
          <Button
            size="small"
            type="link"
            hidden={!hiddenDimMore}
            onClick={showDims}
            style={{ paddingLeft: 0, marginBottom: 5, fontSize: 12 }}>
            {intl.get('DIM_MORE')}
          </Button>
        </div>
      ) : null}
      {cellProp.state === "ready" && Array.isArray(outputs) && outputs.length > 0 ? (
        <div className={"right"} >
          <div id="dataframe-graph" style={{ padding: "10px", overflowX: "scroll", maxHeight: 470 }}>
            {parseOutputs(outputs, cellId)}
          </div>
        </div>
      ) : null}
    </div>
  )
}

export default React.forwardRef(DataFrame)
