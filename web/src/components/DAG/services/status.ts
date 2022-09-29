// import cloneDeep from 'lodash/cloneDeep'

import pipelineApi from 'idpServices/pipelineApi';
import { formatDateTime, formatCostTime } from './lib'

let state = {
  idx: 0,
  running: false,
  statusRes: {
    lang: 'zh_CN',
    success: true,
    data: {
      instStatus: {
        '10571193': 'success',
        '10571194': 'success',
        '10571195': 'success',
        '10571196': 'success',
        '10571197': 'success',
      },
      execInfo: {
        '10571193': {
          jobStatus: 'success',
          defName: '读数据表',
          name: 'germany_credit_data',
          id: 10571193,
        },
        '10571194': {
          jobStatus: 'success',
          defName: '离散值特征分析',
          name: '离散值特征分析',
          id: 10571194,
        },
        '10571195': {
          jobStatus: 'success',
          defName: '分箱',
          startTime: '2020-10-19 13:28:55',
          endTime: '2020-10-19 13:30:20',
          name: '分箱',
          id: 10571195,
        },
        '10571196': {
          jobStatus: 'success',
          defName: '评分卡训练',
          startTime: '2020-10-19 13:28:55',
          endTime: '2020-10-19 13:32:02',
          name: '评分卡训练-1',
          id: 10571196,
        },
      },
      status: 'default',
    },
    Lang: 'zh_CN',
  } as any,
}

const transformStatus = (status) => {
  switch (status) {
    case 'Init':
      return 'waiting'
    case 'Pending':
      return 'waiting'
    case 'Running':
      return 'running'
    case 'Success':
      return 'success'
    case 'Kill':
      return 'fail'
    case 'Fail':
      return 'fail'
    default:
      return 'ready'
  }
}

const statusName = {
  waiting: '等待中',
  running: '运行中',
  success: '成功',
  fail: '失败',
}

const formatState = (taskInstanceList) => {
  let instStatus = {};
  let execInfo = {};
  let running = false;
  let success = true;
  for (const taskInstance of taskInstanceList) {
    const taskId = `${taskInstance.taskId}`;
    const taskInstanceStatus = transformStatus(taskInstance.status);
    const taskResult = taskInstance.stdErr ? taskInstance.stdErr.stdDesc.display : statusName[taskInstanceStatus];
    instStatus[taskId] = taskInstanceStatus;
    execInfo[taskId] = {
      jobStatus: taskInstanceStatus,
      jobResult: taskResult,
      defName: taskInstance.script,
      name: taskInstance.taskName,
      id: taskInstance.taskId,
      taskInstanceId: taskInstance.taskInstanceId,
      startTime: formatDateTime(taskInstance.startTime),
      endTime: formatDateTime(taskInstance.endTime),
      timeCost: formatCostTime(taskInstance.timeCost),
      machine: taskInstance.resource.machine,
    };
    running = running || taskInstanceStatus === 'running' || taskInstanceStatus==='waiting';
    success = success && taskInstanceStatus !== 'fail';
  }
  return {
    lang: 'zh_CN',
    success,
    data: {
      instStatus,
      execInfo,
      status: running ? 'running' : 'done',
    },
    Lang: 'zh_CN',
  } as any;
}

export const runGraph = async (jobId: string) => {
  let jobInstanceId = null;
  await pipelineApi.jobRunOnce({ jobId: jobId }).then((res) => {
    jobInstanceId = `${res.data[0].id}`;
  });
  return jobInstanceId;
}

export const stopGraphRun = async (jobInstanceId: string) => {
  let success = false;
  await pipelineApi.jobInstanceKill({ jobInstanceId }).then(() => {
    success = true
  }).catch((err) => {
    console.log(err);
  })
  return { success }
}

// const getStatus = () => cloneDeep(state.statusRes)

export const queryGraphStatus = async (experimentInstanceId: string) => {
  let newState = null;
  await pipelineApi.taskGetTaskPage({ jobInstanceId: experimentInstanceId }).then((res) => {
    newState = formatState(res.data.records);
  });
  return newState;
}
