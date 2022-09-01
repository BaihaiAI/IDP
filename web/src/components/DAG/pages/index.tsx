import React from 'react'
import classNames from 'classnames'
import { Layout } from 'antd'
import { RouteComponentProps } from 'react-router'
import { DndProvider } from 'react-dnd'
import { HTML5Backend } from 'react-dnd-html5-backend'
// import { GuideHeader } from '@/layout/header'
import { ComponentTreePanel } from './component-tree-panel'
import { ComponentConfigPanel } from './component-config-panel'
import { DAGCanvas } from './dag-canvas'
import "@antv/x6-react-components/dist/index.css"
import styles from './index.module.less';

interface Props extends RouteComponentProps<{ experimentId: string, mode: string }> {
  experimentId: string
  experimentInstanceId?: string
  mode?: string
}

const { Content } = Layout

const DagDemo: React.FC<Props> = (props) => {
  const { experimentId, experimentInstanceId, mode  } = props
  return (
    <Layout className={styles.layout}>
      {/* <GuideHeader experimentId={experimentId} /> */}
      <Content className={styles.content}>
        <div className={classNames(styles.experiment)}>
          <DndProvider backend={HTML5Backend}>
            {mode !== 'view' &&
            <ComponentTreePanel
              experimentId={experimentId}
              className={styles.nodeSourceTree}
              mode={mode}
            />
            }
            <div className={styles.editPanel}>
              <DAGCanvas
                experimentId={experimentId}
                experimentInstanceId={experimentInstanceId}
                className={styles.dagCanvas}
                mode={mode}
              />
              <ComponentConfigPanel
                experimentId={experimentId}
                experimentInstanceId={experimentInstanceId}
                className={styles.confPanel}
                mode={mode}
              />
            </div>
          </DndProvider>
        </div>
      </Content>
    </Layout>
  )
}

export default DagDemo
