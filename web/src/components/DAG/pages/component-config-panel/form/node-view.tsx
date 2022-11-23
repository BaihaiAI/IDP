import React, { useState } from 'react'
import { List, Drawer, Button, Tooltip, Spin, Divider } from 'antd'
import { filter, map } from 'rxjs/operators'
import { UnControlled as CodeMirror } from 'react-codemirror2';
import { useObservableState } from '../../../common/hooks/useObservableState'
import { useExperimentGraph } from '../../rx-models/experiment-graph'
import { NExecutionStatus } from '../../rx-models/typing'
import contentApi from 'idpServices/contentApi'
import pipelineApi from 'idpServices/pipelineApi'
import { HtmlView } from './html-view'
import 'antd/lib/style/index.css'

export interface Props {
  nodeId: string
  experimentId: string
  experimentInstanceId: string
}

interface StatusObj {
  name: string
  defName: string
  jobStatus: string
  startTime: string
  endTime: string
  timeCost: string
  machine: string
  jobResult: string
}

const nodeAtts: StatusObj = {
  name: '任务名称',
  jobStatus: '任务执行状态',
  defName: '任务脚本内容',
  machine: '任务资源配置',
  startTime: '任务开始时间',
  endTime: '任务结束时间',
  timeCost: '任务执行时长',
  jobResult: '任务运行结果',
}

export const NodeView: React.FC<Props> = ({
  nodeId,
  experimentId,
  experimentInstanceId,
}) => {
  const experimentGraph = useExperimentGraph(experimentId, experimentInstanceId)
  const [instanceStatus] = useObservableState(
    () =>
      experimentGraph.executionStatus$.pipe(
        filter((x) => !!x),
        map((x) => x.execInfo),
      ),
    {} as NExecutionStatus.ExecutionStatus['execInfo'],
  )
  const nodeStatus: any = instanceStatus[nodeId] || {}

  const [loading, setLoading] = useState(true);
  const [scriptVisible, setScriptVisible] = useState(false)
  const [scriptContent, setScriptContent] = useState('')
  const [scriptTitle, setScriptTitle] = useState('脚本内容')
  const showScript = (path: string, title?: string) => {
    title && setScriptTitle(title)
    setScriptVisible(true);
    setLoading(true)
    const options = {
      path,
      jobId: experimentId,
      jobInstanceId: experimentInstanceId,
      taskId: nodeStatus.taskInstanceId,
    }
    contentApi.taskResult(options).then((res: any) => {
      const { content } = res.data
      setLoading(false);
      setScriptContent(content)
    }).catch((err: any) => {
      setLoading(false);
      setScriptContent(String(err))
      console.log(err)
    })
  }

  const view = () => {
    const path = nodeStatus.defName
    if (!path) return null
    const suffix = path.slice(path.lastIndexOf('.'))
    switch (suffix) {
      case '.ipynb':
      case '.idpnb':
        return (
          <HtmlView
            id="scriptContent"
            html={scriptContent}
          />
        )
      case '.py':
        return (
          <CodeMirror
            value={scriptContent}
            options={{
              readOnly: 'nocursor',
              theme: 'xq-light',
              mode: 'python',
            }}
          />
        )
      case '.sh':
        return (
          <CodeMirror
            value={scriptContent}
            options={{
              readOnly: 'nocursor',
              theme: 'xq-light',
              mode: 'shell',
            }}
          />
        )
      default:
        return (
          <HtmlView
            id="scriptContent"
            html={`<pre>${scriptContent}</pre>`}
          />
        )
    }
  }

  const [logVisible, setLogVisible] = useState(false)
  const [logContent, setLogContent] = useState([])
  const [logIsOver, setLogIsOver] = useState(false)
  const [logStart, setLogStart] = useState(1)
  const getLog = () => {
    const limit = 30
    setLogVisible(true)
    if (logIsOver) return
    setLoading(true)
    const options = {
      start: logStart,
      limit: limit,
      jobId: experimentId,
      instanceId: experimentInstanceId,
      taskId: nodeStatus.taskInstanceId,
      path: nodeStatus.defName
    }
    pipelineApi.taskLog(options).then((res: any) => {
      const { content, totalLine} = res.data
      setLoading(false)
      setLogContent([...logContent, ...content.split('\n')]);
      if ((logStart + limit) > totalLine) {
        setLogIsOver(true)
      }
      setLogStart(logStart + limit)
    }).catch((err: any) => {
      console.log(err)
      setLoading(false)
      setLogIsOver(true)
    })
  }

  const description = (key, value) => {
    if (key === 'defName') {
      return (
        <Button
          style={{ padding: 0 }}
          type="link"
          onClick={() => showScript(value, '脚本内容')}
        >{value.length > 30
          ? (<Tooltip title={value}>{value.slice(0, 30) + '...'}</Tooltip>)
          : value}
        </Button>
      )
    } else if (key === 'jobResult') {
      const suffix = nodeStatus.defName.slice(nodeStatus.defName.lastIndexOf('.'))
      return (
        <div>
          <span>{value}</span>
          {/* {
            suffix !== '.ipynb' && suffix !== '.idpnb' ?
              <Button type="link" style={{ fontSize: '12px' }} onClick={getLog}>更多日志...</Button> : null
          } */}
          {
            <Button type="link" style={{ fontSize: '12px' }} onClick={
              suffix !== '.ipynb' && suffix !== '.idpnb' ? getLog : () => showScript(nodeStatus.defName, '任务日志')
            }>更多日志...</Button>
          }
        </div>
      )
    } else {
      return value
    }
  }

  return (
    <>
      <List
        itemLayout="horizontal"
        style={{ height: document.body.clientHeight }}
      >
        {Object.entries(nodeAtts).map(([key, text]) => {
          const value = nodeStatus[key as keyof StatusObj]
          if (value) {
            return (
              <List.Item>
                <List.Item.Meta
                  title={text}
                  description={description(key, value)}
                />
              </List.Item>
            )
          }
          return null
        })}
      </List>
      <Drawer
        title={scriptTitle}
        placement="right"
        visible={scriptVisible}
        maskClosable={false}
        width={1000}
        onClose={() => setScriptVisible(false)}
      >
        <Spin spinning={loading} size="large">
          {view()}
        </Spin>
      </Drawer>
      <Drawer
        title="任务日志"
        placement="right"
        visible={logVisible}
        maskClosable={false}
        width={1000}
        onClose={() => setLogVisible(false)}
      >
        <Spin spinning={loading} size="large">
          <div>
            {logContent.map((value, index) => <div key={index}>
              {value}
            </div>)}
            {logIsOver ? null : <Divider>
              <Button type="link" onClick={getLog}>更多日志</Button>
            </Divider>}
          </div>
        </Spin>
      </Drawer>
    </>

  )
}
