import { manageApiPath, modelApiPath, noteApiPath2 } from './httpClient';
import { projectId, teamId, userId } from '@/store/cookie';
import request from "./request"
import axios from 'axios';


// 老 模型列表 搜索 分页
function listOrSearchOrPagination({modelName="", size, current}){
  let url = `${manageApiPath}/admin-rs/model-manage/list?modelName=${modelName}&size=${size}&current=${current}`
  return request.get(url,)
}

// 新模型列表 普通筛选 common
function commonFilterList({
                            size,
                            current,
                            searchInfo,
                            sortField,
                            sort="desc"
                          }){
  const url = `${modelApiPath}/package/package-list`
  return request.get(url, {params: {
      size,
      current,
      searchInfo,
      sortField,
      sort
    }})
}

// 获取高级筛选 类别
function getCategory({parentId = 1}){
  const url = `${modelApiPath}/package/get-category-list?parentId=${parentId}`
  return request.get(url)
}


// 新模型列表 高级筛选
function advancedFilterList({
                              size,
                              current,
                              createTimeStart,
                              createTimeEnd,
                              updateTimeStart,
                              updateTimeEnd,
                              cateName,
                              visible,
                              searchInfo,
                              fileFrom,
                              sortField,
                              sort="desc"
                            }){
  const url = `${modelApiPath}/package/advanced-search`;
  return request.get(url,{params: {
      size,
      current,
      createTimeStart,
      createTimeEnd,
      updateTimeStart,
      updateTimeEnd,
      cateName,
      visible,
      searchInfo,
      fileFrom,
      sortField,
      sort
    }})
}


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

// 删除模型
function deteleModel({packageId}){
  const url = `${modelApiPath}/package/delete`;
  return request.post(url, {
    userId,
    teamId,
    packageId
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

// 获取详情
function getDetail({packageId}){
  const url = `${modelApiPath}/package/get-package-info`
  return request.get(url, {params: {
      packageId
    }})
}

// 模型更新字段
function upDataField({id, key, inputVal}){
  const url = `${modelApiPath}/package/update`;
  return request.post(url, {
    id,
    updaterId: userId,
    [key]: inputVal,
  })
}

// 获取模型版本列表
function getModelVersionList({packageId, size, current,sharingFlag=false}){
  const url = `${modelApiPath}/package/related-edition-list`;
  return request.get(url, {params: {
      packageId,
      userId,
      teamId,
      size,
      current,
      sharingFlag
    }})
}

// 修改版本说明
function RevisedReleaseNotes({intro, editionId}){
  const url = `${modelApiPath}/edition/update`;
  return request.post(url, {
    projectId: Number(projectId),
    userId,
    teamId,
    intro,
    editionId
  })
}

// 删除版本
function deteleVersion({editionId, packageId}){
  const url = `${modelApiPath}/edition/delete`;
  return request.post(url, {
    editionId,
    packageId,
    projectId: Number(projectId),
    teamId,
    userId
  })
}

// 取消共享
function cancelShared({editionId}){
  const url = `${modelApiPath}/edition/sharing`;
  return request.post(url, {
    editionId,
    projectId: Number(projectId),
    userId,
    teamId,
    sharingFlag: false
  })
}

// 克隆 到项目
function cloneToProject({id}){
  const url = `${modelApiPath}/edition/copy-to-project`;
  return request.post(url, {
    id,
    toTeamId: teamId,
    toProjectId: Number(projectId)
  })
}

// 模型文件包
function getModelFileList({mid}){
  const url = `${modelApiPath}/edition/file-list`;
  return request.get(url, {params: {
      id: mid,
      path: '/'
    }})
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

// 右键压缩ZIP
function compression({path}){
  const url = `${noteApiPath2}/workspace/dir/zip`
  const data ={
    path,
    teamId,
    projectId: Number(projectId)
  }
  return axios.post(url, data)
}

function decompressFile(path, extractTo) {
  const data ={ path, teamId, projectId, extractTo };
  return request.post(`${noteApiPath2}/workspace/file/decompress`, data);
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




const warenhouseApi = {
  listOrSearchOrPagination,
  commonFilterList,
  advancedFilterList,
  getCategory,
  upLoda,
  createModel,
  getDetail,
  upDataField,
  getModelVersionList,
  RevisedReleaseNotes,
  deteleVersion,
  cancelShared,
  cloneToProject,
  addVersion,
  getModelFileList,
  decideToShare,
  deteleModel,

  compression,
  copyModalToProject,
  uploadFile,
  decompressFile
}


export default warenhouseApi;
