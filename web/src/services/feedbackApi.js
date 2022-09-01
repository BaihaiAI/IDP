import {  manageApiPath } from './httpClient'
import { userId } from '../store/cookie'
import request from "./request"

function save(options) {
  const url = `${manageApiPath}/feedback/save`
  return request.post(url, { ...options, userId })
}

const feedbackApi = {
  save,
}

export default feedbackApi;
