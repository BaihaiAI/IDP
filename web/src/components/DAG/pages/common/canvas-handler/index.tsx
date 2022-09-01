import React from 'react'
import { Popover } from 'antd'
import {
  CompressOutlined,
  OneToOneOutlined,
  ZoomInOutlined,
  ZoomOutOutlined,
  RollbackOutlined,
} from '@ant-design/icons'
import classNames from 'classnames'
import styles from './index.module.less'

interface Props {
  className?: string
  onZoomIn: () => void
  onZoomOut: () => void
  onFitContent: () => void
  onRealContent: () => void
  onRedoContent?: () => void
  onUndoContent?: () => void
  mode?: string
}

export const CanvasHandler: React.FC<Props> = (props) => {
  const { className, onZoomIn, onZoomOut, onFitContent, onRealContent, onRedoContent, onUndoContent, mode } = props

  return (
    <ul className={classNames(styles.handler, className)}>
      <Popover
        overlayClassName={styles.popover}
        content="放大"
        placement="right"
      >
        <li onClick={onZoomIn} className={styles.item}>
          <ZoomInOutlined />
        </li>
      </Popover>
      <Popover
        overlayClassName={styles.popover}
        content="缩小"
        placement="right"
      >
        <li onClick={onZoomOut} className={styles.item}>
          <ZoomOutOutlined />
        </li>
      </Popover>
      <Popover
        overlayClassName={styles.popover}
        content="实际尺寸"
        placement="right"
      >
        <li onClick={onRealContent} className={styles.item}>
          <OneToOneOutlined />
        </li>
      </Popover>
      <Popover
        overlayClassName={styles.popover}
        content="适应画布"
        placement="right"
      >
        <li onClick={onFitContent} className={styles.item}>
          <CompressOutlined />
        </li>
      </Popover>
      { mode !== 'view' && <div>
      <Popover
        overlayClassName={styles.popover}
        content="撤销"
        placement="right"
      >
        <li onClick={onUndoContent} className={styles.item}>
          <RollbackOutlined/>
        </li>
      </Popover>
      <Popover
        overlayClassName={styles.popover}
        content="重做"
        placement="right"
      >
        <li onClick={onRedoContent} className={styles.item}>
          <RollbackOutlined rotate={180}/>
        </li>
      </Popover>
      </div>
      }
    </ul>
  )
}
