import { useEffect, useMemo,Fragment, memo } from "react"
import { Tooltip, Typography, Button } from "antd"
import Icons from "../../../Icons/Icons"
import "./cellTools.less"
import intl from "react-intl-universal"
import { useRef, useState } from "react"
import {CheckOutlined} from "@ant-design/icons"

// 不同系统显示不同运行方式
const winOrMac = () => {
    // 去掉了判断window的逻辑， mac是 ⌘ 其他设备均为 Ctrl
    const isMac = /macintosh|mac os x/i.test(navigator.userAgent);
    if (isMac) {
        return '(⌘+Enter)'
    } else {
        return '(Ctrl+Enter)'
    }
}

const ToolRunCell = (props) => {

    const {
        cellType,
        executionCount,
        focus,
        cellState,
        stopCell,
        runCell,
        cellId,
        cellProp,
        outputIsError,
    } = props

    const currentcell = document.getElementById("cellbox-" + cellId)
    const cellTop = currentcell && currentcell.getBoundingClientRect().top

    const toolbar = document.getElementById("cellbar-" + cellId)
    const contentHeight = currentcell && currentcell.offsetHeight
    const toolbarHeight = toolbar && toolbar.offsetHeight
    const [showCellButton, setShowCellButton] = useState(false);

    useEffect(() => {
        setShowCellButton(Math.abs(cellTop - 130 - toolbarHeight) < contentHeight)
    }, [cellTop]);

    const cellButton = () => {
        let title;
        switch (cellState) {
            case "pending":
                title = intl.get("PENDING")
                break
            case "executing":
                title = intl.get("STOP")
                break
            case 'paused':
                title = intl.get("PAUSED")
                break
            default:
                title = intl.get("RUN") + winOrMac()
        }

        const stateButton = () => {
            switch (cellState) {
                case "pending":
                    return (
                        <Button
                            icon={<Icons.BHCellPendingIcon />}
                            style={{ height: '24px' }}
                            type="text"
                        ></Button>
                    )
                case "executing":
                    return (
                        <Button
                            icon={<Icons.BHCellExecutingIcon />}
                            style={{ height: '24px' }}
                            type="text"
                            onClick={stopCell}
                        ></Button>
                    )
                case 'paused':
                    return (
                        <Button
                            icon={<Icons.BHCellResumeIcon />}
                            style={{ height: '24px' }}
                            type="text"
                        ></Button>
                    )
                default:
                    return (
                      <Fragment>
                        <div>
                          <Button
                            icon={<Icons.BHCellReadyIcon />}
                            style={{ height: '24px' }}
                            type="text"
                            onClick={runCell}
                          ></Button>
                        </div>
                        {
                          cellProp.hasExecuted && !outputIsError ?<CheckOutlined style={{transform:'translateY(-6px)',color:'skyblue'}} />:null
                        }
                      </Fragment>
                    )
            }
        }

        return (
            <Tooltip trigger={["focus", "hover"]} placement="bottom" title={title}>
                {stateButton()}
            </Tooltip>
        )
    }

    const runTypeFlgs = ['sql', 'code', 'visualization', 'data_exploration'].includes(cellType);
    const fliterRunStatus = () => {
        const cellStateFlgs = ['pending', 'executing'].includes(cellState);
        return runTypeFlgs && cellStateFlgs;
    }

    return (
        <>
            <div style={{ color: '#8D99A4', height: '22px', display: runTypeFlgs ? "" : "none" }}>
                {(executionCount ? `[${executionCount}]` : '[  ]')}
            </div>
            <div
                id={"cellbar-" + cellId}
                className={fliterRunStatus() ? '' : 'controls-start'}
                style={{ display: runTypeFlgs ? "" : "none" }}
            >
                {cellButton()}
            </div>
            <div style={{ height: '100px' }}>
                {/* 留一段空白，为了鼠标划过去时能显示执行按钮 */}
            </div>
        </>
    )
}

export default ToolRunCell
