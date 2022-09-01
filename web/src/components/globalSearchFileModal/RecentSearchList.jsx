import React from "react"
import { SearchOutlined } from "@ant-design/icons"
import "./RecentSearchList.less"
function RecentSearchList(props) {
  const { list, handlerSearchItemClick } = props
  return (
    <ul className={"recent-search-list"}>
      {list.map((item, index) => {
        return (
          <li
            onClick={() => {
              handlerSearchItemClick(item)
            }}
            key={index}
          >
            <SearchOutlined style={{ color: "#98A3AC" }} />
            <span className={"search-name"}>{item}</span>
          </li>
        )
      })}
    </ul>
  )
}

export default RecentSearchList
