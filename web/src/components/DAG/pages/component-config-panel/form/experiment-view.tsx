import React from 'react'
import { List} from 'antd'
import { useObservableState } from '../../../common/hooks/useObservableState'
import { useExperimentGraph } from '../../rx-models/experiment-graph'

export interface Props {
  experimentId: string
  experimentInstanceId: string
}

interface StatusObj {
  name: string
  jobStatus: string
  startTime: string
  endTime: string
  timeCost: string
}

const experimentAtts: StatusObj = {
  name: '工作流名称',
  jobStatus: '工作流执行状态',
  startTime: '工作流开始时间',
  endTime: '工作流结束时间',
  timeCost: '工作流执行时长',
}

export const ExperimentView: React.FC<Props> = ({ experimentId, experimentInstanceId }) => {
  const expGraph = useExperimentGraph(experimentId, experimentInstanceId)
  const [activeExperiment] = useObservableState(expGraph.experiment$)

  return (
    <List
      itemLayout="horizontal"
    >
      {activeExperiment ? Object.entries(experimentAtts).map(([key, text]) => {
        const value = activeExperiment[key as keyof StatusObj]
        if (value) {
          return (
            <List.Item>
              <List.Item.Meta
                title={text}
                description={value}
              />
            </List.Item>
          )
        }
        return null
      }): null}
    </List>
  )
}
