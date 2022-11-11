import React from "react"
import "./ModalFooter.less"
import intl from "react-intl-universal"

function ModalFooter(props) {
  const { showLocalFileAndSearch, searchFileList } = props

  return (
    <div className={"global-search-modal-footer"}>
      {showLocalFileAndSearch ? (
        <div className={"show-local-file-tip"}>↑ ↓ {intl.get("CHOOSE")} enter{intl.get("OPEN")}</div>
      ) : searchFileList.length ? (
        <div className={"show-search-file-tip"}>
          共找到
          <span>{searchFileList.length}</span>条搜索结果
        </div>
      ) : null}
    </div>
  )
}

export default ModalFooter
