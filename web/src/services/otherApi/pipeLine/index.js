import request from "../../request"
import { projectId, teamId, userId } from "../../../store/cookie"
import { manageApiPath } from "../../httpClient"

const getJobList = (data) => {
  Object.keys(data).forEach((key) => {
    if (!data[key]) {
      delete data[key]
    }
  })
  return request.get(`${manageApiPath}/pipeline/job/getJobPage`, {
    params: {
      ...data,
      userId,
      teamId,
      projectId,
    },
  })
}

const getJobInstanceList = (data) => {
  Object.keys(data).forEach((key) => {
    if (!data[key]) {
      delete data[key]
    }
  })

  return request.get(
    `${manageApiPath}/pipeline/jobinstance/getJobInstancePage`,
    {
      params: {
        ...data,
        userId,
        teamId,
        projectId,
      },
    }
  )
}

const runJob = (jobId) => {
  return request.post(`${manageApiPath}/pipeline/job/runOnce`, {
    jobId,
    userId,
    teamId,
    projectId,
  })
}

const deleteJob = (jobId) => {
  return request.post(`${manageApiPath}/pipeline/job/delete`, {
    jobId,
    projectId,
    userId,
    teamId,
  })
}

const cloneJob = (jobId) => {
  return request.post(`${manageApiPath}/pipeline/job/clone`, {
    jobId,
    projectId,
    userId,
    teamId,
  })
}

const getTaskPageList = (data) => {
  return request.get(`${manageApiPath}/pipeline/task/getTaskPage`, {
    params: {
      ...data,
      projectId,
      userId,
      teamId,
    },
  })
}

const killJobInstanceById = (id) => {
  return request.post(
    `${manageApiPath}/pipeline/jobinstance/killJobInstanceById`,
    { id, projectId, userId, teamId }
  )
}

const pipeLineApi = {
  getJobList,
  getJobInstanceList,
  runJob,
  deleteJob,
  cloneJob,
  getTaskPageList,
  killJobInstanceById,
}

export default pipeLineApi
