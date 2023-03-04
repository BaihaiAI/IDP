import request from "./request"
import { manageApiPath } from "./httpClient"
import {getTeamId, teamId, userId} from "../store/cookie"

const getProjectInfo = (id) => {
  return request.get(`${manageApiPath}/project/get`, {
    params: { id, teamId: getTeamId() },
  })
}

const getProjectPage = ({ current, size, name }) => {
  return request.get(`${manageApiPath}/project/getPage`, {
    params: { teamId, current, size, name },
  })
}

const deleteProject = (id) => {
  return request.post(`${manageApiPath}/project/delete`, { id })
}

const addOrUpdateProject = ({ id, name }) => {
  return request.post(`${manageApiPath}/project/update`, {
    id,
    name,
    owner: userId,
  })
}

const transProjectOwner = ({ id, owner }) => {
  return request.post(`${manageApiPath}/project/transOwner`, {
    id,
    owner,
    teamId,
  })
}

const getFirstProject =  ()=>{
  return request.get("/0/api/v1/project/getFirstProject",{
    params:{
      teamId
    }
  })
}

const projectApi = {
  getProjectPage,
  getProjectInfo,
  deleteProject,
  addOrUpdateProject,
  transProjectOwner,
  getFirstProject
}
export default projectApi
