import { message } from 'antd';
import { observable, action } from 'mobx';
import intl from "react-intl-universal"
import clusterApi from '../../../services/clusterApi';

class ResourceControl {
  @observable machineStatus = 'stopped';
  @observable waitPending = false;

  @observable numCpu = 0.5;
  @observable numGpu = 0;
  @observable numMemory = 1.0;
  @observable priority = 3;
  @observable gpuUnit = intl.get('RESOURCEBAR_CARD');

  @observable minNumCpu = 0.5;
  @observable minNumGpu = 0;
  @observable minNumMemory = 1.0;
  @observable minPriority = 1;

  @observable maxNumCpu = 4;
  @observable maxNumGpu = 0;
  @observable maxNumMemory = 16;
  @observable maxPriority = 5;

  @action
  setMachineStatus = (value: string) => {
    this.machineStatus = value;
  }
  @action
  setWaitPending = (value: boolean) => {
    this.waitPending = value;
  }
  @action
  setNumCpu = (value: number) => {
    this.numCpu = value;
  }
  @action
  setNumGpu = (value: number) => {
    this.numGpu = value;
  }
  @action
  setNumMemory = (value: number) => {
    this.numMemory = value;
  }
  @action
  setPriority = (value: number) => {
    this.priority = value;
  }
  @action
  setGpuUnit = (value: string) => {
    this.gpuUnit = value;
  }
  @action
  setMaxNumCpu = (value: number) => {
    this.maxNumCpu = value;
  }
  @action
  setMaxNumGpu = (value: number) => {
    this.maxNumGpu = value;
  }
  @action
  setMaxNumMemory = (value: number) => {
    this.maxNumMemory = value;
  }

  // 获取资源可用状态
  @action
  getRuntimeStatus = (cb: Function) => {
    const _this = this;
    clusterApi.runtimeStatus({ waitPending: _this.waitPending }).then((res) => {
      const { resource, running, status, waitPending, pendingDuration } = res.data;
      const { numCpu, numGpu, memory, priority } = resource;
      if (numCpu !== undefined) _this.numCpu = numCpu;
      if (numGpu !== undefined) _this.numGpu = numGpu;
      if (memory !== undefined) _this.numMemory = memory;
      if (priority !== undefined) _this.priority = priority;
      if (waitPending !== undefined) _this.waitPending = waitPending;
      
      switch (status) {
        case 'PodNotFound':
          _this.machineStatus = 'stopped';
          break;
        case 'Running':
          _this.machineStatus = 'running';
          break;
        case 'Pending':
          _this.machineStatus = 'pending';
          break;
        case 'Creating':
          _this.machineStatus = 'starting';
          break;
        default:
          break;
      }

      if (cb) cb(_this.waitPending, _this.machineStatus, pendingDuration);
    })
  }
  
  // 获取可用资源
  @action
  getAvailableResource = (options: any, cb: Function) => {
    const _this = this;
    clusterApi.suggestV3(options).then((res) => {
      const { maxResource, suggestionResource } = res.data;
      const { cpu, gpu, gpu_unit, memory } = suggestionResource;
      _this.gpuUnit = gpu_unit === 'mem' ? 'GB' : intl.get('RESOURCEBAR_CARD');
      _this.maxNumCpu = maxResource.cpu;
      _this.maxNumGpu = maxResource.gpu;
      _this.maxNumMemory = maxResource.memory;

      _this.numCpu = Math.floor(cpu * 10) / 10;
      _this.numGpu = Math.floor(gpu * 10) / 10;
      _this.numMemory = Math.floor(memory * 10) / 10;

      if (_this.numCpu < _this.minNumCpu || _this.numGpu < _this.minNumGpu || _this.numMemory < _this.minNumMemory) {
        message.warning(`当前系统可用资源少于最低资源要求，cpu: ${_this.minNumCpu} gpu: ${_this.minNumGpu} memory: ${_this.minNumMemory}`);
      }

      if (cb) cb({
        cpu: _this.numCpu, 
        gpu: _this.numGpu, 
        memory: _this.numMemory
      });
    });
  }
}

export default new ResourceControl();