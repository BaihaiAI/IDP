import { manageApiPath, noteApiPath2 } from './httpClient';
import { userId, teamId, projectId } from '../store/cookie';
import request from "./request.js"
// todo
function jobUpdate(options) {
  const url = options.jobId === 0 ? `${manageApiPath}/pipeline/job/new` : `${manageApiPath}/pipeline/job/update`;
  const data = {...options, userId, projectId,teamId}
  return request.post(url, data);
}

function jobFetch(options) {
  const url = `${manageApiPath}/pipeline/job/getJobById?jobId=${options.jobId}&projectId=${projectId}`;
  return request.get(url);
}

function jobCreatePlan(options) {
  const url = `${manageApiPath}/pipeline/job/updateSchedule`;
  const data = {...options, projectId,teamId}
  return request.post(url, data);
}

function jobRunOnce(options) {
  const url = `${manageApiPath}/pipeline/job/runOnce`;
  return request.post(url, { jobId: options.jobId, projectId,teamId });
}

function jobInstanceGet(options) {
  const url = `${manageApiPath}/pipeline/jobinstance/getJobInstanceById?id=${options.jobInstanceId}&userId=${userId}&projectId=${projectId}`
  return request.get(url);
}

function jobInstanceKill(options) {
  const url = `${manageApiPath}/pipeline/jobinstance/killJobInstanceById`;
  return request.post(url, { id: options.jobInstanceId, projectId,teamId,userId});
}

function taskGetTaskPage(options) {
  const url = `${manageApiPath}/pipeline/task/getTaskPage?jobInstanceId=${options.jobInstanceId}&userId=${userId}&projectId=${projectId}&size=100`;
  return request.get(url);
}

function taskLog(options) {
  const { start, limit, jobId, instanceId, taskId, path } = options;
  const qs = `start=${start}&limit=${limit}&jobId=${jobId}&jobInstanceId=${instanceId}&taskInstanceId=${taskId}&path=${encodeURIComponent(path)}&teamId=${teamId}&projectId=${projectId}`

  const url = `${noteApiPath2}/pipeline/result?${qs}`;
  return request.get(url);
}

const pipelineApi = {
  jobUpdate,
  jobFetch,
  jobCreatePlan,
  jobRunOnce,
  jobInstanceGet,
  jobInstanceKill,
  taskGetTaskPage,
  taskLog,
};

export default pipelineApi;
