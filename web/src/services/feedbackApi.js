import {  manageApiPath, adminRsPath } from './httpClient'
import { userId } from '../store/cookie'
import request from "./request"

function save(options) {
  const url = `${manageApiPath}/feedback/save`;
  if (Boolean(process.env.NODE_OPEN)) {
    return Promise.resolve(false);
  } else {
    return request.post(url, { ...options, userId })
  }
}

function save_new(options){
  const { feedback, contact, userName, fileIdList } = options;
  const url = `${adminRsPath}/feedback/new`
  return request.post(url, {
    feedback,
    contact,
    userName,
    fileList:fileIdList,
    userId 
  })
}

const feedbackApi = {
  save,
  save_new
}

export default feedbackApi;
