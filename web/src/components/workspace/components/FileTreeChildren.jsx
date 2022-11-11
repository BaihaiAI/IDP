import React from "react"
import { List } from "antd"
import PropTypes from "prop-types"

const formatSize = (size) => {
  if (size < 1024 * 1024) {
    return (size / 1024).toFixed(1) + "K"
  } else if (size >= 1024 * 1024 && size < 1024 * 1024 * 1024) {
    return (size / (1024 * 1024)).toFixed(1) + "M"
  } else {
    return (size / (1024 * 1024 * 1024)).toFixed(1) + "G"
  }
}

function FileTreeChildren(props) {
  const { uploadFileList } = props

  if (uploadFileList.length > 0) {
    return (
      <List
        size="small"
        dataSource={uploadFileList}
        locale={null}
        renderItem={(item) => (
          <List.Item key={item.key}>
            <List.Item.Meta
              description={
                item.name.length <= 18
                  ? item.name
                  : item.name.slice(0, 18) + "..."
              }
            />
            <div>
              {formatSize(item.completeSize)} / {formatSize(item.totalSize)}
            </div>
          </List.Item>
        )}
      />
    )
  }

  return <div></div>
}

FileTreeChildren.propTypes = {
  uploadFileList:PropTypes.array.isRequired,
}

export default FileTreeChildren
