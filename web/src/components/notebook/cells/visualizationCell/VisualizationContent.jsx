import React, { useContext, useEffect, useMemo, useRef, useState } from "react"
import {Button, Input, message, Modal, Select} from "antd"
import DataFrame from "./DataFrame"
import "./VisualizationContent.less"
import { useDispatch } from "react-redux"
import {
  contentUpdateCellSource,
  updateCellMetadata,
} from "../../../../store/features/notebookSlice"
import { useSetState, useUpdateEffect } from "ahooks"
import { VisualizationCellContext } from "./VisualizationCell"
import variableManagerApi from "../../../../services/variableManagerApi"
import { projectId } from "../../../../store/cookie"
import { NotebookComponentContext } from "../../Notebook"
import intl from "react-intl-universal"

const handleColumns = (columns) => {
  let arr = []
  columns.forEach((item) => {
    let newItem
    if (typeof item === "number") {
      newItem = String(item)
    } else {
      newItem = item
    }
    arr.push(newItem)
  })
  return arr
}

const  VisualizationContent = (props) => {
  const { cellId, metadata, runCell, path } = useContext(
    VisualizationCellContext
  )
  const dispatch = useDispatch()
  const { show_table } = metadata
  const showTableTab = useMemo(() => !!show_table, [show_table])
  const { variableList } = useContext(NotebookComponentContext)
  const dataFrameVariableList = useMemo(() => {
    return variableList.filter((item) => item.type === "dataframe")
  }, [variableList])
  const countRef = useRef(0)

  const [metaDataInfo, setMetaDataInfo] = useSetState({
    df_name: metadata.df_name || "",
  })
  const [columnSelectList, setColumnSelectList] = useState([])

  const dataFrameRef = useRef()
  const selectParentRef = useRef()

  const toggleTab = () => {
    if (showTableTab) {
      const newMetaData = { ...metadata, show_table: "" }
      dispatch(updateCellMetadata({ path, cellId, metadata: newMetaData }))
    } else {
      const newMetaData = { ...metadata, show_table: "true" }
      dispatch(updateCellMetadata({ path, cellId, metadata: newMetaData }))
    }
  }

  /*  useUpdateEffect(() => {
    runCell(cellId)
  }, [showTableTab])*/

  useUpdateEffect(() => {
    if (metadata.df_name) {
      runCell(cellId)
      // 当runCell时 保存对应的数据
      dispatch(contentUpdateCellSource({ path, cellId }))
    }
  }, [
    metadata.df_name,
    metadata.x_col,
    metadata.y_col,
    metadata.color_col,
    metadata.pic_type,
    metadata.title,
    metadata.show_table
  ])

  useEffect(() => {
    const { df_name, x_col, y_col, color_col, pic_type, title } = metadata
    // console.log(df_name, x_col, y_col, color_col, pic_type, title ,'----------')
    setMetaDataInfo({
      df_name: df_name || "",
    })
    dataFrameRef.current.setFormValue({
      x_col: x_col || "",
      y_col: y_col || "",
      color_col: color_col || "",
      pic_type: pic_type || "",
      title: title || "",
    })

    const findResult = dataFrameVariableList.find(
      (item) => item.name === df_name
    )
    if (findResult) {
      const newArr = handleColumns(JSON.parse(findResult.meta).columns)
      setColumnSelectList(newArr)
    }
  }, [dataFrameVariableList, metadata])

  const dataFrameNameChange = (value) => {
    let newArr = []
    const findResult = dataFrameVariableList.find((item) => item.name === value)
    if (findResult) {
      newArr = handleColumns(JSON.parse(findResult.meta).columns)
      setColumnSelectList(newArr)
    } else {
      setColumnSelectList([])
    }

    setMetaDataInfo({
      df_name: value || "",
    })
    const defaultFormObj = {}

    if (value) {
      switch (newArr.length) {
        /*        case 1:
                  {
                    defaultFormObj.x_col = newArr[0]
                    defaultFormObj.y_col = ""
                    defaultFormObj.color_col = ""
                  }
                  break
                case 2:
                  {
                    defaultFormObj.x_col = newArr[0]
                    defaultFormObj.y_col = newArr[1]
                    defaultFormObj.color_col = ""
                  }
                  break
                case 3:
                  {
                    defaultFormObj.x_col = newArr[0]
                    defaultFormObj.y_col = newArr[1]
                    defaultFormObj.color_col = newArr[2]
                  }
                  break*/
        default:
        {
          defaultFormObj.x_col = newArr[0]
          defaultFormObj.y_col = newArr[0]
          defaultFormObj.color_col = newArr[0]
        }
      }

      defaultFormObj.pic_type = "line"
    } else {
      defaultFormObj.x_col = ""
      defaultFormObj.y_col = ""
      defaultFormObj.color_col = ""
      defaultFormObj.pic_type = ""
    }

    dataFrameRef.current.setFormValue({
      ...defaultFormObj,
    })

    const newMetaData = {
      ...metadata,
      ...defaultFormObj,
      df_name: value || "",
    }

    dispatch(updateCellMetadata({ path, cellId, metadata: newMetaData }))
  }

  const handleShare = () => {
    variableManagerApi
      .share({
        path,
        cellId,
      })
      .then((res) => {
        const shareId = res.data
        const url = `${window.location.origin}?shareId=${shareId}`
        // const url = `${window.location.origin}?projectId=1&shareId=${shareId}`
        // const url = `localhost:3000?projectId=1&shareId=${shareId}`
        Modal.info({
          title: intl.get("COPY_LINK"),
          content: (
            <Input.TextArea
              readOnly
              className={'share-text-area-input'}
              autoSize={{
                minRows: 3,
                maxRows: 6,
              }}
              defaultValue={url}
              onFocus={(event) => {
                event.target.select()
              }}
            />
          ),
          onOk:()=>{
            const inputNode = document.querySelector('.share-text-area-input')
            inputNode.select()
            document.execCommand("copy")
            message.success(intl.get("COPY_SUCCESSFULLY"),1.5)
          },
          okText:intl.get("COPY")
        })
      }).catch(err => {
        console.log(err);
        if (51000000 === err.code) {
          message.warning(intl.get("SHARE_ERROR_1"), 1.5)
        }
      })
  }


  return (
    <div className={"visualization-container"}>
      <div className={"header"}>
        <div className={"tabList"}>
          <div className={"tab"}>
            {/*<span>Visualization of dataframe</span>*/}
            <span>{intl.get("DATA_VISUALIZATION")}</span>
          </div>
          <div ref={selectParentRef} className={"tab"}>
            {/*<span>variable :</span>*/}
            <span>{intl.get("VARIABLE")} :</span>
            <Select
              getPopupContainer={()=>selectParentRef.current}
              value={metaDataInfo.df_name}
              onChange={dataFrameNameChange}
              allowClear
              style={{ minWidth: "100px" }}
              bordered={false}
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
        <div className={"toggle-container"}>
          <span onClick={toggleTab}>
            {!showTableTab ? intl.get("TOGGLE_FORM") : intl.get("TOGGLE_CHART")}
          </span>
          <span onClick={handleShare} style={{ marginLeft: 15 }}>
            {intl.get("SHARE")}
          </span>
        </div>
      </div>

      <div className={"visualization-content"}>
        <DataFrame
          path={path}
          ref={dataFrameRef}
          columnSelectList={columnSelectList}
          showTableTab={showTableTab}
        />
      </div>
    </div>
  )
}

export default VisualizationContent
