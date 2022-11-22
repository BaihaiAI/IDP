import cookie from 'react-cookies';
import '../../config/open-config';

export const token = cookie.load("token");


export const teamId = cookie.load('teamId');
export const userId = cookie.load('userId');
export const getTeamId = () => cookie.load('teamId')

export const region = cookie.load('region')
export const projectId = new URLSearchParams(window.location.search).get('projectId')

const setProjectId = () => {
  const host = window.location.host
  const domain = host.startsWith('localhost') ? host : host.substring(host.indexOf('.'))
  cookie.save('projectId', projectId, { domain: domain, path: '/'})
}
setProjectId()

export const getProjectId = () => {
  return new URLSearchParams(window.location.search).get('projectId')
}


// 游客
export const isTraveler = () => {
  return !cookie.load("teamId") && !cookie.load('userId')
}

export const userDir = `/store/${teamId}/projects/${projectId}/notebooks`
