import React, { useEffect } from 'react'
import classNames from 'classnames'
import {
  useExperimentGraph,
  useUnmountExperimentGraph,
} from '../rx-models/experiment-graph'
import { CanvasContent } from './canvas-content'
import { CanvasToolbar } from './canvas-toolbar'
// import { BottomToolbar } from './bottom-toolbar'

import styles from './index.module.less'

interface Props {
  experimentId: string
  experimentInstanceId?: string
  className?: string
  mode?: string
}

export const DAGCanvas: React.FC<Props> = (props) => {
  const { experimentId, experimentInstanceId, className, mode } = props
  const expGraph = useExperimentGraph(experimentId, experimentInstanceId)

  // 处理画布卸载
  useUnmountExperimentGraph(experimentId, experimentInstanceId)

  // 自定义算法组件的渲染控制
  useEffect(() => {
    ;(window as any).renderForm = expGraph.setActiveAlgoData
    return () => {
      delete (window as any).renderForm
    }
  }, [expGraph])

  return (
    <div className={classNames(styles.dagContainer, className, {[styles.viewmode] : mode === 'view'})}>
      {mode === 'view'
       ?
       <div></div>
       :
      <CanvasToolbar
        experimentId={experimentId}
        mode={mode}
         />
      }
      <CanvasContent
        experimentId={experimentId}
        experimentInstanceId={experimentInstanceId}
        className={styles.canvasContent}
        mode={mode}
      />
      {/* <BottomToolbar experimentId={experimentId} /> */}
    </div>
  )
}
