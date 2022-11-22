import React,{useEffect, useState} from 'react'
import { Form, Input, Select } from 'antd'
import { useObservableState } from '../../../common/hooks/useObservableState'
import { useExperimentGraph } from '../../rx-models/experiment-graph'
import { getCurrentEnv } from '../../../../../store/config'

const {Option} = Select;
export interface Props {
  experimentId: string
  mode?: string
  environmentList: any
}

export const ExperimentForm: React.FC<Props> = ({ experimentId, mode, environmentList }) => {
  const [form] = Form.useForm()

  const expGraph = useExperimentGraph(experimentId)
  const [activeExperiment] = useObservableState(expGraph.experiment$)

  const onValuesChange = ({ experimentName, taskParallel, maxRetryTimes, envName, priority }: { experimentName: string, taskParallel: string, maxRetryTimes: string, envName: string, priority: number }) => {
    let nextActiveExperiment = { ...activeExperiment };
    if (experimentName !== undefined) nextActiveExperiment = { ...nextActiveExperiment, name: experimentName }
    if (taskParallel) nextActiveExperiment = { ...nextActiveExperiment, taskParallel: Number(taskParallel) }
    if (maxRetryTimes) nextActiveExperiment = { ...nextActiveExperiment, maxRetryTimes: Number(maxRetryTimes) }
    if (envName) {
      nextActiveExperiment = { ...nextActiveExperiment, envName }
      expGraph.updateAllNodeEnvName(envName);
    }
    if (priority !== undefined) {
      let nextPriority = priority
      if (Number(priority) > 5) {
        nextPriority = 5
      }
      nextActiveExperiment = { ...nextActiveExperiment, priority: nextPriority }
      expGraph.updateAllNodePriority(nextPriority);
    }
    expGraph.experiment$.next(nextActiveExperiment);
  }

  useEffect(() => {
    const currentEnv = getCurrentEnv()
    form.setFieldsValue({
      experimentName: activeExperiment ? activeExperiment.name : '',
      taskParallel: activeExperiment ? activeExperiment.taskParallel : '',
      maxRetryTimes: activeExperiment ? activeExperiment.maxRetryTimes : '',
      envName: activeExperiment ? (activeExperiment.envName ? activeExperiment.envName : currentEnv) : currentEnv,
      priority: activeExperiment ? activeExperiment.priority : 50,
    })
  }, [activeExperiment])

  const handlePriorityBlur = (e: any) => {
    const value = e.target.value
    if (value === '') {
      let priority = 1
      form.setFieldsValue({
        priority: priority
      })
      expGraph.experiment$.next({ ...activeExperiment, priority: priority });
      expGraph.updateAllNodePriority(priority);
    }
  }

  return (
    <Form
      form={form}
      layout="vertical"
      initialValues={{
        experimentName: activeExperiment ? activeExperiment.name : '',
        taskParallel: activeExperiment ? activeExperiment.taskParallel : '',
        maxRetryTimes: activeExperiment ? activeExperiment.maxRetryTimes : '',
        envName: activeExperiment ? (activeExperiment.envName ? activeExperiment.envName : getCurrentEnv()) : getCurrentEnv(),
        priority: activeExperiment ? activeExperiment.priority : 50,
      }}
      onValuesChange={onValuesChange}
      requiredMark={false}
    >
      <Form.Item name="experimentName" label="工作流名称">
        <Input placeholder="请输入工作流名称"  disabled={mode === 'view' ? true : false}/>
      </Form.Item>
      <Form.Item name="envName" label="运行环境">
        <Select placeholder="请选择运行环境" disabled={mode === 'view' ? true : false}>
          {environmentList.map(item => <Option key={item} value={item}>{item}</Option>)}
        </Select>
      </Form.Item>
      <Form.Item label="任务并行度" name="taskParallel">
        <Select  disabled={mode === 'view' ? true : false}>
          <Option value="1">1</Option>
          <Option value="2">2</Option>
          <Option value="3">3</Option>
          <Option value="4">4</Option>
          <Option value="5">5</Option>
        </Select>
      </Form.Item>
      <Form.Item label="任务重试次数" name="maxRetryTimes">
        <Select  disabled={mode === 'view' ? true : false}>
          <Option value="0">0</Option>
          <Option value="1">1</Option>
          <Option value="2">2</Option>
          <Option value="3">3</Option>
        </Select>
      </Form.Item>
      <Form.Item label="优先级" name="priority">
        <Input type="number" min="1" max="5" step="1"
          disabled={mode === 'view' ? true : false}
          onBlur={handlePriorityBlur} />
      </Form.Item>
    </Form>
  )
}
