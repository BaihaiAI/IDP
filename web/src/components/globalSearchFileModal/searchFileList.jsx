import React, { useRef, useState } from "react"
import { Scrollbars } from "react-custom-scrollbars"
import { useMemoizedFn } from "ahooks"
import { useHotkeys } from "react-hotkeys-hook"
import KeywordsHighlight from "keywordhighlight"
import "./searchFileList.less"
import classNames from "classnames"
import keywordImg from "../../assets/file.svg"
import fileImg from "../../assets/file2.svg"

const itemHeight = 24
const containerHeight = 400
function SearchFileList(props) {
  const {
    list,
    searchKeyword,
    setGlobalSearchVisible,
    appComponentAddTabFile,
  } = props
  const scrollRef = useRef()
  const [selectIndex, setSelectIndex] = useState(0)

  const handlerUp = useMemoizedFn(() => {
    const newIndex = selectIndex === 0 ? list.length - 1 : selectIndex - 1
    setSelectIndex(newIndex)
    // 如果是向上滚动 则正常进行滚动
    scrollRef.current.scrollTop(newIndex * itemHeight)
  })

  const handlerDown = useMemoizedFn(() => {
    const newIndex = selectIndex === list.length - 1 ? 0 : selectIndex + 1
    setSelectIndex(newIndex)
    // 如果是向下滚动 则需要判断它是否到滚动边界了
    if (newIndex * itemHeight > containerHeight || newIndex === 0) {
      scrollRef.current.scrollTop((newIndex + 1) * itemHeight - containerHeight)
    }
  })
  const handlerEnter = useMemoizedFn(() => {
    setGlobalSearchVisible(false)
    const activeItem = list[selectIndex]
    const path = activeItem.browserPath
    appComponentAddTabFile(path, activeItem.line, activeItem.cellId)
    // const type = activeItem.line === 0 ? "file" : "keyword"
    // if (type === "file") {
    //   appComponentAddTabFile(path)
    // } else {
    //   // todo 如果是keyword类型 再进行特殊处理
    //   // activeItem中有 browserPath,cellId,line等这些属性
      
    // }
  })

  useHotkeys("up", handlerUp)
  useHotkeys("down", handlerDown)
  useHotkeys("enter", handlerEnter)

  return (
    <Scrollbars ref={scrollRef} style={{ height: containerHeight }}>
      <ul className={"search-file-list-wrapper"}>
        {list.map((item, index) => {
          const type = item.line === 0 ? "file" : "keyword"
          let showText
          if (item.text.length > 80) {
            showText =
              item.text.slice(0, 30) +
              "..." +
              (item.text.slice(55).length > 35
                ? item.text.slice(55, 90) + "..."
                : item.text.slice(55))
          } else {
            showText = item.text
          }

          return (
            <li
              onClick={() => {
                setSelectIndex(index)
              }}
              onDoubleClick={() => {
                handlerEnter()
              }}
              key={index}
              className={classNames("search-file-item", {
                active: index === selectIndex,
              })}
            >
              <img src={type === "file" ? fileImg : keywordImg} alt="" />
              <span className={"search-content"}>
                <KeywordsHighlight
                  str={showText}
                  keywords={[
                    [
                      searchKeyword,
                      (text) => (
                        <span
                          style={{ color: "#333", backgroundColor: "#FFE39D" }}
                        >
                          {text}
                        </span>
                      ),
                    ],
                  ]}
                />
              </span>

              <span className={"fileName"}>{item.browserPath}</span>
              {type === "keyword" ? (
                <span className={"line"}>{item.line}</span>
              ) : null}
            </li>
          )
        })}
      </ul>
    </Scrollbars>
  )
}

export default SearchFileList
