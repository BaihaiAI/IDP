import React, { useCallback, useEffect, useState } from 'react'
import { Popover, message } from 'antd'
import {
  CloudUploadOutlined,
  SaveOutlined,
  LogoutOutlined,
  PlayCircleOutlined,
} from '@ant-design/icons'
import classNames from 'classnames'
import { useObservableState } from '../../common/hooks/useObservableState'
import { useExperimentGraph, gModelMap } from '../rx-models/experiment-graph'
import styles from './bottom-toolbar.module.less'

import {formatExperimentGraph} from '../common/utils/experiment'

interface Props {
  experimentId: string
}

export const BottomToolbar: React.FC<Props> = (props) => {
  const { experimentId } = props
  const expGraph = useExperimentGraph(experimentId)
  const [running] = useObservableState(expGraph.running$)
  const [preparingRun, setPreparingRun] = useState(false)
  const [preparingStop, setPreparingStop] = useState(false)

  // running 的值发生变化，说明运行或停止按钮的操作产生了作用
  useEffect(() => {
    setPreparingRun(false)
    setPreparingStop(false)
  }, [running])

  // 运行实验
  const onRunExperiment = useCallback(() => {
    setPreparingRun(true)
    expGraph.runGraph().then((res: any) => {
      if (!res.success) {
        setPreparingRun(false)
      }
    })
  }, [expGraph])

  // 停止运行
  const onStopRunExperiment = useCallback(() => {
    setPreparingStop(true)
    expGraph.stopRunGraph().then((res: any) => {
      if (!res.success) {
        setPreparingStop(false)
      }
    })
  }, [expGraph])

  // 部署
  const onDeploy= useCallback(() => {
    const data = formatExperimentGraph(expGraph);
    console.log(data)
    console.log(gModelMap);

  }, [expGraph])



  const runningConfigs = [
    {
      content: '运行',
      tip: '依次运行本实验的每个组件',
      icon: PlayCircleOutlined,
      disabled: preparingRun,
      clickHandler: onRunExperiment,
    },
    {
      content: '停止',
      tip: '停止运行实验',
      icon: LogoutOutlined,
      disabled: preparingStop,
      clickHandler: onStopRunExperiment,
    },
  ]

  const runningConfig = runningConfigs[Number(!!running)]
  const RunningIcon = runningConfig.icon

  const saveExperimentGraph = () => {
    expGraph.saveExperimentGraph((success: boolean) => {
      if (success) {
        message.success('保存成功！');
      } else {
        message.error('保存失败！');
      }
    });
  }

  return (
    <div className={styles.bottomToolbar} style={{height: 50}}>
      <ul className={styles.itemList}>
        {/* 保存 */}
        <li className={styles.item} onClick={saveExperimentGraph}>
          <SaveOutlined />
          <span>保存</span>
        </li>

        {/* 运行/停止 */}
        <Popover content={runningConfig.tip} overlayClassName={styles.popover}>
          <li
            className={classNames(styles.item, {
              [styles.disabled]: runningConfig.disabled,
            })}
            onClick={runningConfig.clickHandler}
          >
            <RunningIcon />
            <span>{runningConfig.content}</span>
          </li>
        </Popover>
      </ul>
    </div>
  )
}
