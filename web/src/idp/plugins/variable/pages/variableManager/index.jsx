import React, { useMemo, useState } from "react"
import { Button, Input, Layout, List, Tooltip } from "antd"
import { SearchOutlined } from "@ant-design/icons"
import { useDispatch, useSelector } from "react-redux"
import { contentAddCell, updateCellMetadata, variableListAsync } from "../../../../../store/features/notebookSlice"
import { selectActivePath } from "../../../../../store/features/filesTabSlice"
import { useThrottle } from "ahooks"
import Icons from "idpStudio/components/Icons/Icons"
import { useActiveCellProps, useActiveCells, useActiveVariableList, useNotebookItem } from "../../../../../utils/hook/useActiveCellProps";
import PubSub from "pubsub-js";
import intl from "react-intl-universal";

import "./index.less"

const { Sider } = Layout

function VariableManager(props) {
    const { showVariableManager } = props
    const dispatch = useDispatch()
    const path = useSelector(selectActivePath)
    const item = useNotebookItem(path)
    const cellProps = useActiveCellProps(path)
    const cells = useActiveCells(path)
    const variableList = useActiveVariableList(path)

    const [searchVariable, setSearchVariable] = useState("")
    const throttleSearchValue = useThrottle(searchVariable, { wait: 500 })
    const showVariableList = useMemo(() => {
        return variableList.filter((item) => {
            return item.name.indexOf(throttleSearchValue) !== -1
        })
    }, [throttleSearchValue, variableList])

    const handleClickSearchToAddCell = (dfName) => {
        const cellPropsKeyArr = Object.keys(cellProps)
        let focusKey
        for (let i = 0; i < cellPropsKeyArr.length; i++) {
            const key = cellPropsKeyArr[i]
            if (cellProps[key].focus) {
                focusKey = key
                break
            }
        }
        const newIndex = cells.findIndex((item) => item.metadata.id === focusKey) + 1;
        dispatch(contentAddCell({ path, index: newIndex === 0 ? 0 : newIndex, cellType: "visualization", cells })).unwrap().then(({ data }) => {
            const cell = data
            const cellId = cell.metadata.id
            PubSub.publish("updateNotebookScroll", cellId)
            let newArr = []
            const findResult = variableList.find((item) => item.name === dfName)
            if (findResult) {
                newArr = JSON.parse(findResult.meta).columns
            }
            const defaultFormObj = {
                x_col: newArr[0],
                y_col: newArr[0],
                color_col: newArr[0],
                pic_type: "line",
            }
            dispatch(
                updateCellMetadata({
                    path,
                    cellId: cell.metadata.id,
                    metadata: {
                        df_name: dfName,
                        show_table: "true",
                        ...defaultFormObj,
                    },
                })
            )
        })
    }

    return (
        <div className={"variable-manager-container"}>
            <Sider
                collapsed={!showVariableManager}
                width={300}
                collapsedWidth={0}
                trigger={null}
                collapsible
            >
                <div className={"variable-manager-header"}>
                    <span className={"left"}>{intl.get("NOTEBOOK_VARIABLE_MANAGER")}</span>
                    <Button
                        className={"right"}
                        type={"text"}
                        icon={<Icons.BHRefreshIcon />}
                        size="small"
                        onClick={() => {
                            const { inode } = item.metadata
                            dispatch(variableListAsync({ path, inode }))
                        }}
                    />
                </div>
                <div className={"search-input"}>
                    <Input
                        value={searchVariable}
                        allowClear
                        style={{ paddingRight: "10px" }}
                        placeholder={intl.get("SEARCH_VARIABLE_NAME")}
                        onChange={(event) => {
                            setSearchVariable(event.target.value)
                        }}
                        suffix={<SearchOutlined style={{ transform: "translateX(5px)" }} />}
                    />
                </div>
                <div className={"variable-list"}>
                    <List
                        size="small"
                        dataSource={showVariableList}
                        renderItem={(item, i) => {
                            let tooltipTitle = `${item.name} (${item.type}): ${item.value instanceof Object
                                ? JSON.stringify(item.value)
                                : item.value
                                }  `
                            if (tooltipTitle.length > 700) {
                                tooltipTitle = tooltipTitle.slice(0, 700) + "     ......"
                            }

                            return (
                                <List.Item>
                                    <List.Item.Meta
                                        title={
                                            <div className={"variable-title-container"}>
                                                <span className={"variable-text"}>
                                                    <span
                                                        style={{ fontWeight: "bold", textIndent: "2em" }}
                                                    >
                                                        {item.name}{" "}
                                                        <span
                                                            style={{ color: "#8d99a4", fontWeight: "normal" }}
                                                        >
                                                            ({item.type}):
                                                        </span>
                                                    </span>
                                                    <Tooltip placement={"leftTop"} title={tooltipTitle}>
                                                        <span style={{ color: "#333" }}>
                                                            {item.value instanceof Object
                                                                ? JSON.stringify(item.value)
                                                                : item.value}
                                                        </span>
                                                    </Tooltip>
                                                </span>
                                                {item.type === "dataframe" ? (
                                                    <span
                                                        onClick={() => {
                                                            handleClickSearchToAddCell(item.name)
                                                        }}
                                                        className={"search-icon"}
                                                    >
                                                        <SearchOutlined />
                                                    </span>
                                                ) : null}
                                            </div>
                                        }
                                    />
                                </List.Item>
                            )
                        }}
                    />
                </div>
            </Sider>
        </div>
    )
}

export default VariableManager
