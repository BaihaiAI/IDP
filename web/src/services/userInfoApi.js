import { manageApiPath } from "./httpClient"
import {userId} from "@/store/cookie"
import request from "./request.js"


const getUserInfo = () => {
  return request.get(`${manageApiPath}/user/getUserInfo`, { params: { userId } })
}

const updateUsername = ({ username }) => {
  return request.post(`${manageApiPath}/user/mail/update-username`, { username, userId })
}
const uploadAvatar = (file) => {
  const formData = new FormData()
  formData.append("userId", userId)
  formData.append("file", file)
  return request.post(`${manageApiPath}/user/upload-profile-photo`, formData)
}

const getInviteCode = (data) => {
  return request.post(`${manageApiPath}/admin/vefify/send-vefify-code`, data)
}

const verifyInviteCode = (data) => {
  return request.post(`${manageApiPath}/admin/vefify/check-vefify-code`, data)
}

const userInfoApi = {
  getUserInfo,
  updateUsername,
  uploadAvatar,
  getInviteCode,
  verifyInviteCode
}

export default userInfoApi
