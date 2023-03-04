import { Button, Form, Input, message, Modal, Select } from 'antd';
import { useEffect, useState } from 'react';
import intl from 'react-intl-universal';
import { resourceInfo } from '../../../services/resourceApi';
import clusterApi from '../../../services/clusterApi';
import './resource.less';
import resourceControl from '../../../idp/global/resourceControl';

const { Option } = Select

interface Props {
  visible: boolean
  onCancel: Function
  onFinish: Function
}

export const ResourceModal: React.FC<Props> = ({ visible, onCancel, onFinish }) => {
  const [form] = Form.useForm();
  const [preNumCpu, setPreNumCpu] = useState(0.5)
  const [preNumGpu, setPreNumGpu] = useState(0)
  const [preNumMemory, setPreNumMemory] = useState(1.0)
  const [finishLoading, setFinishLoading] = useState(false);

  // 设置最大资源限制
  // const setMaxResource = () => {
  //   resourceInfo().then((res: any) => {
  //     if (res.data && res.data.length > 0) {
  //       const { cpu, gpu, memory } = res.data[0];
  //       if (cpu && cpu.endsWith('m')) {
  //         const xCpu = (Number(cpu.substring(0, cpu.length - 1)) / 1000).toFixed(1);
  //         setMaxNumCpu(xCpu);
  //       }
  //       if (!gpu || gpu === 'null' || gpu === '0') {
  //         setMaxNumGpu('0');
  //       } else {
  //         if (process.env.REACT_APP_VERSION === 'SAAS') {
  //           setMaxNumGpu((Number(gpu) * 12).toFixed(1));
  //         } else {
  //           setMaxNumGpu(gpu);
  //         }
  //       }
  //       if (memory && memory.endsWith('mi')) {
  //         const xMemeory = (Number(memory.substring(0, memory.length - 2)) / 1024).toFixed(1);
  //         setMaxMemory(xMemeory);
  //       }
  //     }
  //   })
  // }

  //获取可用资源
  const getAvailableResource = (options: any) => {
    // resourceControl.getAvailableResource(options, ({ cpu, gpu, memory }) => {
    //   setPreNumCpu(cpu);
    //   setPreNumGpu(gpu);
    //   setPreNumMemory(memory);
    //   form.setFieldsValue({ cpu: cpu, gpu: gpu, memory: memory });
    // });
  }

  const handleCancel = () => {
    onCancel();
  }

  const handleFinish = () => {
    const { cpu, gpu, memory, priority } = form.getFieldsValue();
    setFinishLoading(true);
    clusterApi.runtimeStart({
      memory: Number(memory),
      numCpu: Number(cpu),
      numGpu: Number(gpu),
      priority: Number(priority)
    }).then((res) => {
      message.success(intl.get('RESOURCEMODAL_STARTUP_SUCCEEDED'));
      setFinishLoading(false);
      resourceControl.getRuntimeStatus(null);
      onFinish();
    }).catch((err) => {
      message.error(intl.get('RESOURCEMODAL_STARTUP_FAILED'));
      setFinishLoading(false);
    });
  }

  useEffect(() => {
    getAvailableResource({
      cpu: resourceControl.minNumCpu,
      gpu: resourceControl.minNumGpu,
      memory: resourceControl.minNumMemory,
      lastAdjust: 'memory',
    });
  }, [])

  return (
    <Modal
      className="resourceMadal"
      title={intl.get('RESOURCEMODAL_SET')}
      visible={visible}
      maskClosable={false}
      onCancel={handleCancel}
      footer={<>
        <Button htmlType="button" onClick={() => getAvailableResource({
          cpu: resourceControl.minNumCpu,
          gpu: resourceControl.minNumGpu,
          memory: resourceControl.minNumMemory,
          lastAdjust: 'memory',
        })}>
          {intl.get('RESOURCEMODAL_RESET')}
        </Button>
        <Button type="primary" onClick={handleFinish} loading={finishLoading}>
          {intl.get('RESOURCEMODAL_STARTUP')}
        </Button>
      </>}
    >
      <Form layout="vertical"
        form={form}
        initialValues={{
          cpu: resourceControl.minNumCpu,
          gpu: resourceControl.minNumGpu,
          memory: resourceControl.minNumMemory,
          priority: resourceControl.minPriority
        }}
      >
        <Form.Item name="cpu" label={`${intl.get('RESOURCEMODAL_CPU_LABEL')}: ${resourceControl.maxNumCpu}${intl.get('RESOURCEBAR_CORE')}`} rules={[{ required: true }]}>
          <Input type="number" 
          onBlur={(e) => {
            const cpu = Number(e.target.value);
            if (cpu > preNumCpu) {
              getAvailableResource({
                cpu: cpu,
                gpu: resourceControl.numGpu,
                memory: resourceControl.numMemory,
                lastAdjust: 'cpu',
              });
            }
          }}
          ></Input>
        </Form.Item>
        <Form.Item name="gpu" label={`${intl.get('RESOURCEMODAL_GPU_LABEL')}: ${resourceControl.maxNumGpu}${resourceControl.gpuUnit}`} rules={[{ required: true }]}>
          <Input type="number" onBlur={(e) => {
            const gpu = Number(e.target.value);
            if (gpu > preNumGpu) {
              getAvailableResource({
                cpu: resourceControl.numCpu,
                gpu: gpu,
                memory: resourceControl.numMemory,
                lastAdjust: 'gpu',
              });
            }
          }}></Input>
        </Form.Item>
        <Form.Item name="memory" label={`${intl.get('RESOURCEMODAL_MEMORY_LABEL')}: ${resourceControl.maxNumMemory}GB`} rules={[{ required: true }]}>
          <Input type="number" onBlur={(e) => {
            const memory = Number(e.target.value);
            if (memory > preNumMemory) {
              getAvailableResource({
                cpu: resourceControl.numCpu,
                gpu: resourceControl.numGpu,
                memory: memory,
                lastAdjust: 'memory',
              });
            }
          }}></Input>
        </Form.Item>
        <Form.Item name="priority" label={intl.get('RESOURCEBAR_PRIORITY')} rules={[{ required: true }]}>
          <Select>
            <Option value="1">1</Option>
            <Option value="2">2</Option>
            <Option value="3">3</Option>
            <Option value="4">4</Option>
            <Option value="5">5</Option>
          </Select>
        </Form.Item>
      </Form>
    </Modal>
  )
}