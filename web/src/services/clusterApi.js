import { clusterPath } from "./httpClient";
import request from "./request"

const suggest = () => {
  if (!Boolean(process.env.NODE_OPEN)) {
    const url = `${clusterPath}/suggest`
    return request.get(url)
  } else {
    return Promise.resolve()
  }
}

const clusterApi = {
  suggest,
}

export default clusterApi
