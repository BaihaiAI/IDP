import { manageApiPath, noteApiPath2 } from './httpClient';
import { projectId, teamId, userId } from '@/store/cookie';
import request from "./request"


// 模型列表 搜索 分页
function listOrSearchOrPagination({modelName, size, current}){
  let url = `${manageApiPath}/admin-rs/model-manage/list?modelName=${modelName}&size=${size}&current=${current}`
  return request.get(url)
}

// 右键压缩ZIP
function compression({path}){
  const url = `${noteApiPath2}/workspace/dir/zip`
  const data ={
    path,
    teamId,
    projectId: Number(projectId)
  }
  return request.post(url, data)
}

// 输入名后后查看该名称下版本
function checkVersion({modelName}){
  const url = `${manageApiPath}/admin-rs/model-manage/version-list?modelName=${modelName}&projectId=${projectId}`
  return request.get(url)
}

// 模型上传客户端
function uploadClient({path, modelName, version, intro}){
  const url = `${noteApiPath2}/workspace/model/upload`
  const data = {
    path,
    teamId,
    userId,
    projectId: Number(projectId),
    modelName,
    version,
    intro,
  }
  return request.post(url, data)
}

// copy modal to project
function copyModalToProject({id}){
  const url = `${manageApiPath}/admin-rs/model-manage/copy-to-project`
  const data = {
    id,
    teamId,
    userId,
    projectId: Number(projectId)
  }
  return request.post(url, data)
}


// upload file
function uploadFile(formData){
  const url =`${manageApiPath}/admin-rs/model-manage/upload`
  formData.append("teamId", teamId)
  formData.append("userId", userId)
  formData.append("projectId", projectId)
  // formData.append("modelName", modelName)
  // formData.append("version", version)
  // formData.append("intro", intro)
  // formData.append("datafile", datafile)
  return request.post(url, formData)
}

// cancel upload
function cancelUpload({id}){
  const url = `${manageApiPath}/admin-rs/model-manage/cancel`
  return request.post(url, {id})
}


const warenhouseApi = {
  listOrSearchOrPagination,
  compression,
  checkVersion,
  uploadClient,
  copyModalToProject,
  uploadFile,
  cancelUpload
}
export default warenhouseApi;
