import { projectId, teamId} from "@/store/cookie"
import {  noteApiPath2, manageApiPath } from './httpClient';
import request from "./request"

function getEnvironmentName(){
  const url = `${noteApiPath2}/environment/current?projectId=${projectId}`;
  return request.get(url);
}

function switchEnvironment({environmentName, computeType}){
  const url = `${noteApiPath2}/environment/switch?environmentName=${environmentName}&projectId=${projectId}&computeType=${computeType}`;
  return request.put(url)
}

function saveEnvironment({originName, targetName}){
  const url = `${noteApiPath2}/environment/clone`;
  const data = {
    originName,
    targetName
  };
  return request.post(url, data)
}

function getEnvironmentList(){
  const url = `${noteApiPath2}/environment/list`;
  return request.get(url);
}

function queryCloneState({cloneStateKey}){
  const url = `${noteApiPath2}/environment/clone/state?cloneStateKey=${cloneStateKey}`;
  return request.get(url)
}

// 获取机器列表
function getMachineList(){
  const url = `${manageApiPath}/project/getComputeTypeList?teamId=${teamId}`;
  return request.get(url)
}

const environmentAPI = {
  getEnvironmentName,
  switchEnvironment,
  saveEnvironment,
  getEnvironmentList,
  queryCloneState,
  getMachineList
}
export default environmentAPI;
