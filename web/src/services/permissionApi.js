import request from "@/services/request"


function permissionList() {
  return request.get('/0/api/v1/model-api/permission_list')
}


const permissionApi = {
  permissionList
}


export default permissionApi
