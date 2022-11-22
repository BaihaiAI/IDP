import random from 'lodash/random'
import pipelineApi from 'idpServices/pipelineApi'
// import {userId} from 'src/store/cookie'
import { NExperiment } from '../pages/rx-models/typing';
import { formatDateTime, formatCostTime } from './lib'
import { getCurrentEnv } from '../../../store/config';

interface NodeParams {
  id: string
  name: string
  x: number
  y: number
}

const jobToExperiment = (job: any) => {
  return {
    projectName: job.jobName,
    gmtCreate: job.createtime,
    description: job.jobName,
    name: job.jobName,
    id: job.jobId,
    taskParallel: job.config.taskParallel,
    maxRetryTimes: job.config.maxRetryTimes,
    envName: job.config.envName,
    priority: job.config.priority,
    startTime: formatDateTime(job.startTime),
    endTime: formatDateTime(job.endTime),
    timeCost: formatCostTime(job.timeCost),
    status: job.status,
  }
}

const experimentToJob = (experiment: NExperiment.Experiment) => {
  return {
    // jobSuperid: userId,
    jobName: experiment.name,
    jobId: experiment.id,
    jobType: 1,
    config: {
      taskParallel: experiment.taskParallel,
      maxRetryTimes: experiment.maxRetryTimes,
      envName: experiment.envName,
      priority: Number(experiment.priority),
    }
  };
}

const graphToTask = ({ nodes, links }) => {
  let edgeObj = {};
  for (const link of links) {
    let targetList = edgeObj[link.source];
    if (targetList) {
      targetList.push(link.target);
    } else {
      targetList = [link.target];
    }
    edgeObj[link.source] = targetList;
  }
  let tasks = [];
  for (const node of nodes) {
    const id = node.id;
    const targetList = edgeObj[id];
    let taskEdge = [];
    if (targetList) {
      for (const target of targetList) {
        const edge = {
          toTask: Number(target),
          fromTask: Number(id),
        };
        taskEdge.push(edge);
      }
    }
    const task = {
      script: node.script,
      resource: {
        machine: node.machine,
        env: '',
        envName: node.envName,
        taskResourceType: 1,
        priority: Number(node.priority),
      },
      taskEdge: taskEdge,
      taskName: node.name,
      taskId: Number(id),
      exInfo: {
        positionX: node.positionX,
        positionY: node.positionY,
      },
      config: {
        preNodeRelation: node.preNodeRelation,
        maxRetryTimes: 2
      },
      taskType: 1,
    };
    tasks.push(task);
  }
  return tasks;
}

const taskToGraph = (tasks) => {
  let nodes = [];
  let links = [];
  for (const item of tasks) {
    const taskId = item.taskId;
    const node = {
      id: `${taskId}`,
      name: item.taskName,
      script: item.script,
      inPorts: [
        {
          sequence: 1,
          id: `${taskId}_in_1`,
        },
      ],
      outPorts: [
        {
          sequence: 1,
          id: `${taskId}_out_1`,
        },
      ],
      envName: item.resource.envName,
      machine: item.resource.machine,
      preNodeRelation: item.config.preNodeRelation,
      priority: item.resource.priority,
      positionX: item.exInfo.positionX,
      positionY: item.exInfo.positionY,
      status: 3,
    }
    nodes.push(node);

    for (const edge of item.taskEdge) {
      const link = {
        source: `${edge.fromTask}`,
        target: `${edge.toTask}`,
        outputPortId: `${edge.fromTask}_out_1`,
        inputPortId: `${edge.toTask}_in_1`,
      }
      links.push(link);
    }
  }
  return {
    nodes: nodes,
    links: links,
  };
}

export const copyNode = (node: any) => {
  console.log(node);
  const id = `${Date.now()}`
  return {
    id: id,
    name: node.name,
    inPorts: [
      {
        sequence: 1,
        id: id + '_in_1',
      },
    ],
    outPorts: [
      {
        sequence: 1,
        id: id + '_out_1',
      },
    ],
    script: node.script,
    machine: node.machine,
    preNodeRelation: node.preNodeRelation,
    priority: node.priority,
    positionX: node.positionX + 200 + random(20, false),
    positionY: node.positionY + random(10, false),
    status: 3,
  }
}
export const addNode = (nodeMeta: NodeParams) => {
  const { name, x, y } = nodeMeta;
  const id = `${Date.now()}`
  return {
    id,
    name,
    inPorts: [
      {
        sequence: 1,
        id: id + '_in_1',
      },
    ],
    outPorts: [
      {
        sequence: 1,
        id: id + '_out_1',
      },
    ],
    script: nodeMeta.id,
    machine: '0.5vCPUs 0.5GB',
    preNodeRelation: 'ALL_SUCCESS',
    priority: 3,
    positionX: x,
    positionY: y,
    status: 3,
  }
}

const defaultExperiment = () => {
  return {
    experiment: {
      projectName: '',
      gmtCreate: '',
      description: '未命名工作流',
      name: '未命名工作流',
      id: '0_' + (new Date().getTime()),
      taskParallel: 5,
      maxRetryTimes: 2,
      envName: getCurrentEnv() || '',
      priority: 3,
    },
    graph: {
      nodes: [],
      links: [],
    }
  }
}

export const queryExperiment = async (id: string) => {
  // if ('0' === id) return defaultExperiment();
  if (id.startsWith('0_')) return defaultExperiment();
  let experiment = {};
  let graph = {};
  let status = '';
  let cronConfig = {}
  await pipelineApi.jobFetch({ jobId: id }).then((res) => {
    experiment = jobToExperiment(res.data);
    graph = taskToGraph(res.data.tasks);
    status = res.data.status;
    cronConfig = res.data.cronConfig
  }).catch((err) => {
    console.log(err);
  });
  return {
    experiment,
    graph,
    status,
    cronConfig,
  };
}

export const queryExperimentInstance = async (instanceId: string) => {
  let experiment = {};
  let graph = {};
  let status = '';
  await pipelineApi.jobInstanceGet({ jobInstanceId: instanceId }).then((res) => {
    const jobInstance = res.data;
    experiment = jobToExperiment(jobInstance);
    graph = taskToGraph(jobInstance.tasks);
    status = jobInstance.status;
  }).catch((err) => {
    console.log(err);
  });
  return {
    experiment,
    graph,
    status,
  };
}

export const saveExperiment = async (experiment: any, graph: any) => {
  console.log(experiment);
  console.log(graph);
  const job = experimentToJob(experiment);
  const tasks = graphToTask(graph);
  const options = {...job, tasks };
  return pipelineApi.jobUpdate(options);
}

export const createPlan = async (experiment: any, graph: any, plan: any) => {
  // const job = experimentToJob(experiment);
  // const tasks = graphToTask(graph);
  pipelineApi.jobCreatePlan({
    experiment: experiment.id,
    status: 'Schedule',
    cronConfig:{...plan}
  });
}

export const addNodeGroup = async (groupName: string) => {
  return {
    success: true,
    data: {
      group: {
        name: groupName,
        id: Date.now(),
      },
    },
  }
}

const initData = {
  nodes: [
    {
      id: '1603716783816',
      name: 'newbility.ipynb',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716783816_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716783816_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716783816_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716783816_out_2',
        },
      ],
      positionX: -200,
      positionY: -300,
      // codeName: 'source_11111',
      // catId: 1,
      // nodeDefId: 111111,
      // category: 'source',
      status: 3,
      // groupId: 0,
    },
    {
      id: '1603716786205',
      name: '算法组件2',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716786205_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716786205_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716786205_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716786205_out_2',
        },
      ],
      positionX: -369,
      positionY: -161,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716788394',
      name: '算法组件2',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716788394_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716788394_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716788394_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716788394_out_2',
        },
      ],
      positionX: -154,
      positionY: -161,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716792826',
      name: '算法组件3',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716792826_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716792826_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716792826_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716792826_out_2',
        },
      ],
      positionX: -520,
      positionY: -30,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716795011',
      name: '算法组件2',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716795011_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716795011_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716795011_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716795011_out_2',
        },
      ],
      positionX: 74,
      positionY: -160,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716814719',
      name: '算法组件3',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716814719_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716814719_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716814719_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716814719_out_2',
        },
      ],
      positionX: -310,
      positionY: -30,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716822805',
      name: '算法组件3',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716822805_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716822805_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716822805_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716822805_out_2',
        },
      ],
      positionX: -50,
      positionY: -30,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716828657',
      name: '算法组件3',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716828657_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716828657_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716828657_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716828657_out_2',
        },
      ],
      positionX: 160,
      positionY: -30,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716834901',
      name: '算法组件2',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716834901_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716834901_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716834901_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716834901_out_2',
        },
      ],
      positionX: -390,
      positionY: 90,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716844054',
      name: '算法组件2',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716844054_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716844054_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716844054_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716844054_out_2',
        },
      ],
      positionX: -170,
      positionY: 90,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716854368',
      name: '算法组件2',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716854368_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716854368_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716854368_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716854368_out_2',
        },
      ],
      positionX: 40,
      positionY: 90,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716858435',
      name: '算法组件3',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716858435_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716858435_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716858435_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716858435_out_2',
        },
      ],
      positionX: -310,
      positionY: 230,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
    {
      id: '1603716868041',
      name: '算法组件2',
      inPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输入1',
          id: '1603716868041_in_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输入2',
          id: '1603716868041_in_2',
        },
      ],
      outPorts: [
        {
          tableName: 'germany_credit_data',
          sequence: 1,
          description: '输出表1',
          id: '1603716868041_out_1',
        },
        {
          tableName: 'germany_credit_data',
          sequence: 2,
          description: '输出表2',
          id: '1603716868041_out_2',
        },
      ],
      positionX: -100,
      positionY: 230,
      codeName: 'source_11111',
      catId: 1,
      nodeDefId: 111111,
      category: 'source',
      status: 3,
      groupId: 0,
    },
  ],
  links: [
    {
      source: '1603716783816',
      target: '1603716786205',
      outputPortId: '1603716783816_out_1',
      inputPortId: '1603716786205_in_1',
    },
    {
      source: '1603716783816',
      target: '1603716788394',
      outputPortId: '1603716783816_out_2',
      inputPortId: '1603716788394_in_1',
    },
    {
      source: '1603716783816',
      target: '1603716795011',
      outputPortId: '1603716783816_out_2',
      inputPortId: '1603716795011_in_1',
    },
    {
      source: '1603716786205',
      target: '1603716792826',
      outputPortId: '1603716786205_out_1',
      inputPortId: '1603716792826_in_1',
    },
    {
      source: '1603716788394',
      target: '1603716814719',
      outputPortId: '1603716788394_out_1',
      inputPortId: '1603716814719_in_1',
    },
    {
      source: '1603716795011',
      target: '1603716822805',
      outputPortId: '1603716795011_out_1',
      inputPortId: '1603716822805_in_1',
    },
    {
      source: '1603716795011',
      target: '1603716828657',
      outputPortId: '1603716795011_out_2',
      inputPortId: '1603716828657_in_2',
    },
    {
      source: '1603716792826',
      target: '1603716834901',
      outputPortId: '1603716792826_out_1',
      inputPortId: '1603716834901_in_1',
    },
    {
      source: '1603716814719',
      target: '1603716844054',
      outputPortId: '1603716814719_out_1',
      inputPortId: '1603716844054_in_1',
    },
    {
      source: '1603716822805',
      target: '1603716854368',
      outputPortId: '1603716822805_out_1',
      inputPortId: '1603716854368_in_1',
    },
    {
      source: '1603716834901',
      target: '1603716858435',
      outputPortId: '1603716834901_out_1',
      inputPortId: '1603716858435_in_1',
    },
    {
      source: '1603716844054',
      target: '1603716858435',
      outputPortId: '1603716844054_out_1',
      inputPortId: '1603716858435_in_2',
    },
    {
      source: '1603716854368',
      target: '1603716868041',
      outputPortId: '1603716854368_out_1',
      inputPortId: '1603716868041_in_1',
    },
  ],
}
