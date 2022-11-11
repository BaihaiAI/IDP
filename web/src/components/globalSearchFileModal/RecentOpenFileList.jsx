import React, { Fragment, useRef, useState } from "react"
import { Scrollbars } from "react-custom-scrollbars"
import "./RecentOpenFileList.less"
import { findFileOrDirName, findFileTreeParentKey } from "../../utils"
import fileImg from "../../assets/file.svg"
import { useHotkeys } from "react-hotkeys-hook"
import classNames from "classnames"
import { useMemoizedFn } from "ahooks"

const itemHeight = 26
const containerHeight = 200
function RecentOpenFileList(props) {
  const { list, appComponentAddTabFile, setGlobalSearchVisible } = props
  const [selectIndex, setSelectIndex] = useState(0)
  const scrollRef = useRef()

  const handlerUp = useMemoizedFn(() => {
    const newIndex = selectIndex === 0 ? list.length - 1 : selectIndex - 1
    setSelectIndex(newIndex)
    // 如果是向上滚动 则正常进行滚动
    scrollRef.current.scrollTop(newIndex * itemHeight)
  })

  const handlerDown = useMemoizedFn(() => {
    const newIndex = selectIndex === list.length - 1 ? 0 : selectIndex + 1
    setSelectIndex(newIndex)
    scrollRef.current.scrollTop(newIndex * itemHeight)
  })

  const handlerEnter = useMemoizedFn(() => {
    setGlobalSearchVisible(false)
    appComponentAddTabFile(list[selectIndex].name)
  })

  useHotkeys("up", handlerUp)
  useHotkeys("down", handlerDown)
  useHotkeys("enter", handlerEnter)

  return (
    <Scrollbars ref={scrollRef} style={{ height: containerHeight }}>
      <ul className={"recent-open-file-list"}>
        {list.map((item, index) => {
          const fileName = findFileOrDirName(item.name)
          const path = findFileTreeParentKey(item.name)

          return (
            <li
              onClick={() => {
                setSelectIndex(index)
              }}
              onDoubleClick={() => {
                setGlobalSearchVisible(false)
                appComponentAddTabFile(list[selectIndex].name)
              }}
              key={index}
              className={classNames({
                active: index === selectIndex,
              })}
            >
              <img src={fileImg} alt="" />
              <span className={"file-name"}>{fileName}</span>
              {path ? (
                <Fragment>
                  -<span className={"file-path"}>{path}</span>
                </Fragment>
              ) : null}
            </li>
          )
        })}
      </ul>
    </Scrollbars>
  )
}

export default RecentOpenFileList
