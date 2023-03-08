import React, { useEffect, useState } from "react"
import PubSub from "pubsub-js"
import "./DataSetShow.less"
import { Table, Tooltip } from "antd"
const JSONBig = require("json-bigint")
const BigNumber = require('bignumber.js');

function DataSetShow(props) {
  const [schemaListTable, setSchemaListTable] = useState([])
  const [schemaListColumns, setSchemaListColumns] = useState([])
  const [selectListTable, setSelectListTable] = useState([])
  const [selectListColumns, setSelectListColumns] = useState([])

  const [aliasDB, setAliasDB] = useState("")
  const [tableName, setTableName] = useState("")

  useEffect(() => {
    const subscriber = PubSub.subscribe(
      "getDataSetShowData",
      (msg, { aliasDB, tableName, selectList, selectListHeader, schemaList, schemaListHeader }) => {
        let newSelectListTable = []
        let newSelectListColumns = []
        let newSchemaListTable = []
        let newSchemaListColumns = []

        if (selectList.length > 0) {
          Object.keys(selectListHeader).forEach((key) => {
            newSelectListColumns[selectListHeader[key]] = {
              title: key,
              dataIndex: key,
              ellipsis: true,
              align: "center",
              render: (text) => {
                if (text instanceof Object) {
                  text = JSON.stringify(text)
                }
                return (
                  <Tooltip title={text}>
                    <span>{text}</span>
                  </Tooltip>
                )
              },
            }
          })
          newSelectListColumns[0] = {
            render: function (text, record, index) {
              return <span>{index + 1}</span>
            },
            align: "center",
            width: 100,
          }

          for (const item of selectList) {
            let newItem = {};
            Object.keys(item).forEach((key) => {
              if (key === 'task_superid') {
                console.log(typeof item[key])
                console.log(item[key])
              }
              if (typeof item[key] === "object") {
                if (item[key] instanceof BigNumber) {
                  newItem[key] = JSONBig.stringify(item[key])
                } else {
                  newItem[key] = item[key]
                    ? item[key]['value'] ? item[key]['value'] : JSON.stringify(item[key])
                    : item[key]
                }
              } else {
                newItem[key] = item[key]
              }
            })
            newSelectListTable.push(newItem)
          }
        }

        if (schemaList.length > 0) {
          Object.keys(schemaListHeader).forEach((key) => {
            newSchemaListColumns[schemaListHeader[key] - 1] = {
              title: key,
              dataIndex: key,
              align: "center",
              ellipsis: true,
              render: (text) => {
                if (text instanceof Object) {
                  text = JSON.stringify(text)
                }
                return (
                  <Tooltip title={text}>
                    <span>{text}</span>
                  </Tooltip>
                )
              },
            }
          })

          newSchemaListColumns[0] = {
            render: function (text, record, index) {
              return <span>{index + 1}</span>
            },
            align: "center",
            width: 100,
          }

          newSchemaListTable = schemaList
        }

        setSchemaListColumns(newSchemaListColumns)
        setSchemaListTable(newSchemaListTable)
        setSelectListColumns(newSelectListColumns)
        setSelectListTable(newSelectListTable)
        setAliasDB(aliasDB)
        setTableName(tableName)
      }
    )
    return () => {
      PubSub.unsubscribe(subscriber)
    }
  }, [])

  if (!aliasDB && !tableName) {
    return null
  }

  return (
    <div className={"data-set-show-container"}>
      <div className={"data-set-show-header"}>
        <div className={"table-name"}>table name</div>
        <span className={"path-text"}>
          {aliasDB}/{tableName}
        </span>
      </div>

      <div className={"schema-container"}>
        <div className={"header"}>schema</div>
        <div className={"table"}>
          <Table
            dataSource={schemaListTable}
            columns={schemaListColumns}
            pagination={false}
            rowClassName={"schema-data-row"}
            scroll={{ y: 160 }}
          />
        </div>
      </div>

      <div className={"select-container"}>
        <div className={"header"}>sample data</div>
        <div className={"table"}>
          <Table
            dataSource={selectListTable}
            columns={selectListColumns}
            pagination={false}
            rowClassName={"select-data-row"}
            scroll={{ x: selectListColumns.length * 10 + "%", y: 330 }}
          />
        </div>
      </div>
    </div>
  )
}

export default DataSetShow
