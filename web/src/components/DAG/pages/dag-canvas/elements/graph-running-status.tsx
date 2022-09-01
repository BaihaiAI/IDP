import React from 'react'
import { LoadingOutlined } from '@ant-design/icons'
import { useObservableState } from '../../../common/hooks/useObservableState'
import { useExperimentGraph } from '../../rx-models/experiment-graph'

interface Props {
  className?: string
  experimentId: string
  experimentInstanceId?: string
}

export const GraphRunningStatus: React.FC<Props> = (props) => {
  const { className, experimentId, experimentInstanceId } = props
  const experimentGraph = useExperimentGraph(experimentId, experimentInstanceId)
  const [executionStatus] = useObservableState(
    () => experimentGraph.executionStatus$,
  )

  return (
    <div className={className}>
      {executionStatus?.status === 'preparing' && (
        <>
          <LoadingOutlined style={{ marginRight: 4 }} /> 准备中...
        </>
      )}
    </div>
  )
}
