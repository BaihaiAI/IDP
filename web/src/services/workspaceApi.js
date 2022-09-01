import { noteApiPath2 } from "./httpClient"
import { projectId, teamId } from "../store/cookie"
import request from "./request"

function lazyLoadDirBrowse({ path = [], onlyPipelineSupport = false }) {
  const data = {
    path,
    onlyPipelineSupport,
    projectId: Number(projectId),
    teamId,
  }
  const url = `${noteApiPath2}/workspace/dir/browse`
  return request.post(url, data)
}

function dirBrowseForPipeline() {
  const url = `${noteApiPath2}/workspace/dir/recursive_browse`
  const data = {
    path: "/",
    onlyPipelineSupport: true,
    projectId: Number(projectId),
    teamId,
  }
  return request.post(url, data)
}

function dirNew(options) {
  const url = `${noteApiPath2}/workspace/dir/new`
  const data = {
    projectId: Number(projectId),
    path: options.path,
  }
  return request.post(url, data)
}

function wdelete({ isFile, path,autoClose=false }) {
  let url = isFile
    ? `${noteApiPath2}/workspace/file`
    : `${noteApiPath2}/workspace/dir`
  url = `${url}?path=${encodeURIComponent(path)}&projectId=${projectId}`
  if(autoClose){
    url = `${url}&autoClose=true`
  }
  return request.delete(url)
}

function fileNew(options) {
  const url = `${noteApiPath2}/workspace/file`
  const data = {
    projectId: Number(projectId),
    path: options.path,
  }
  return request.post(url, data)
}

function fileRename({ source, path, dest, autoClose = false }) {
  const url = `${noteApiPath2}/workspace/file/rename`
  const data = {
    projectId: Number(projectId),
    source,
    path,
    dest,
  }
  if (autoClose) {
    data.autoClose = autoClose
  }
  return request.post(url, data)
}

function uploadFile(options) {
  const url = `${noteApiPath2}/note/uploadbigfile`
  options.append("projectId", projectId)
  options.append("teamId", teamId)
  return request.post(url, options)
}

function moveFileOrDir({ originPath, targetPath, autoClose = false }) {
  const data = {
    originPath,
    targetPath,
    projectId: projectId * 1,
  }
  if (autoClose) {
    data.autoClose = autoClose
  }

  return request.post(`${noteApiPath2}/workspace/move`, data)
}

function copyFileOrDir({ originPath, targetPath }) {
  const data = {
    originPath,
    targetPath,
    projectId: projectId * 1,
  }
  return request.post(`${noteApiPath2}/workspace/copy`, data)
}

function globalKeywordSearch(keyword) {
  const data = {
    keyword,
    projectId: projectId * 1,
  }
  return request.post(
    `${noteApiPath2}/workspace/dir/global_keyword_search`,
    data
  )
}

const workspaceApi = {
  lazyLoadDirBrowse,
  dirBrowseForPipeline,
  dirNew,
  wdelete,
  fileNew,
  fileRename,
  uploadFile,
  moveFileOrDir,
  copyFileOrDir,
  globalKeywordSearch,
}

export default workspaceApi
