import { adminRsPath, modelServicePath} from "./httpClient"
import axiosRequest from "./request"
import {getProjectId, getTeamId, projectId, userId} from "../store/cookie"

function deployApiPod({
  serviceName,
  intro,
  modelId,
  modelFullName,
  runTime,
  request,
  response,
}) {
  const data = {
    serviceName,
    intro,
    modelId,
    modelFullName,
    runTime,
    request,
    response,
    projectId:Number(getProjectId()),
    teamId:getTeamId(),
    userId,
  }
  return axiosRequest.post(`${adminRsPath}/model-deploy/deploy-pod`,data)
}

function updateModelDeploy({
   id,
   serviceName,
   intro,
   modelId,
   runTime,
   request,
   response,
}) {
  const data = {
    id,
    serviceName,
    intro,
    modelId,
    runTime,
    request,
    response,
  }
  return axiosRequest.post(`${adminRsPath}/model-deploy/update`,data)
}

function destroyApiPod(id) {
  return axiosRequest.post(`${adminRsPath}/model-deploy/stop-pod`,{id})
}
function startApiPod(id) {
  return axiosRequest.post(`${adminRsPath}/model-deploy/start-pod`,{id})
}

function getModelInfo(id) {
  return axiosRequest.get(`${adminRsPath}/model-deploy/get-model-info`,{
    params:{
      id
    }
  })
}

function getModelDeployList({ current, size,serviceName="" }) {
  const params = {
    current,
    size,
    projectId: getProjectId(),
  }
  if(serviceName.trim()){
    params.serviceName = serviceName
  }

  return axiosRequest.get(`${adminRsPath}/model-deploy/list`, {
    params
  })
}


function getModelFileList({modelId, mid, path=""}){
  const url = `${modelServicePath}/model-deploy/file-list?mid=${mid}`
  return axiosRequest.post(url, {modelId, path, id: mid})
}

function downLoad({modelId, mid}){
  const url = `${modelServicePath}/model-deploy/model-download?teamId=${getTeamId()}&userId=${userId}&projectId=${projectId}&modelId=${modelId}&mid=${mid}`
  return axiosRequest.get(url)
}

function deletePod(id) {
  return axiosRequest.post(`${adminRsPath}/model-deploy/delete-pod`,{
    id
  })
}



const modelServiceApi = {
  getModelDeployList,
  deployApiPod,
  destroyApiPod,
  startApiPod,
  getModelInfo,
  getModelFileList,
  downLoad,
  deletePod,
  updateModelDeploy
}

export default modelServiceApi
