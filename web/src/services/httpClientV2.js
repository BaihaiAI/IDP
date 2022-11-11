import axios from 'axios';
import { message as messageShow } from "antd"
import { userId } from '../store/cookie';
import { logout } from "../utils"

export const manageApiPath = '/0/api/v1';

const expiredCode = 41110401
const httpExpiredCode = 401

// 自动提交异常信息
const submitErrInfo = (source, error) => {
  const isJson = typeof (error) === 'object'
    && Object.prototype.toString.call(error).toLowerCase() === '[object object]'
    && !error.length;
  const message = isJson ? JSON.stringify(error) : error.toString()
  const feedback = {
    source,
    message
  }
  const params = {
    category: 1,
    userId: userId,
    feedback: JSON.stringify(feedback),
    fileIdList: []
  }
  axios.post(`${manageApiPath}/feedback/save`, params)
    .catch((error) => {
      console.log(error);
    })
}

const handleError = (url, data, reject, config) => {
  const { code, message } = data
  // 处理错误信息
  if (config && config.error) {
    reject({ message })
  } else {
    const messageType = (code > 30000000 && code < 50000000) ? 'warning' : 'error'
    const messageText = `${process.env.NODE_ENV === 'development' ? '后端接口' + code + '异常' : '异常'} ${message ? 'message: ' + message : ''}`
    if (messageType === 'error') {
      messageShow.error(messageText, 3)
    } else {
      messageShow.warning(messageText, 3)
    }
  }

  // 自动提交异常信息
  submitErrInfo(url, message)
}

const handleCode = (url, data, resolve, reject, config) => {
  const { code } = data
  if (code === 200 || (code >= 20000000 && code <= 29999999)) {
    resolve(data);
  } else {
    // 处理错误信息
    handleError(url, data, reject, config)
    // 验证失败，重定向回登录页
    if (code === expiredCode || code === httpExpiredCode) {
      setTimeout(() => {
        logout()
      }, 2000)
    }
  }
}

const getRequestConfig = (config) => {
  return config || {}
}

const handleCatch = (url, error, reject, config) => {
  const code = error.response ? 10000000 + error.response.status : 10000400
  handleError(url, { code, message: error.message }, reject, config)
}

const get = (url, config) => {
  const requestConfig = getRequestConfig(config)
  return new Promise((resolve, reject) => {
    axios.get(url, requestConfig)
      .then((response) => {
        handleCode(url, response.data, resolve, reject, config)
      }).catch((error) => {
        handleCatch(url, error, reject, config)
      })
  });
}

const post = async (url, data, config) => {
  const requestConfig = getRequestConfig(config)
  return new Promise((resolve, reject) => {
    axios.post(url, data, requestConfig)
      .then((response) => {
        handleCode(url, response.data, resolve, reject, config)
      }).catch((error) => {
        handleCatch(url, error, reject, config)
      })
  });
}

const put = (url, data, config) => {
  const requestConfig = getRequestConfig(config)
  return new Promise((resolve, reject) => {
    axios.put(url, data, requestConfig)
      .then((response) => {
        handleCode(url, response.data, resolve, reject, config)
      })
      .catch((error) => {
        handleCatch(url, error, reject, config)
      })
  });
}

const del = (url, config) => {
  const requestConfig = getRequestConfig(config)
  return new Promise((resolve, reject) => {
    axios.delete(url, requestConfig)
      .then((response) => {
        handleCode(url, response.data, resolve, reject, config)
      })
      .catch((error) => {
        handleCatch(url, error, reject, config)
      })
  });
}

const httpClientV2 = {
  get,
  post,
  put,
  del
}

export default httpClientV2