import { clusterPath } from "./httpClient";
import { projectId, teamId } from '../store/cookie';
import request from "./request"

const suggest = ({ path }) => {
  if (!Boolean(process.env.NODE_OPEN)) {
    const url = `${clusterPath}/suggest?teamId=${teamId}&projectId=${projectId}&path=${path}`
    return request.get(url)
  } else {
    return Promise.resolve()
  }
}

const clusterApi = {
  suggest,
}

export default clusterApi
