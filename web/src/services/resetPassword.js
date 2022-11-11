import { manageApiPath } from "./httpClient.js"
import request from "./request"

const getVerificationCode = (data) => {
  return request.post(`${manageApiPath}/user/mail/send-active-code`, data)
}

const resetPasswordPlase = (data) => {
  return request.post(`${manageApiPath}/user/rollback/reset-secret`, data)
}

const resetPassword = {
  getVerificationCode,
  resetPasswordPlase
}

export default resetPassword;
