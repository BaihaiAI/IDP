import React, { useCallback, useContext, useEffect, useRef } from 'react'
import { Button, message } from 'antd'
import '@antv/x6-react-shape'
import { useDrop } from 'react-dnd'
import classNames from 'classnames'
import { DRAGGABLE_ALGO_COMPONENT, DRAGGABLE_MODEL } from '../../constants/graph'
import { useExperimentGraph } from '../rx-models/experiment-graph'
import { FloatingContextMenu } from './elements/floating-context-menu'
import { CanvasHandler } from '../common/canvas-handler'
import { GraphRunningStatus } from './elements/graph-running-status'
import { PipeLineContext } from '@/components/pipeLine/PipeLineHome'
import styles from './canvas-content.module.less'
import DAG from '../../DAG'
import { useObservableState } from '../../common/hooks/useObservableState'

interface Props {
  experimentId: string
  experimentInstanceId?: string
  className?: string
  mode?: string
}

export const CanvasContent: React.FC<Props> = (props) => {
  const { experimentId, experimentInstanceId, className, mode } = props

  const containerRef = useRef<HTMLDivElement | null>(null)
  const canvasRef = useRef<HTMLDivElement | null>(null)
  const expGraph = useExperimentGraph(experimentId, experimentInstanceId)
  const [activeExperiment] = useObservableState(expGraph.experiment$)

  // 打开编辑工作流tab页
  const { addTabPane } = useContext(PipeLineContext)
  const editorExperiment = () => {
    addTabPane({
      key: "editJonInstance" + experimentId,
      title: `编辑${activeExperiment.name}`,
      content: <DAG experimentId={experimentId + ""} />,
    })
  }

  // 渲染画布
  useEffect(() => {
    expGraph.renderGraph(containerRef.current!, canvasRef.current!)
  }, [expGraph])

  // 处理组件拖拽落下事件
  const [, dropRef] = useDrop({
    accept: [DRAGGABLE_ALGO_COMPONENT, DRAGGABLE_MODEL],
    drop: (item: any, monitor) => {
      const currentMouseOffset = monitor.getClientOffset()
      const sourceMouseOffset = monitor.getInitialClientOffset()
      const sourceElementOffset = monitor.getInitialSourceClientOffset()
      const diffX = sourceMouseOffset!.x - sourceElementOffset!.x
      const diffY = sourceMouseOffset!.y - sourceElementOffset!.y
      const x = currentMouseOffset!.x - diffX
      const y = currentMouseOffset!.y - diffY
      if (expGraph.isGraphReady()) {
        expGraph.requestAddNode({
          clientX: x,
          clientY: y,
          nodeMeta: item.component,
        })
      } else {
        message.info('实验数据建立中，请稍后再尝试添加节点')
      }
    },
  })

  // 画布侧边 toolbar handler
  const onHandleSideToolbar = useCallback(
    (action: 'in' | 'out' | 'fit' | 'real'| 'redo' | 'undo') => () => {
      // 确保画布已渲染
      if (expGraph.isGraphReady()) {
        switch (action) {
          case 'in':
            expGraph.zoomGraph(0.1)
            break
          case 'out':
            expGraph.zoomGraph(-0.1)
            break
          case 'fit':
            expGraph.zoomGraphToFit()
            break
          case 'real':
            expGraph.zoomGraphRealSize()
            break
          case 'redo':
            expGraph.redoDeleteNode()
            break
          case 'undo':
            expGraph.undoDeleteNode()
            break
          default:
        }
      }
    },
    [expGraph],
  )
  const modeView = () => {
    if(mode === 'view'){
      return  <div>
      <CanvasHandler
        onZoomIn={onHandleSideToolbar('in')}
        onZoomOut={onHandleSideToolbar('out')}
        onFitContent={onHandleSideToolbar('fit')}
        onRealContent={onHandleSideToolbar('real')}
        mode={mode}
      />

      </div>
    }else{
     return  <div>
      <FloatingContextMenu experimentId={experimentId} />
      <CanvasHandler
        onZoomIn={onHandleSideToolbar('in')}
        onZoomOut={onHandleSideToolbar('out')}
        onFitContent={onHandleSideToolbar('fit')}
        onRealContent={onHandleSideToolbar('real')}
        onRedoContent={onHandleSideToolbar('redo')}
        onUndoContent={onHandleSideToolbar('undo')}
        mode={mode}
      />

      </div>
    }

  }

  return (
    <div
      ref={(elem) => {
        containerRef.current = elem
        dropRef(elem)
      }}
      className={classNames(className, styles.canvasContent)}
    >
      {/* 编辑最新的工作流模版 */}
      {mode === 'view' ? <Button type="link" onClick={editorExperiment}>编辑工作流</Button> : null}

      {/* 图和边的右键菜单 */}
      {modeView()}

      {/* 图运行状态 */}
      <GraphRunningStatus
        className={styles.runningStatus}
        experimentId={experimentId}
        experimentInstanceId={experimentInstanceId}
      />

      {/* 图容器 */}
      <div  ref={canvasRef} />
    </div>
  )
}
