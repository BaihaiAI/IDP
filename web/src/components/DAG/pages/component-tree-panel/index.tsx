import React, { useState } from 'react'
import classNames from 'classnames'
import { ComponentSourceTree } from './component-source-tree'
import styles from './index.module.less'
import Icons from "../../../Icons/Icons"
import {Button} from "antd"
import PubSub from "pubsub-js"

interface Props {
  className?: string
  experimentId: string
  mode: string
}

type TabOption = 'component' | 'model'

export const ComponentTreePanel: React.FC<Props> = (props) => {
  const { className } = props
  const [activeTab, setActiveTab] = useState<TabOption>('component')

  return (
    <div className={classNames(className, styles.nodeSourceTreeContainer)}>
      <div className={styles.tabWrapper}>
        <div
          style={{
            display:'flex',
            justifyContent:'space-between'
          }}
          className={classNames(styles.tab, {
            [styles.active]: activeTab === 'component',
          })}
          onClick={() => {
            setActiveTab('component')
          }}
        >
          <span>资源库</span>
          <Button
            type={"text"}
            icon={<Icons.BHRefreshIcon />}
            size="small"
            onClick={() => {
              PubSub.publish("refresh-category-tree")
            }}
          />
        </div>
      </div>
      <div className={styles.tabContentWrapper}>
        <ComponentSourceTree
          className={classNames({ [styles.hide]: activeTab !== 'component' })}
        />
      </div>
    </div>
  )
}
