import React, {useCallback, useState, useRef, useContext} from 'react'
import { toLower } from 'lodash-es'
import { Button, Popover, Tag } from 'antd'
import { DragSource, ConnectDragPreview, ConnectDragSource } from 'react-dnd'
import { DatabaseFilled, ReadOutlined } from '@ant-design/icons'
// import marked from 'marked'
// import { useSafeSetHTML } from '../../../../../pages/common/hooks/useSafeSetHtml'
import { DRAGGABLE_ALGO_COMPONENT } from '../../../../../constants/graph'
import styles from './node-title.module.less'
import { ScriptView } from '../../../../component-config-panel/form/script-view'
import {observer} from "mobx-react"
import globalData from "idp/global"

// marked.setOptions({
//   gfm: true,
//   breaks: true,
// })

const Document = observer((props: { node: any }) => {
    const { node } = props
    const {workspaceRef} = globalData.appComponentData
    const descriptionNodeRef = useRef<HTMLDivElement>(null)
    const { id, tag = '' } = node


    // const htmlStr = marked(
    //   unescape(description || '暂无文档').replace(/\\n/gi, ' \n '),
    // )
    // useSafeSetHTML(descriptionNodeRef, htmlStr)

    const [scriptVisible, setScriptVisible] = useState(false)

    return (
      <div className={styles.popover}>
        {tag ? (
          <div className={styles.tag}>
            <span className={styles.label}> 标签: </span>
            {tag.split(',').map((str: string) => (
              <Tag key={str}>{str}</Tag>
            ))}
          </div>
        ) : null}
        <div className={styles.description}>
          <div ref={descriptionNodeRef} />
          {id}
          <div className={styles.doclink}>
            <Button style={{padding: 0}} type="link" onClick={() => setScriptVisible(true)}>
              预览
            </Button>
            <Button style={{ padding: 0, marginLeft: 15 }} type="link" onClick={() => {
              const node = {
                key: id,
                name: id.substring(id.lastIndexOf("/")),
                isLeaf: true,
                fileType: "FILE",
              }
              const info = {
                node: node,
              }
              workspaceRef.onSelect(null, info)
            }}>编辑</Button>
          </div>
        </div>
        <ScriptView path={id} visible={scriptVisible} onClose={() => setScriptVisible(false)} />
      </div>
    )
  }
)
interface Props {
  node: any
  searchKey: string
  isDragging: boolean
  connectDragSource: ConnectDragSource
  connectDragPreview: ConnectDragPreview
}

const InnerNodeTitle = (props: Props) => {
  const {
    node = {},
    searchKey = '',
    connectDragPreview,
    connectDragSource,
  } = props
  const { name = '', isDir } = node
  const [visible, setVisible] = useState<boolean>(false)
  const onMouseIn = useCallback(() => {
    setVisible(true)
  }, [])
  const onMouseOut = useCallback(() => {
    setVisible(false)
  }, [])
  const onMouseDown= useCallback(() => {
    setVisible(false)
  }, [])
  // 文件夹
  if (isDir) {
    return <div className={styles.folder}>{name}</div>
  }

  const keywordIdx = searchKey ? toLower(name).indexOf(toLower(searchKey)) : -1

  // 搜索高亮
  if (keywordIdx > -1) {
    const beforeStr = name.substr(0, keywordIdx)
    const afterStr = name.substr(keywordIdx + searchKey.length)

    return connectDragPreview(
      connectDragSource(
        <span className={styles.node}>
          <DatabaseFilled className={styles.nodeIcon} />
          <span className={styles.label}>
            {beforeStr}
            <span className={styles.keyword}>{searchKey}</span>
            {afterStr}
          </span>
        </span>,
      ),
    )
  }

  return (
    <div
      className={styles.nodeTitleWrapper}
      onMouseEnter={onMouseIn}
      onMouseLeave={onMouseOut}
      // onMouseDown={onMouseDown}
    >
      {connectDragPreview(
        connectDragSource(
          <div className={styles.node}>
            <DatabaseFilled className={styles.nodeIcon} />
            <span className={styles.label}>{name}</span>
          </div>,
        ),
      )}
      {visible && (
        <Popover
          visible={true}
          title={name}
          placement="right"
          content={<Document node={node} />}
          key="description"
        >
          <a className={styles.doc}>
            <ReadOutlined /> 文档
          </a>
        </Popover>
      )}
    </div>
  )
}

export const NodeTitle = DragSource(
  DRAGGABLE_ALGO_COMPONENT,
  {
    beginDrag: (props: Props) => ({
      component: props.node,
    }),
  },
  (connect, monitor) => ({
    connectDragSource: connect.dragSource(),
    connectDragPreview: connect.dragPreview(),
    isDragging: monitor.isDragging(),
  }),
)(InnerNodeTitle)
