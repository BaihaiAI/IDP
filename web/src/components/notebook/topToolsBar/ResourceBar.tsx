import { Input, Space, Button, Tooltip, Modal, message } from "antd"
import { useContext, useEffect, useImperativeHandle } from "react"
import { useState } from "react"
import kernelApi from "../../../services/kernelApi"
import { NotebookComponentContext } from "../Notebook"
import './resourceBar.less'
import clusterApi from "../../../services/clusterApi"
import { selectKernelList } from "../../../store/features/kernelSlice"
import { useDispatch, useSelector } from "react-redux"
import intl from "react-intl-universal"
import { resourceInfo } from '../../../services/resourceApi'
import { PoweroffOutlined } from "@ant-design/icons"
import { observer } from 'mobx-react';
import resourceControl from '../../../idp/global/resourceControl';

interface Props {
}

export const ResourceBar: React.FC<Props> = observer(() => {
  const [preNumCpu, setPreNumCpu] = useState(0.5);
  const [preNumGpu, setPreNumGpu] = useState(0);
  const [preNumMemory, setPreNumMemory] = useState(0.5);
  const [machineLoading, setMachineLoading] = useState(false);

  // 设置最大资源限制
  // const setMaxResource = () => {
  //   resourceInfo().then((res) => {
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
  //           setMaxNumGpu((Number(gpu) * 12).toFixed(0));
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

  // 获取可用资源
  const getAvailableResource = (options: any) => {
    // resourceControl.getAvailableResource(options, ({cpu, gpu, memory}) => {
    //   setPreNumCpu(cpu);
    //   setPreNumGpu(gpu);
    //   setPreNumMemory(memory);
    // });
  }

  // 获取机器运行状态
  const getRuntimeStatus = (success: Function) => {
    let timer = null;
    resourceControl.getRuntimeStatus((waitPending: boolean, machineStatus: string, pendingDuration: number) => {
      if (machineStatus === 'pending') {
        if (waitPending || pendingDuration < 12) {  // 选择了继续等待或者等待的时间小于12秒
          timer && clearTimeout(timer);
          timer = setTimeout(() => {
            getRuntimeStatus(success);
          }, 5000);
        } else {
          Modal.confirm({
            title: '申请的资源大于当前系统可用资源，您可以点击“继续等待”按钮等待系统分配资源或者点击“取消”按钮调整资源配置重新申请资源',
            maskClosable: false,
            keyboard: false,
            okText: '继续等待',
            cancelText: '取消',
            onOk: () => {
              resourceControl.setWaitPending(true);
              getRuntimeStatus(success);
            },
            onCancel: () => {
              setMachineLoading(true);
              resourceControl.setWaitPending(false);
              clusterApi.runtimeStop().then((res) => {
                setMachineLoading(false);
                resourceControl.setMachineStatus('stopped');
              }).catch((err) => {
                setMachineLoading(false);
              })
            }
          });
        }
      } else if (machineStatus === 'starting') {
        timer && clearTimeout(timer);
        timer = setTimeout(() => {
          getRuntimeStatus(success);
        }, 5000);
      } else if (machineStatus === 'running') {
        success && success();
      }
    });
  }

  useEffect(() => {
    getRuntimeStatus(null);
  }, []);

  const cpuProps = {
    type: 'number',
    prefix: <b>CPU:</b>,
    suffix: intl.get('RESOURCEBAR_CORE'),
    min: resourceControl.minNumCpu,
    max: resourceControl.maxNumCpu,
    // step: '0.1',
    value: resourceControl.numCpu,
    bordered: false,
    style: { width: '110px', borderBottom: 'gray 1px solid' },
    disabled: resourceControl.machineStatus !== 'stopped',
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => { 
      if (e.target.value.indexOf('-') !== -1) return; 
      resourceControl.setNumCpu(Number(e.target.value));
    },
    onBlur: (e: React.FocusEvent<HTMLInputElement>) => {
      const value = Number(e.target.value);
      if (value > preNumCpu) {
        getAvailableResource({
          cpu: value,
          lastAdjust: 'cpu',
          gpu: resourceControl.numGpu,
          memory: resourceControl.numMemory,
        });
      }
    },
    onPressEnter: (e) => {
      e.target.blur()
    }
  }
  const gpuProps = {
    type: 'number',
    prefix: <b>GPU:</b>,
    suffix: resourceControl.gpuUnit,
    min: resourceControl.minNumGpu,
    max: resourceControl.maxNumGpu,
    // step: '1',
    value: resourceControl.numGpu,
    bordered: false,
    style: { width: '110px', borderBottom: 'gray 1px solid' },
    disabled: resourceControl.machineStatus !== 'stopped',
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => {
      if (e.target.value.indexOf('-') !== -1) return; 
      resourceControl.setNumGpu(Number(e.target.value));
    },
    onBlur: (e: React.FocusEvent<HTMLInputElement>) => {
      const value = Number(e.target.value);
      if (value > preNumGpu) {
        getAvailableResource({
          gpu: value,
          lastAdjust: "gpu",
          cpu: resourceControl.numCpu,
          memory: resourceControl.numMemory,
        });
      }
    },
    onPressEnter: (e) => {
      e.target.blur()
    }
  }
  const memoryProps = {
    type: 'number',
    prefix: <b>{intl.get('RESOURCEBAR_MEMORY')}:</b>,
    suffix: 'GB',
    min: resourceControl.minNumMemory,
    max: resourceControl.maxNumMemory,
    // step: '0.1',
    value: resourceControl.numMemory,
    bordered: false,
    style: { width: '110px', borderBottom: 'gray 1px solid' },
    disabled: resourceControl.machineStatus !== 'stopped',
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => { 
      if (e.target.value.indexOf('-') !== -1) return; 
      resourceControl.setNumMemory(Number(e.target.value));
    },
    onBlur: (e: React.FocusEvent<HTMLInputElement>) => {
      const value = Number(e.target.value);
      if (value > preNumMemory) {
        getAvailableResource({
          memory: value,
          lastAdjust: 'memory',
          gpu: resourceControl.numGpu,
          cpu: resourceControl.numCpu,
        });
      }
    },
    onPressEnter: (e) => {
      e.target.blur()
    }
  }
  const priorityProps = {
    type: 'number',
    prefix: <b>{intl.get('RESOURCEBAR_PRIORITY')}:</b>,
    min: resourceControl.minPriority,
    max: resourceControl.maxPriority,
    // step: '1',
    value: resourceControl.priority,
    bordered: false,
    style: { width: '85px', borderBottom: 'gray 1px solid' },
    disabled: resourceControl.machineStatus !== 'stopped',
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => { resourceControl.setPriority(Number(e.target.value)) },
    onBlur: (e: React.FocusEvent<HTMLInputElement>) => {
      const value = Number(e.target.value);
      if (value <= 0) {
        resourceControl.setPriority(1);
      } else if (value > 5) {
        resourceControl.setPriority(5);
      }
    },
    onPressEnter: (e) => {
      e.target.blur()
    }
  }

  const startupMachine = () => {
    if (resourceControl.numCpu < resourceControl.minNumCpu
      || resourceControl.numGpu < resourceControl.minNumGpu
      || resourceControl.numMemory < resourceControl.minNumMemory) {
      Modal.error({
        title: `当前配置资源少于运行的最低资源要求，CPU至少${resourceControl.minNumCpu}${intl.get('RESOURCEBAR_CORE')}，内存至少${resourceControl.minNumMemory}GB，请重新设置后再申请。`,
      });
    } else {
      Modal.info({
        title: `${intl.get('RESOURCEBAR_STARTUP_INFO')} CPU: ${resourceControl.numCpu}${intl.get('RESOURCEBAR_CORE')}, GPU: ${resourceControl.numGpu}${resourceControl.gpuUnit}, 内存: ${resourceControl.numMemory}GB`,
        closable: true,
        okText: intl.get('OK'),
        onOk: () => {
          setMachineLoading(true);
          clusterApi.runtimeStart({
            memory: resourceControl.numMemory,
            numCpu: resourceControl.numCpu,
            numGpu: resourceControl.numGpu,
            priority: resourceControl.priority
          }).then((res) => {
            setMachineLoading(false);
            getRuntimeStatus(() => {
              message.success('资源申请成功！');
            });
          }).catch((err) => {
            setMachineLoading(false);
          })
        }
      });
    }
  }
  const shutdownMachine = () => {
    Modal.warning({
      title: intl.get('RESOURCEBAR_SHUTDOWN_INFO'),
      closable: true,
      okText: intl.get('OK'),
      onOk: () => {
        setMachineLoading(true);
        resourceControl.setWaitPending(false);
        clusterApi.runtimeStop().then((res) => {
          setMachineLoading(false);
          resourceControl.setMachineStatus('stopped');
        }).catch((err) => {
          setMachineLoading(false);
        })
      }
    });
  }
  const shutdownMachineWithoutModal = () => {
    setMachineLoading(true);
    clusterApi.runtimeStop().then((res) => {
      setMachineLoading(false);
      resourceControl.setMachineStatus('stopped');
      resourceControl.setWaitPending(false);
    }).catch((err) => {
      setMachineLoading(false);
      resourceControl.setWaitPending(false);
    })
  }

  const getMachineButton = () => {
    switch (resourceControl.machineStatus) {
      case 'stopped':
        return (<Button type="text" size="small" onClick={startupMachine} loading={machineLoading} className="machine-button">
          <Tooltip placement="bottom" title={intl.get('RESOURCEBAR_STARTUP')}>
            <PoweroffOutlined style={{ fontSize: 18, color: 'red' }} />
          </Tooltip>
        </Button>);
      case 'running':
        return (<Button type="link" size="small" onClick={shutdownMachine} loading={machineLoading} className="machine-button">
          <Tooltip placement="bottom" title={intl.get('RESOURCEBAR_SHUTDOWN')}>
            <PoweroffOutlined style={{ fontSize: 18, color: '#43B02A' }} />
          </Tooltip>
        </Button>);
      case 'pending':
        return (<Button type="link" size="small" onClick={shutdownMachineWithoutModal} loading={machineLoading} className="machine-button">
          <Tooltip placement="bottom" title={intl.get('RESOURCEBAR_PENDING')}>
            <PoweroffOutlined style={{ fontSize: 18, color: '#FAAD14' }} />
          </Tooltip>
        </Button>);   
      default:
        return (<Button type="link" size="small" onClick={shutdownMachineWithoutModal} loading={machineLoading} className="machine-button">
          <Tooltip placement="bottom" title={intl.get('RESOURCEBAR_STARTING')}>
            <PoweroffOutlined style={{ fontSize: 18, color: '#FAAD14' }} />
          </Tooltip>
        </Button>); 
    }
  }

  if (Boolean(process.env.NODE_OPEN)) {
    return (<></>);
  } else {
    return (<div className="resourceBar">
      <Space>
        <Input size="small" {...cpuProps} />
        <Input size="small" {...gpuProps} />
        <Input size="small" {...memoryProps} />
        <Input size="small" {...priorityProps} />
        {getMachineButton()}
      </Space>
    </div>);
  }
});