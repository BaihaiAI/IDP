import { Input, Space, Checkbox } from "antd"
import type { CheckboxChangeEvent } from 'antd/es/checkbox';
import { useContext, useEffect, useImperativeHandle } from "react"
import { useState } from "react"
import kernelApi from "../../../services/kernelApi"
import { NotebookComponentContext } from "../Notebook"
import './resourceBar.less'
import clusterApi from "../../../services/clusterApi"
import { selectKernelList } from "../../../store/features/kernelSlice"
import { useDispatch, useSelector } from "react-redux"

interface Props {
  resourceRef: any
}

export const ResourceBar: React.FC<Props> = ({ resourceRef }) => {
  const minNumCpu = '0.5'
  const minNumGpu = '0.0'
  const minMemory = '0.5'
  const minPriority = '1'
  const maxPriority = '5'
  const [numCpu, setNumCpu] = useState(minNumCpu)
  const [numGpu, setNumGpu] = useState(minNumGpu)
  const [memory, setMemory] = useState(minMemory)
  const [priority, setPriority] = useState('3')
  const [maxNumCpu, setMaxNumCpu] = useState('16.0')
  const [maxNumGpu, setMaxNumGpu] = useState('22.0')
  const [maxMemory, setMaxMemory] = useState('32.0')
  const [isExecuting, setIsExecuting] = useState(false)
  const [enableSaveSession, setEnableSaveSession] = useState(false)
  const notebook: any = useContext(NotebookComponentContext)

  const kernelList = useSelector(selectKernelList)

  // 获取可用资源
  const getAvailableResource = (callback: any) => {
    clusterApi.suggest({ path: notebook.path }).then(res => {
      // console.log(res.data)
      const availableCpu = res.data.CPU || 0
      const availableGpu = res.data.GPU || 0
      const availableMemory = res.data.memory || 0
      const priority = res.data.priority || '3'
      // setMaxNumCpu(availableCpu.toFixed(1))
      // setMaxNumGpu(availableGpu.toFixed(1))
      // setMaxMemory((availableMemory / (1024 * 1024 * 1024)).toFixed(1))

      callback && callback({
        availableCpu,
        availableGpu,
        availableMemory: availableMemory / (1024 * 1024 * 1024),
        priority,
      })
    }).catch(err => {
      console.log(err)
    })
  }

  useEffect(() => {
    if (notebook.path) {
      getAvailableResource((available: any) => {
        const { availableCpu, availableGpu, availableMemory, priority } = available
        setNumCpu(availableCpu.toFixed(2))
        setNumGpu(availableGpu.toFixed(2))
        setMemory(availableMemory.toFixed(2))
        setPriority(priority)
      })
    }
  }, [notebook.path])

  useEffect(() => {
    kernelApi.kernelState().then((response) => {
      for (const kernel of response.data) {
        if (kernel.notebookPath === notebook.path) {
          setIsExecuting(true)
          break
        }
      }
    })
  }, [notebook.path])

  // useEffect(() => {
  //   let resourceTimer = null
  //   let priorityTimer = null
  //   if (isExecuting) {
  //     // 更新优先级
  //     if (!priorityTimer) {
  //       // priorityTimer = setInterval(() => {

  //       // }, 5000)
  //     }

  //     if (resourceTimer) {
  //       resourceTimer = null
  //       clearInterval(resourceTimer)
  //     }
  //   } else {
  //     // 更新可用资源
  //     if (!resourceTimer) {
  //       resourceTimer = setInterval(() => {
  //         getAvailableResource((available: any) => {
  //           const { availableCpu, availableGpu, availableMemory } = available
  //           setNumCpu(Math.min(availableCpu, Number(numCpu)).toString())
  //           setNumGpu(Math.min(availableGpu, Number(numGpu)).toString())
  //           setMemory(Math.min(availableMemory, Number(memory)).toFixed(1))
  //         })
  //       }, 5000)
  //     }
  //   }

  //   return () => {
  //     // 清除计时器
  //     resourceTimer && clearInterval(resourceTimer)
  //     resourceTimer = null
  //     priorityTimer && clearInterval(priorityTimer)
  //     priorityTimer = null
  //   }
  // }, [isExecuting, numCpu, numGpu, memory])

  useEffect(() => {
    if (notebook.path in kernelList) {
      !isExecuting && setIsExecuting(true)
    } else {
      isExecuting && setIsExecuting(false)
    }
  }, [kernelList])

  const cpuProps = {
    type: 'number',
    addonBefore: 'CPU',
    addonAfter: '核',
    min: minNumCpu,
    max: maxNumCpu,
    step: '0.1',
    value: numCpu,
    style: { width: '130px' },
    disabled: isExecuting,
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => { 
      setNumCpu(e.target.value)
    },
    onBlur: (e: React.FocusEvent<HTMLInputElement>) => {
      const value = e.target.value
      if (value === '' || Number(value) <= 0) {
        setNumCpu(minNumCpu)
      } else if (Number(value) > Number(maxNumCpu)) {
        setNumCpu(maxNumCpu)
      }
    }
  }
  const gpuProps = {
    type: 'number',
    addonBefore: 'GPU',
    addonAfter: 'GB',
    min: minNumGpu,
    max: maxNumGpu,
    step: '1',
    value: numGpu,
    style: { width: '130px' },
    disabled: isExecuting,
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => { 
      setNumGpu(e.target.value)
    },
    onBlur: (e: React.FocusEvent<HTMLInputElement>) => {
      const value = e.target.value
      if (value === '' || Number(value) <= 0) {
        setNumGpu(minNumGpu)
      } else if (Number(value) > Number(maxNumGpu)) {
        setNumGpu(maxNumGpu)
      }
    }
  }
  const memoryProps = {
    type: 'number',
    addonBefore: '内存',
    addonAfter: 'GB',
    min: minMemory,
    max: maxMemory,
    step: '0.1',
    value: memory,
    style: { width: '130px' },
    disabled: isExecuting,
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => { 
      setMemory(e.target.value)
    },
    onBlur: (e: React.FocusEvent<HTMLInputElement>) => {
      const value = e.target.value
      if (value === '' || Number(value) <= 0) {
        setMemory(minMemory)
      } else if (Number(value) > Number(maxMemory)) {
        setMemory(maxMemory)
      }
    }
  }
  const priorityProps = {
    type: 'number',
    addonBefore: '优先级',
    min: minPriority,
    max: maxPriority,
    step: '1',
    value: priority,
    style: { width: '100px' },
    disabled: isExecuting,
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => { setPriority(e.target.value) },
    onBlur: (e: React.FocusEvent<HTMLInputElement>) => {
      const value = e.target.value
      if (value === '' || Number(value) <= 0) {
        setPriority('1')
      } else if (Number(value) > 5) {
        setPriority('5')
      }
    }
  }

  const getResource = () => {
    return {
      numCpu: Number(numCpu),
      numGpu: Number(numGpu),
      memory: Number(memory),
      priority: Number(priority),
    }
  }
  const setKernelIsExecuting = (isExecuting: boolean) => {
    setIsExecuting(isExecuting)
  }
  useImperativeHandle(resourceRef, () => ({ getResource, setKernelIsExecuting, getEnableSaveSession }))

  const autoCheckpoint = (e: CheckboxChangeEvent) => {
    setEnableSaveSession(e.target.checked)
  }
  const getEnableSaveSession = () => {
    return enableSaveSession
  }

  return (<div className="resourceBar">
    <Space>
      <Input size="small" {...cpuProps} />
      <Input size="small" {...gpuProps} />
      <Input size="small" {...memoryProps} />
      <Input size="small" {...priorityProps} />
      <Checkbox onChange={autoCheckpoint}>自动保存上下文</Checkbox>
    </Space>
  </div>)
}