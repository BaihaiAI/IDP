import { useSelector, useDispatch } from "react-redux"
import {useState, useEffect, useRef, useContext} from "react"
import intl from "react-intl-universal"
import { Button } from "antd"
import { contentAddCell } from "../../../../store/features/notebookSlice"
import "./cellTools.less"
import {NotebookComponentContext} from "../../Notebook"
import { contentApi } from "@/services"

const ToolAddCell = (props) => {
  const { responsive, onAddCell, path, index } = props
  const dispatch = useDispatch()
  const [visible, setVisible] = useState(false)
  let timer = null
  const {cells} = useContext(NotebookComponentContext)

  const mouseover = (e) => {
    // e?.stopPropagation()
    // e?.preventDefault()
    if (timer) {
      clearTimeout(timer)
      timer = null
    }
    setVisible(true)
  }
  const mouseout = (e) => {
    timer = setTimeout(function () {
      setVisible(false)
    }, 500)
  }

  const addCell = (cellType) => {
    console.log(cellType, '----=======-------')
    // 增加cell之前保存下快照
    contentApi.snapshot({
      path,
      label: intl.get("SAVE_VERSION_AUTO"),
    }).catch((error) => {
      console.log(error)
    })
    dispatch(contentAddCell({ path, index, cellType,cells }))
      .unwrap()
      .then((res) => {
        console.log(res, 'res----======-------')
        onAddCell(res.data.metadata.id)
      })
  }

  return (
    <>
      {responsive ? (
        <div
          onMouseOver={mouseover}
          onMouseOut={mouseout}
          className={
            "addcell-btns-box" + (visible ? " addcell-btns-box-hover" : "")
          }
        >
          <div className="addcell-btns">
            <Button
              style={{
                marginRight: "10px",
                backgroundColor: "#fff",
                color: "#8a8a8a",
                fontWeight: 500,
                borderRadius: 4,
              }}
              onClick={() => addCell("code")}
            >
              + {intl.get("CODE")}
            </Button>
            <Button
              style={{
                marginRight: "10px",
                backgroundColor: "#fff",
                color: "#8a8a8a",
                fontWeight: 500,
                borderRadius: 4,
              }}
              onClick={() => addCell("markdown")}
            >
              + {intl.get("TEXT")}
            </Button>
            <Button
              style={{
                marginRight: "10px",
                backgroundColor: "#fff",
                color: "#8a8a8a",
                fontWeight: 500,
                borderRadius: 4,
              }}
              onClick={() => addCell("sql")}
            >
              + {intl.get("SQL")}
            </Button>
            <Button
              style={{
                marginRight: "10px",
                backgroundColor: "#fff",
                color: "#8a8a8a",
                fontWeight: 500,
                borderRadius: 4,
              }}
              onClick={() => addCell("visualization")}
            >
              + {intl.get("DATA_VISUALIZATION")}
            </Button>
            <Button
              style={{
                marginRight: "10px",
                backgroundColor: "#fff",
                color: "#8a8a8a",
                fontWeight: 500,
                borderRadius: 4,
              }}
              onClick={() => addCell("data_exploration")}
            >
              + 数据探索
            </Button>
          </div>
        </div>
      ) : (
        <div className="addcell-btns-box">
          <div className="addcell-btns">
            <Button
              style={{
                marginRight: "10px",
                backgroundColor: "#fff",
                color: "#8a8a8a",
                fontWeight: 500,
                borderRadius: 4,
              }}
              onClick={() => addCell("code")}
            >
              + {intl.get("CODE")}
            </Button>
            <Button
              style={{
                marginRight: "10px",
                backgroundColor: "#fff",
                color: "#8a8a8a",
                fontWeight: 500,
                borderRadius: 4,
              }}
              onClick={() => addCell("markdown")}
            >
              + {intl.get("TEXT")}
            </Button>
            <Button
              style={{
                marginRight: "10px",
                backgroundColor: "#fff",
                color: "#8a8a8a",
                fontWeight: 500,
                borderRadius: 4,
              }}
              onClick={() => addCell("sql")}
            >
              + {intl.get("SQL")}
            </Button>
            <Button
              style={{
                marginRight: "10px",
                backgroundColor: "#fff",
                color: "#8a8a8a",
                fontWeight: 500,
                borderRadius: 4,
              }}
              onClick={() => addCell("visualization")}
            >
              + {intl.get("DATA_VISUALIZATION")}
            </Button>
            <Button
              style={{
                marginRight: "10px",
                backgroundColor: "#fff",
                color: "#8a8a8a",
                fontWeight: 500,
                borderRadius: 4,
              }}
              onClick={() => addCell("data_exploration")}
            >
              + 数据探索
            </Button>
          </div>
        </div>
      )}
    </>
  )
}

export default ToolAddCell
