import { noteApiPath2 } from './httpClient';
import { projectId } from '../store/cookie';
import request from "./request"

const start = (options) => {
  const url = `${noteApiPath2}/tensorboard/start`;
  return request.post(url, {
    projectId: projectId,
    logDir: options.logDir,
  })
}

const stop = (options) => {
  const url = `${noteApiPath2}/tensorboard/stop`;
  return request.post(url, {
    projectId: projectId,
  });
}

const info = (options) => {
  const url = `${noteApiPath2}/tensorboard/info`;
  return request.post(url, {
    projectId: projectId,
  });
}

const tensorboardApi = {
  start,
  stop,
  info,
}

export default tensorboardApi;
