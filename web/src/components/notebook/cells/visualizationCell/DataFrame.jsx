import React, { useContext, useImperativeHandle } from "react"
import { Form, Input, Select } from "antd"
import "./DataFrame.less"
import { useForm } from "antd/es/form/Form"
import { VisualizationCellContext } from "./VisualizationCell"
import { useDispatch } from "react-redux"
import { updateCellMetadata } from "../../../../store/features/notebookSlice"
import Ansi from "ansi-to-react"
import TextPlain from "../outputCell/TextPlain"
import intl from "react-intl-universal"
import { useDebounceFn } from "ahooks"

const picType = "pic_type"
const title = "title"
const xCol = "x_col"
const yCol = "y_col"
const colorCol = "color_col"

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
    const newMetaData = { ...metadata, ...formInfo }
    dispatch(updateCellMetadata({ path,cellId, metadata: newMetaData }))
  }
  const parseOutputs = (outputs, cellId) => {
    if (undefined === outputs || outputs.length === 0) {
      return ""
    }
    // Handle "data" type cells
    let textPlain = ""
    let stdout = ""
    let stdoutProgressHeader = ""
    let stdoutProgress = ""
    let isStdoutcomplete = true
    let stdouts = []
    let errors = ""
    let imgList = []
    let imgData = "data:image/png;base64,"

    //booleans
    let stdoutFound = false
    let textFound = false
    let errorFound = false
    let imgFound = false

    //maxlines for each output type
    let linesStdout = 3
    let linesTextPlain = 3
    let linesErrorTrace = 3
    let scripts = []
    // console.log(outputs);
    for (const outs of outputs) {
      if ("data" in outs) {
        const outData = outs["data"]
        if ("text/plain" in outData || "text/html" in outData) {
          let arr = outData["text/html"]
          if (!arr) {
            arr = outData["text/plain"]
          }
          let scriptFlag = false
          for (const text of arr) {
            if (text.indexOf("<script") >= 0) {
              if (text.indexOf("</script>") < 0) {
                scriptFlag = true
              }
              scripts.push(text)
              continue
            } else if (text.indexOf("</script>") >= 0) {
              scripts.push(text)
              scriptFlag = false
              continue
            }
            if (scriptFlag) {
              if (text.indexOf("buildPlotFromProcessedSpecs") !== -1) {
                scripts.push(`plotSpec["ggsize"] = { "width": document.getElementById("dataframe-graph").offsetWidth - 20, "height":400 };`)
              }
              scripts.push(text)
            } else {
              textPlain += text
            }
          }
          textFound = true
          linesTextPlain = arr.length
        }
        if ("image/png" in outData) {
          imgList.push(imgData + outData["image/png"])
          imgFound = true
        }
      }
      if ("name" in outs) {
        for (const text of outs["text"]) {
          if (isStdoutcomplete) {
            // 安装包时输出太多，做下合并
            if (text.startsWith("\u001b[?25l\r\u001b[K")) {
              stdoutProgressHeader = text
              isStdoutcomplete = false
            } else {
              stdout += text
            }
          } else {
            // 安装包时输出太多，做下合并
            stdoutProgress = text
            if (text.indexOf("\r\n") !== -1) {
              stdout = stdout + stdoutProgressHeader + stdoutProgress
              stdoutProgressHeader = ""
              stdoutProgress = ""
              isStdoutcomplete = true
            }
          }
        }
        stdoutFound = true
        linesStdout = outs["text"].length
        if (isStdoutcomplete) {
          stdouts.push(stdout)
          stdout = ""
        }
      }
      // Check for errors
      if ("ename" in outs) {
        errors += outs["ename"] + "\n" + outs["evalue"] + "\n"
        for (const trace of outs["traceback"]) {
          errors += trace
        }
        errorFound = true
        linesErrorTrace = outs["traceback"].length
      }
    }

    stdout = stdout + stdoutProgressHeader + stdoutProgress
    if (stdout !== "") {
      stdouts.push(stdout)
    }

    const returnTemplate = (
      <div>
        <div
          style={{
            marginTop: "-1px",
            display: stdoutFound ? "" : "none",
          }}
        >
          {stdouts.map((item, i) => (
            <div key={i}>
              <Ansi key={i} useClasses className="ansi-black">
                {item}
              </Ansi>
            </div>
          ))}
        </div>
        <div
          className={"test"}
          style={{ width: "100%", display: textFound ? "" : "none" }}
        >
          <TextPlain cellId={cellId} textPlain={textPlain} scripts={scripts} />
        </div>
        <div
          style={{
            marginTop: "-1px",
            display: imgFound ? "" : "none",
          }}
        >
          {imgList.map((img, i) => (
            <img
              key={i}
              alt=""
              src={img}
              style={{
                display: imgFound ? "" : "none",
                backgroundColor: "white",
              }}
            />
          ))}
        </div>
        <div
          style={{
            marginTop: "-1px",
            display: errorFound ? "" : "none",
          }}
        >
          <div style={{ color: "red", fontWeight: "bold" }}>Error:</div>
          <Ansi>{errors}</Ansi>
        </div>
      </div>
    )
    return returnTemplate
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

            <Form.Item label={`${intl.get("DIMENSION")}(X轴)`} name={xCol}>
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
            <Form.Item label={`${intl.get("DIMENSION")}(${intl.get("COLOR")})`} name={colorCol}>
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
          </Form>
        </div>
      ) : null}
      {cellProp.state === "ready" && Array.isArray(outputs) && outputs.length > 0 ? (
        <div className={"right"} >
          <div id="dataframe-graph" style={{ padding: "10px", overflowX: "scroll", maxHeight: 415 }}>
            {parseOutputs(outputs, cellId)}
          </div>
        </div>
      ) : null}
    </div>
  )
}

export default React.forwardRef(DataFrame)
