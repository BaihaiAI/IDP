import { useContext, useEffect } from "react"
import { useSelector, useDispatch } from "react-redux"
import intl from "react-intl-universal"
import { Col, Tooltip, Button } from "antd"

import Icons from "../../../Icons/Icons"
import {
  contentDelCell,
  contentMoveCell,
  contentUpdateCellOutputs,
  delCell,
  moveCell,
  updateCell,
  updateCellProps,
} from "../../../../store/features/notebookSlice"
import { NotebookComponentContext } from "../../Notebook"
import Pubsub from 'pubsub-js'
import { contentApi } from "@/services"

const RightTopBar = (props) => {
  const { path, cellId, index, stopCell, runCurrentCellAndAbove, runCurrentCellAndBelow,outputs } = props
  const { cells, cellProps } = useContext(NotebookComponentContext)
  const dispatch = useDispatch()

  useEffect(() => {
    const subscribe = Pubsub.subscribe("mapkeyDeleteCell",(msg, data) => {
      if (data.key === cellId) {
        deleteCell(data.bol)
      }
    })
    return () => {
      Pubsub.unsubscribe(subscribe)
    }
  }, [])

  //上移 cell
  const moveUpCell = () => {
    if (index <= 0) return
    const originIndex = index
    const targetIndex = index - 1
    const neighborCellId = cells[targetIndex].metadata.id
    dispatch(
      contentMoveCell({
        path,
        cellId,
        neighborCellId
      })
    ).then(() => {
      dispatch(
        moveCell({
          path,
          originIndex,
          targetIndex,
        })
      )
    })
  }

  //下移 cell
  const moveDownCell = () => {
    if (index >= cells.length - 1 || index < 0) return
    const originIndex = index
    const targetIndex = index + 1

    const neighborCellId = cells[targetIndex].metadata.id

    dispatch(
      contentMoveCell({
        path,
        cellId,
        neighborCellId
      })
    ).then(() => {
      dispatch(
        moveCell({
          path,
          originIndex,
          targetIndex,
        })
      )
    })
  }
  const resetKernel = () => {
    let nextCellProps = {}
    for (const key in cellProps) {
      nextCellProps[key] = { ...cellProps[key], state: "ready" }
    }
    dispatch(updateCellProps({ path, cellProps: nextCellProps }))
  }

  const outputIsError = (outputs) => {
    if (outputs) {
      for (const output of outputs) {
        if ("ename" in output) {
          return true
        }
      }
    }
    return false
  }

  // 清空输出
  const clearOutput = () => {
    const isError = outputIsError(outputs)
    const cell = { ...cells[index], outputs: [] }
    dispatch(updateCell({ path, cellId, cell,isError }))
    dispatch(contentUpdateCellOutputs({ path, cellId }))
  }

  // 删除 cell
  const deleteCell = async (bol=false) => {
    if (cellProps[cellId].state !== "ready") {
      await stopCell(cellId)
      resetKernel()
    }
    // 当中断接口调用完成后 执行删除操作
    // 删除cell之前保存下快照
    await contentApi.snapshot({
      path,
      label: intl.get("SAVE_VERSION_AUTO"),
    }).catch((error) => {
      console.log(error)
    })

    dispatch(contentDelCell({ path, index, cellId, bol })).then(() => {
      dispatch(delCell({ path, index }))
    })
  }

  return (
    <Col className="code-editor-topbar-actions anticon">
      <Tooltip placement="bottom" title={intl.get("RUN_CURRENT_CELL_AND_ABOVE")}>
        <Button
          icon={<Icons.BHRunPreCellsIcon />}
          size="small"
          type="text"
          onClick={() => {
            // console.log(props.cellId)
            return runCurrentCellAndAbove(props.cellId)
          }}
        ></Button>
      </Tooltip>
      <Tooltip placement="bottom" title={intl.get("RUN_CURRENT_CELL_AND_BELOW")}>
        <Button
          icon={<Icons.BHRunNextCellsIcon />}
          size="small"
          type="text"
          onClick={() => runCurrentCellAndBelow(props.cellId)}
        ></Button>
      </Tooltip >
      <Tooltip placement="bottom" title={intl.get("MOVEUPCELL")}>
        <Button
          icon={<Icons.BHArrowUpIcon />}
          size="small"
          type="text"
          onClick={() => moveUpCell()}
        ></Button>
      </Tooltip>
      <Tooltip placement="bottom" title={intl.get("MOVEDOWNCELL")}>
        <Button
          icon={<Icons.BHArrowDownIcon />}
          size="small"
          type="text"
          onClick={() => moveDownCell()}
        ></Button>
      </Tooltip>
      <Tooltip placement="bottom" title={intl.get("CLEANOUTPUT")}>
        <Button
          icon={<Icons.BHCleanIcon />}
          size="small"
          type="text"
          onClick={() => clearOutput()}
        ></Button>
      </Tooltip>
      <Tooltip placement="bottom" title={intl.get("DELETECELL")}>
        <Button
          icon={<Icons.BHDeleteIcon />}
          size="small"
          type="text"
          onClick={() => deleteCell()}
        ></Button>
      </Tooltip>
    </Col>
  )
}

export default RightTopBar
