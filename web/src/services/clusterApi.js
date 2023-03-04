import { clusterPath, noteApiPath2, volcanoApiPath } from "./httpClient";
import { projectId, teamId } from '../store/cookie';
import request from "./request"

const suggest = ({ path }) => {
  if (!Boolean(process.env.NODE_OPEN)) {
    const url = `${clusterPath}/suggest?teamId=${teamId}&projectId=${projectId}&path=${path}`
    return request.get(url)
  } else {
    return Promise.resolve()
  }
}

const suggestV2 = (options) => {
  const { cpu, keepCpu, memory, keepMemory, gpu, keepGpu } = options;
  const url = `${clusterPath}/suggest_new?projectId=${projectId}&cpu=${cpu}&keepCpu=${keepCpu}&memory=${memory}&keepMemory=${keepMemory}&gpu=${gpu}&keepGpu=${keepGpu}`;
  return request.get(url)
}

const suggestV3 = (options) => {
  const { cpu, gpu, memory, lastAdjust } = options;
  const url = `${volcanoApiPath}/suggest?cpu=${cpu}&gpu=${gpu}&memory=${memory}&lastAdjust=${lastAdjust}`;
  return request.get(url);
}

const runtimeStatus = (options) => {
  const { waitPending } = options;
  return request.get(`${noteApiPath2}/runtime/status?projectId=${projectId}&waitPending=${waitPending}`);
}

const runtimeStart = ({ memory, numCpu, numGpu, priority }) => {
  return request.post(`${clusterPath}/runtime/start`, {
    projectId,
    resource: {
      memory,
      numCpu,
      numGpu,
      priority
    }
  });
}

const runtimeStop = () => {
  return request.post(`${clusterPath}/runtime/stop`, {
    projectId
  });
}

const clusterApi = {
  suggest,
  suggestV2,
  suggestV3,
  runtimeStatus,
  runtimeStart,
  runtimeStop,
}

export default clusterApi
