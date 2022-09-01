import request, {requestWithBigIntResponse} from "./request"
import { projectId,teamId } from "../store/cookie"
import { commandApiPath, commandApiPath2, commandManagerApiPath } from "./httpClient"

const project = projectId

const connectDataBase = ({
  db,
  host,
  port,
  username,
  password,
  dbName,
  aliasDB,
}) => {
  return request.post(`${commandApiPath}/database/conn`, {
    db,
    host,
    port,
    username,
    password,
    dbName,
    aliasDB,
    teamId,
    project,
  })
}

const addDataBase = ({
  db,
  host,
  port,
  username,
  password,
  dbName,
  aliasDB,
}) => {
  /*
   {
    "db": "ucloud s3",
    "host": "endpoint值",
    "port"：""
    "username": "accessKey值",
    "password": "secretKey值",
    "dbName": "bucket值",
    "aliasDB": "halo",
    "teamId": "2",
    "project": "11"
    }
  */
  return request.post(`${commandManagerApiPath}/command/datasource/add`, {
    db,
    host,
    port,
    username,
    password,
    dbName,
    aliasDB,
    teamId,
    project,
  })
}

const addNFS = ({
  cloudName,
  mountPath,
  endPoint,
  alias,
  fsType,
  db=' ',
  host,
  port=' ',
  username=' ',
  password=' ',
  dbName=' ',
  aliasDB,
}) => {
  return request.post(`${commandManagerApiPath}/command/datasource/add`, {
    cloudName,
    mountPath,
    endPoint,
    alias,
    fsType,
    db,
    host,
    port,
    username,
    password,
    dbName,
    aliasDB,
    teamId,
    project,
  })
}

const getDataBaseList = () => {
  if (!Boolean(process.env.NODE_OPEN)) {
    return request.post(`${commandManagerApiPath}/command/datasource/list`, { teamId, project })
  } else {
    return new Promise((resolve, reject)=>{
      resolve({
        code:200,
        data: {
          record:[],
          schema:{}
        }
      })
    })
  }
}

const deleteDataBase = (aliasDB) => {
  return request.post(`${commandManagerApiPath}/command/datasource/delete`, {
    aliasDB,
    teamId,
    project,
  })
}
const getActiveDataBaseList = () => {
  if (!Boolean(process.env.NODE_OPEN)) {
    return request.post(`${commandApiPath}/database/active`, { project })
  } else {
    return new Promise((resolve)=> {
      resolve({
        code: 200,
        data: []
      })
    })
  }
}
const getActiveDataBaseList_v2 = () => {
  return request.post(`${commandApiPath2}/database/active`, { project })
}

const reconnectDataBase = (data) => {
  const { aliasDB, username, password, db, dbUrl, dbName } = data
  return request.post(`${commandApiPath}/database/reconnect`, {
    aliasDB,
    username,
    password,
    db,
    dbUrl,
    dbName,
    project,
  })
}
const closeDataBase = (aliasDB) => {
  return request.post(`${commandApiPath}/database/close`, {
    aliasDB,
    project,
  })
}
const runSql = ({ sql, aliasDB, project }) => {
  return request.post(`${commandApiPath}/database/sql`, {
    sql: encodeURIComponent(sql),
    aliasDB,
    project,
  })
}

const getTableList = (aliasDB) => {
  return request.post(`${commandApiPath}/database/tables`, {
    aliasDB,
    project,
  })
}

const getSchemaList = ({ aliasDB, tableName }) => {
  return request.post(`${commandApiPath}/database/schema`, {
    aliasDB,
    tableName,
    project,
  })
}

const getSelectList = ({ aliasDB, tableName }) => {
  return requestWithBigIntResponse.post(`${commandApiPath}/database/select`, {
    aliasDB,
    tableName,
    limit: "10",
    project,
  })
}

const mountCloud = ({
  cloudName,
  mountPath,
  bucket,
  accessKey,
  secretKey,
  endPoint,
  alias,
  fsType,
  url,
}) => {
  const data = {
    cloudName,
    mountPath,
    bucket,
    accessKey,
    secretKey,
    endPoint,
    alias,
    fsType,
    project,
    url,
    teamId
  }

  return request.post(`${commandApiPath}/cloud/mount`, data)
}

const kernelMountCloud = ({
  cloudName,
  mountPath,
  bucket,
  accessKey,
  secretKey,
  endPoint,
  alias,
  fsType,
  url,
}) => {
  const data = {
    cloudName,
    mountPath,
    bucket,
    accessKey,
    secretKey,
    endPoint,
    alias,
    fsType,
    project,
    url,
    teamId
  }

  return request.post(`${commandApiPath2}/cloud/mount`, data)
}

const unMoundCloud = ({ cloudName, mountPath, alias }) => {
  const data = { cloudName, mountPath, alias, project, teamId }
  return request.post(`${commandApiPath}/cloud/umount`, data)
}
const unMoundCloud_v2 = ({ cloudName, mountPath, alias }) => {
  const data = { cloudName, mountPath, alias, project, teamId }
  return request.post(`${commandApiPath2}/cloud/umount`, data)
}

const nfsMount = ({cloudName, mountPath, endPoint, alias, db, fsType}) => {
  const data = {cloudName, mountPath, endPoint, alias, fsType, db, project, teamId}
  return request.post(`${commandApiPath}/cloud/nfsmount`, data)
}
const dataSetApi = {
  connectDataBase,
  getDataBaseList,
  getActiveDataBaseList,
  getActiveDataBaseList_v2,
  closeDataBase,
  reconnectDataBase,
  deleteDataBase,
  getTableList,
  getSchemaList,
  getSelectList,
  addDataBase,
  mountCloud,
  kernelMountCloud,
  unMoundCloud,
  unMoundCloud_v2,
  nfsMount,
  addNFS
}

export default dataSetApi
