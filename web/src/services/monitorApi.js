import { noteApiPath2, kernelApiPath,manageApiPath } from './httpClient';
import { projectId, region, teamId } from '../store/cookie';
import request from "./request";

/**
 * workspace/pipeline
 */
function taskMonitorPage(options) {
  console.log(options)
  let url = `${manageApiPath}/admin-rs/dashboard/team/task-monitor-page?projectId=${projectId}&kernelSource=${options.kernelSource}&size=${1000}&current=${1}&jobInstanceId=${options.jobInstanceId}`;
  // 区分 pipeline逻辑
  if (options.sortField) url += `&sortField=${options.sortField}&sort=${options.sort}`;
  return request.get(url);
}

/**
 * pipeline
 */
function pipelineJobinstancePage() {
    const url = `${noteApiPath2}/dashboard/pipeline-jobinstance-page?projectId=${projectId}&region=${region}`;
    return request.get(url);
}

// pie
function pieChartData() {
  const url = `${manageApiPath}/admin-rs/dashboard/team/task-monitor-total?teamId=${teamId}&projectId=${projectId}`;
  return request.get(url)
}

function taskMonitorAll() {
  const url = `${manageApiPath}/admin-rs/dashboard/team/task-monitor-list-all?projectId=${projectId}`;
  return request.get(url);
}

// 关闭所有空闲内核/所有内核
function shutdownAll(options) {
    let url = `${kernelApiPath}/kernel/shutdown_all?projectId=${projectId}`;
    if (options && options.hasOwnProperty("state")) url += `&state=${options.state}`;
    return request.get(url);
}

const monitorApi = {
    taskMonitorPage,
    pipelineJobinstancePage,
    pieChartData,
    taskMonitorAll,
    shutdownAll
};

export default monitorApi;
