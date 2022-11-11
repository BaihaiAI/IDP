import React, { useState, useEffect } from 'react'
import { Form, Input, Select, TreeSelect } from 'antd'
import { useObservableState } from '../../../common/hooks/useObservableState'
import { useExperimentGraph } from '../../rx-models/experiment-graph'
import 'antd/lib/style/index.css'
import { InfoCircleOutlined } from '@ant-design/icons'
import workspaceApi from 'idpServices/workspaceApi';
import { ScriptView } from './script-view'

const { Option } = Select;

interface treeNode {
  key: string,
  title: string,
  value: string,
  isLeaf: boolean,
  disabled: boolean,
  children: treeNode[],
}

export interface Props {
  nodeId: string
  experimentId: string
  mode?: string
}

export const NodeFormDemo: React.FC<Props> = ({
  nodeId,
  experimentId,
  mode,
}) => {

  const [treeData, setTreeData] = useState([]);
  const [form] = Form.useForm()

  const expGraph = useExperimentGraph(experimentId)
  const [node] = useObservableState(() => expGraph.activeNodeInstance$)
  const [scriptPath, setScriptPath] = useState(node ? node.script : '');

  const onValuesChange = async ({ name, script, machine, preNodeRelation, priority }: { name: string, script: string, machine: string, preNodeRelation: string, priority: number }) => {
    if (name && node.name !== name) {
      await expGraph.renameNode(nodeId, name)
    }
    if (script && node.script !== script) {
      setScriptPath(script)
      await expGraph.updateNodeScript(nodeId, script);
    }
    if (machine && node.machine !== machine) {
      await expGraph.udpateNodeMachine(nodeId, machine);
    }
    if (preNodeRelation && node.preNodeRelation !== preNodeRelation) {
      await expGraph.updateNodePreNodeRelation(nodeId, preNodeRelation);
    }
    if (node.priority !== priority) {
      let nextPriority = priority
      if (!priority) {
        nextPriority = 1
      } else if (Number(priority) > 100) {
        nextPriority = 100
      }
      await expGraph.updateNodePriority(nodeId, nextPriority);
      if (priority !== nextPriority) {
        form.setFieldsValue({ priority: nextPriority})
      }
    }
  }

  const parseDirBrowse = (data) => {
    let treeData = [];
    for (const item of data) {
      let node: treeNode = {
        key: item.browserPath,
        title: item.fileName,
        value: item.browserPath,
        isLeaf: 'FILE' === item.fileType,
        disabled: 'FILE' !== item.fileType,
        children: [],
      };
      if (item.hasChildren) {
        node.children = parseDirBrowse(item.children);
      }
      treeData.push(node);
    }
    return treeData;
  }

  useEffect(() => {
    workspaceApi.dirBrowseForPipeline()
      .then(function (response) {
        setTreeData(parseDirBrowse(response.data.children));
      }).catch((err) => {
        console.log(err);
      });
  }, [experimentId]);

  const [scriptVisible, setScriptVisible] = useState(false)

  return (
    <>
      <Form
        form={form}
        layout="vertical"
        initialValues={{
          name: node ? node.name : '',
          script: node ? node.script : '',
          machine: node ? node.machine : '2vCPUs 8GB',
          preNodeRelation: node ? node.preNodeRelation : 'ALL_SUCCESS',
          priority: node ? node.priority : 0,
        }}
        onValuesChange={onValuesChange}
        requiredMark={false}
      >
        <Form.Item label="节点名称" name="name">
          <Input placeholder="input placeholder" disabled={mode === 'view' ? true : false} />
        </Form.Item>
        <Form.Item label="脚本内容" name="script">
          <TreeSelect
            showSearch
            style={{ width: '100%' }}
            dropdownStyle={{ maxHeight: 400, overflow: 'auto' }}
            placeholder="Please select"
            treeData={treeData}
            disabled={mode === 'view' ? true : false}
            suffixIcon={<InfoCircleOutlined onClick={() => setScriptVisible(true)} />}
          >
          </TreeSelect>


        </Form.Item>
        <Form.Item label="资源配置" name="machine">
          <Select style={{ width: "100%" }} disabled={mode === 'view' ? true : false}>
            <Option value="0.1vCPUs 0.1GB">0.1vCPUs 0.1GB</Option>
            <Option value="1vCPUs 1GB">1vCPUs 1GB</Option>
            <Option value="1vCPUs 2GB">1vCPUs 2GB</Option>
            <Option value="2vCPUs 4GB">2vCPUs 4GB</Option>
            <Option value="4vCPUs 4GB">4vCPUs 4GB</Option>
            <Option value="4vCPUs 8GB">4vCPUs 8GB</Option>
            <Option value="8vCPUs 16GB">8vCPUs 16GB</Option>
            <Option value="0.1vCPUs 0.1GB 0.1GPUs">0.1vCPUs 0.1GB 0.1GPUs</Option>
            <Option value="1vCPUs 1GB 1GPUs">1vCPUs 1GB 1GPUs</Option>
            <Option value="1vCPUs 2GB 1GPUs">1vCPUs 2GB 1GPUs</Option>
            <Option value="2vCPUs 4GB 1GPUs">2vCPUs 4GB 1GPUs</Option>
            <Option value="4vCPUs 4GB 1GPUs">4vCPUs 4GB 1GPUs</Option>
            <Option value="4vCPUs 8GB 2GPUs">4vCPUs 8GB 2GPUs</Option>
            <Option value="8vCPUs 16GB 2GPUs">8vCPUs 16GB 2GPUs</Option>
          </Select>
        </Form.Item>
        <Form.Item label="执行条件" name="preNodeRelation">
          <Select style={{ width: "100%" }} disabled={mode === 'view' ? true : false}>
            <Option value="ALL_SUCCESS">All success</Option>
            <Option value="ONE_SUCCESS">One success</Option>
            <Option value="ALL_FINISH">All finish</Option>
            <Option value="ONE_FINISH">One finish</Option>
          </Select>
        </Form.Item>
        <Form.Item label="优先级" name="priority">
          <Input type="number" min="1" max="100" step="1" disabled={mode === 'view' ? true : false} />
        </Form.Item>
      </Form>
      <ScriptView path={scriptPath} visible={scriptVisible} onClose={() => setScriptVisible(false)} expGraph={expGraph}  />
    </>
  )
}
