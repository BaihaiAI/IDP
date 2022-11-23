import { projectId, teamId, userId, region } from '@/store/cookie';
import request from '../../../services/request'
const manageApiPath = "/0/api/v1",
      modelApiPath = "/0/api/v1/model-api",
      noteApiPath2 = `/${region}/api/v2/idp-note-rs`;


// 上传
function upLoda({file}){
  const url = `${manageApiPath}/files`;
  const formData = new FormData()
  formData.append('file', file)
  formData.append('path', `model_files/${teamId}/projects/${projectId}/models/`)
  return request.post(url, formData)
}

// 创建模型
function createModel(data) {
  const url = `${modelApiPath}/package/new`;
  return request.post(url, {
    ...data,
    teamId,
    creatorId: userId,
    updaterId: userId,
    projectId: Number(projectId),
  })
}


// 给模型新增版本
function addVersion(data){
  const url = `${modelApiPath}/edition/new`;
  return request.post(url, {
    ...data,
    projectId: Number(projectId),
    userId,
    teamId,
  })
}


// 共享模型  sharingFlag true 开启 false 取消
function decideToShare({editionId, sharingFlag}){
  const url = `${modelApiPath}/edition/sharing`;
  return request.post(url, {
    editionId,
    projectId: Number(projectId),
    userId,
    teamId,
    sharingFlag
  })
}

// 上传到模型管理 notebook 右键发布模型 对于前端来说 是获取上传成功后的字符串
function getSuccessString({path}){
  const url = `${noteApiPath2}/workspace/model/export`;
  return request.post(url, {
    projectId: Number(projectId),
    userId,
    teamId,
    path
  })
}

function getFileSuccessString({path}){
  const url = `${noteApiPath2}/workspace/model/export_dir`;
  return request.post(url, {
    projectId: Number(projectId),
    userId,
    teamId,
    path
  })
}

// 获取高级筛选 类别
function getCategory({parentId = 1}){
  const url = `${modelApiPath}/package/get-category-list?parentId=${parentId}`
  return request.get(url)
}


const createOrAddAModelOrVersionApi = {
  upLoda,
  createModel,
  addVersion,
  decideToShare,
  getSuccessString,
  getFileSuccessString,
  getCategory
}


export default createOrAddAModelOrVersionApi;
