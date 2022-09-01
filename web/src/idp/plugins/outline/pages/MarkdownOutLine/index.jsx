import React, { useCallback, useMemo, useState } from "react"
import { Collapse, Layout, List } from "antd"
import intl from "react-intl-universal"
import { updateCellPropFocus } from "../../../../../store/features/notebookSlice"
import { selectActivePath } from "../../../../../store/features/filesTabSlice"
import { marked } from "marked"
import { useDispatch, useSelector } from "react-redux"
import "./index.less"
import { useActiveCells } from "../../../../../utils/hook/useActiveCellProps"
import PubSub from "pubsub-js"

const { Sider } = Layout
const { Panel } = Collapse


function MarkdownOutLine(props) {
    const [clickedMarkdownIndex, setClickedMarkdownIndex] = useState(-1)

    const path = useSelector(selectActivePath)
    const cells = useActiveCells(path)
    const dispatch = useDispatch()


    /*markdown面板相关逻辑 start*/
    const markDownList = useMemo(() => {
        setClickedMarkdownIndex(-1)
        return cells?.filter((item) => item.cell_type === "markdown") || []
    }, [cells])

    const handleClickMarkDownTitle = useCallback(({ item, index }) => {
        return () => {
            const cellId = item.itemObj.metadata.id;
            dispatch(updateCellPropFocus({ path, cellId }));
            PubSub.publish("updateNotebookScroll", cellId);
            setClickedMarkdownIndex(index);
        }
    }, []);

    const showMarkdownList = useMemo(() => {
        return markDownList.map((item) => {
            if (item.source.length === 0) return null
            const markdownValue = item.source.join("")
            const htmlText = marked.parse(markdownValue)

            if (htmlText.indexOf("<h") === -1) {
                return null
            }
            const titleText = htmlText.slice(
                htmlText.indexOf("<h"),
                htmlText.indexOf("</h")
            )
            const level = titleText[2] * 1

            const sliceText = titleText.slice(titleText.indexOf('id="') + 4)
            const showText = sliceText.slice(0, sliceText.indexOf('"'))
            if (showText.length) {
                return {
                    level,
                    showText,
                    itemObj: item,
                }
            } else {
                return null
            }
        }).filter((item) => item).map((item, index, array) => {
            item.indent = 0
            for (let i = index - 1; i >= 0; i--) {
                const itemElement = array[i]
                if (itemElement.level < item.level) {
                    item.indent = itemElement.indent + 1
                    break
                }
            }
            return item
        })
    }, [markDownList])
    return (
        <Sider
            theme="light"
            width="300"
            style={{ height: document.body.clientHeight - 40 }}
        >
            <div className={"markdown-outline-header"}>
                <span className={"left"}>{intl.get("MARK_DOWN_OUT_LINE")}</span>
            </div>
            <List
                size="small"
                style={{ height: document.body.clientHeight - 73, overflow: "auto" }}
                dataSource={showMarkdownList}
                renderItem={(item, index) => {
                    return (
                        <List.Item
                            style={{
                                backgroundColor:
                                    clickedMarkdownIndex === index ? "#f0f0f0" : "#fff",
                            }}
                            onClick={handleClickMarkDownTitle({ item, index })}
                        >
                            <List.Item.Meta
                                style={{
                                    whiteSpace: "nowrap",
                                    overflow: "hidden",
                                    textOverflow: "ellipsis",
                                    textIndent: `${item.indent}em`,
                                    cursor: "pointer",
                                }}
                                title={item.showText}
                            />
                        </List.Item>
                    )
                }}
            ></List>
        </Sider>
    )
}

export default MarkdownOutLine
