import request from "./request"
import { projectId } from "../store/cookie"
import { kernelApiPath,noteApiPath2 } from "./httpClient"

const getResourceVars = (filePath, inode) => {
  return request.get(
    `${kernelApiPath}/notebook/vars?filePath=${encodeURIComponent(filePath)}&projectId=${projectId}&inode=${inode}`
  )
}

const share = ({ path, cellId }) => {
  return request.get(`${noteApiPath2}/content/share`, {
    params: {
      path,
      cellId,
      projectId,
    },
  })
}

const variableManagerApi = {
  getResourceVars,
  share,
}

export default variableManagerApi
