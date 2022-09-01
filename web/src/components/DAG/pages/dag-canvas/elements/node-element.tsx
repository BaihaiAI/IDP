import React from 'react'
import { Node } from '@antv/x6'
import classNames from 'classnames'
import { ConfigProvider } from 'antd'
import { filter, map } from 'rxjs/operators'
// import { DatabaseFilled } from '@ant-design/icons'
import { useObservableState } from '../../../common/hooks/useObservableState'
import { ANT_PREFIX } from '../../../constants/global'
import { NExecutionStatus } from '../../rx-models/typing'
import { useExperimentGraph } from '../../rx-models/experiment-graph'
import { NodeStatus } from '../../common/graph-common/node-status'
import { NodePopover } from '../../common/graph-common/node-popover'
import styles from './node-element.module.less'

interface Props {
  experimentId: string
  experimentInstanceId?: string
  node?: Node
}

export const NodeElement: React.FC<Props> = (props) => {
  const { experimentId, experimentInstanceId, node } = props
  const experimentGraph = useExperimentGraph(experimentId, experimentInstanceId)
  const [instanceStatus] = useObservableState(
    () =>
      experimentGraph.executionStatus$.pipe(
        filter((x) => !!x),
        map((x) => x.execInfo),
      ),
    {} as NExecutionStatus.ExecutionStatus['execInfo'],
  )
  const data: any = node?.getData() || {}
  const { name, id, selected } = data
  const nodeStatus:any = instanceStatus[id] || {}

  // const editScript = () => {
  //   if (experimentGraph) {
  //     experimentGraph.saveExperimentGraphSync();
  //   }
  //   const node = {
  //     key: data.script,
  //     name: data.name,
  //     isLeaf: true,
  //     fileType: "FILE",
  //   }
  //   const info = {
  //     node: node,
  //   }
  //   // workspaceRef.onSelect(null, info)
  // }

  return (
    <ConfigProvider prefixCls={ANT_PREFIX}>
      <NodePopover status={nodeStatus}>
        <div
          className={classNames(styles.nodeElement, {
            [styles.selected]: !!selected,
          })}
          // onDoubleClick={editScript}
        >
          <div className={styles.notation}>
          </div>
          <div className={styles.name}>{name}</div>
          {nodeStatus.jobStatus ?
            <NodeStatus
              className={styles.statusIcon}
              status={nodeStatus.jobStatus as any}
            />
            :
            <NodeStatus
              className={styles.statusIcon}
              status={'ready'}
            />
          }
        </div>
      </NodePopover>
    </ConfigProvider>
  )
}
