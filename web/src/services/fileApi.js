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

function uploadBigFiles({
datafile,
path,
index,
total,
generateName,
}) {
  const formData = new FormData()
  formData.append("datafile",datafile)
  formData.append("path",path)
  formData.append("index",index)
  formData.append("total",total)
  formData.append("generateName",generateName)
  return request.post('/0/api/v1/files/upload',formData)
}



const fileApi = {
  uploadFiles,
  uploadBigFiles
}

export default fileApi
