import React from 'react'
import { Tabs } from 'antd'
import classNames from 'classnames'
import { useObservableState } from '../../common/hooks/useObservableState'
import { useExperimentGraph } from '../rx-models/experiment-graph'
import { ExperimentForm } from './form/experiment-config'
import { ExperimentView } from './form/experiment-view'
import { NodeFormDemo } from './form/node-config'
import { NodeView } from './form/node-view'
import css from './index.module.less'

interface Props {
  experimentId: string
  experimentInstanceId?: string
  className?: string
  mode?: string
}

export const ComponentConfigPanel: React.FC<Props> = (props) => {
  const { experimentId, experimentInstanceId, className, mode } = props
  const expGraph = useExperimentGraph(experimentId, experimentInstanceId)
  const [activeNodeInstance] = useObservableState(
    () => expGraph.activeNodeInstance$,
  )

  const nodeId = activeNodeInstance && activeNodeInstance.id;

  return (
    <div className={classNames(className, css.confPanel)}>
      <div className={css.setting}>
        <Tabs
          defaultActiveKey="setting"
          type="card"
          size="middle"
          tabPosition="top"
          destroyInactiveTabPane={true}
        >
          {mode === 'view' ? (<Tabs.TabPane tab={nodeId ? "任务详情" : "工作流实例详情"} key="view">
            <div className={css.form}>
              {nodeId && (
                <NodeView
                  nodeId={nodeId}
                  experimentId={experimentId}
                  experimentInstanceId={experimentInstanceId}
                />
              )}
              {!nodeId && (
                <ExperimentView
                  experimentId={experimentId}
                  experimentInstanceId={experimentInstanceId}
                />
              )}
            </div>
          </Tabs.TabPane>) : (<Tabs.TabPane tab={nodeId ? "任务设置" : "工作流设置"} key="setting">
            <div className={css.form}>
              {nodeId && (
                <NodeFormDemo
                  nodeId={nodeId}
                  experimentId={experimentId}
                  mode={mode}
                />
              )}
              {!nodeId && (
                <ExperimentForm
                  experimentId={experimentId}
                  mode={mode}
                />
              )}
            </div>
          </Tabs.TabPane>)}

          {/* <Tabs.TabPane tab="全局参数" key="params" disabled={false}>
            <div className={css.form} />
          </Tabs.TabPane> */}
        </Tabs>
      </div>
      {/* <div className={css.footer} /> */}
    </div>
  )
}
