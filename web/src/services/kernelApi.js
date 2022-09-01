import {kernelApiPath, noteApiPath2} from './httpClient'
import { projectId, teamId } from '@/store/cookie';
import request from "./request"

// function executeCell(options) {
//   const url = `${noteApiPath}/execute/cell`;
//   const data = {
//     projectId: projectId,
//     session: options.session,
//     path: options.path,
//     cellId: options.cellId,
//     cellType: options.cellType,
//     code: options.code,
//     kernel: options.kernel,
//     identity: options.identity,
//   };
//   return post(url, data);
// }

// function executeInfo(options) {
//   const url = `${noteApiPath}/execute/info`;
//   const data = {
//     projectId: projectId,
//     session: options.session,
//     kernel: options.kernel,
//     identity: options.identity,
//   };
//   return post(url, data);
// }

function executeInterrupt(options) {
  const url = `${kernelApiPath}/kernel/interrupt?inode=${options.inode}`;
  // const data = {
  //   projectId: projectId,
  //   session: options.session,
  //   kernel: options.kernel,
  //   identity: options.identity,
  //   inode: options.inode,
  //   batchId: options.batchId,
  //   path: options.path,
  // };
  return request.get(url)
}

function executeState(options) {
  const url = `${kernelApiPath}/notebook/cell_state?path=${encodeURIComponent(options.path)}&projectId=${projectId}`;
  return request.get(url);
}

/*
function kernelNew(options) {
  const url = `${noteApiPath}/kernel/new?name=${options.name}&path=${options.path}&projectId=${projectId}`;
  return get(url);
}

function list() {
  const url = `${noteApiPath}/kernel/list?projectId=${projectId}`;
  return get(url);
}
*/

function shutdown(options) {
  const { name, identity, inode, path } = options;
  const url = `${kernelApiPath}/kernel/shutdown`;
  const data = {
    name: name,
    identity: identity,
    inode: inode,
    path: path,
    restart: false,
    teamId: teamId,
    projectId: projectId,
  }
  return request.post(url, data);
}

function restart(options) {
  const { name, identity, inode, path, numCpu, numGpu, memory, priority } = options;
  const url = `${kernelApiPath}/kernel/shutdown`;
  const data = {
    name: name,
    identity: identity,
    inode: inode,
    path: path,
    restart: true,
    teamId: teamId,
    projectId: projectId,
    resource: {
      numCpu: numCpu,
      numGpu: numGpu,
      memory: memory,
      priority: priority,
    }
  }
  return request.post(url, data)
}

function kernelState() {
  // const url = `${noteApiPath2}/state/kernel-list?projectId=${projectId}`;
  const url = `${kernelApiPath}/kernel/list?projectId=${projectId}`;
  return request.get(url);
}

// 恢复kernel
function kernelResume(options) {
  const qs = `inode=${options.inode}&path=${encodeURIComponent(options.path)}&projectId=${projectId}`;
  const url = `${kernelApiPath}/kernel/resume?${qs}`;
  return request.get(url);
}

// 挂起kernel
function kernelPause(options) {
  const qs = `name=${options.name}&identity=${options.identity}&inode=${options.inode}&path=${encodeURIComponent(options.path)}&projectId=${projectId}`;
  const url = `${kernelApiPath}/kernel/pause?${qs}`;
  return request.get(url);
}

const kernelApi = {
  // executeCell,
  // executeInfo,
  executeInterrupt,
  executeState,
  // kernelNew,
  // list,
  shutdown,
  restart,
  kernelState,
  kernelResume,
  kernelPause,
};

export default kernelApi;
