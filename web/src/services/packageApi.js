import {manageApiPath,kernelApiPath,noteApiPath2} from './httpClient'
import {projectId, teamId} from '../store/cookie'
import request from "./request"

function list() {
  const url = `${noteApiPath2}/package/list?projectId=${projectId}&teamId=${teamId}`;
  return request.get(url);
}


function searchV2(options) {
  const url = `${manageApiPath}/package/search?packageName=${options.packageName}&size=50`;
  return request.get(url);
}

function searchV3({ packageName, current = 1, size = 50 }) {
  const path = `${noteApiPath2}/package/search`
  return request.get(path, {
    params: {
      packageName,
      current,
      size,
      projectId,
    },
  })
}

function install(options) {
  const url = `${kernelApiPath}/package/install`;
  const data = {
      projectId: projectId,
      packageName: options.packageName,
      version: options.version
  };
  return request.post(url, data);
}

function uninstall(options) {
    const url = `${kernelApiPath}/package/uninstall`;
    const data = {
      projectId: projectId,
        packageName: options.packageName,
        version: options.version
    };
    return request.post(url, data);
}

const packageApi = {
    list,
    searchV2,
    install,
    uninstall,
    searchV3
};

export default packageApi;
