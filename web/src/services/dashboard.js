import request from "./request"
import {noteApiPath2,manageApiPath} from "./httpClient"
import {getProjectId, getTeamId, region, userId } from "../store/cookie"


function taskMonitorTotal() {
  return request.get(`${noteApiPath2}/runtime/resource_usage?`,{
    params:{
      teamId:getTeamId(),
      projectId:getProjectId(),
      userId
    }
  })
}




const dashboardApi = {
  taskMonitorTotal
}

export default dashboardApi
