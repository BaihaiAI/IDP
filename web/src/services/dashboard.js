import request from "./request"
import {noteApiPath2,manageApiPath} from "./httpClient"
import {getProjectId, getTeamId, region} from "@/store/cookie"


function taskMonitorTotal() {
  return request.get(`${manageApiPath}/admin-rs/dashboard/team/task-monitor-total?`,{
    params:{
      teamId:getTeamId(),
      projectId:getProjectId()
    }
  })
}




const dashboardApi = {
  taskMonitorTotal
}

export default dashboardApi
