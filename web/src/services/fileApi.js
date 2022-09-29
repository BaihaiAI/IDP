import request from "@/services/request"


function uploadFiles({
file,
path
}) {
  const formData = new FormData()
  formData.append('path',path)
  formData.append('file',file)
  return request.post('/0/api/v1/files',formData)
}




const fileApi = {
  uploadFiles
}

export default fileApi
