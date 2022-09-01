import cookie from 'react-cookies';
import '../../config/open-config';

export const token = cookie.load("token");


export const teamId = cookie.load('teamId');
export const userId = cookie.load('userId');
export const getTeamId = () => cookie.load('teamId')

export const region = cookie.load('region')
export const projectId = new URLSearchParams(window.location.search).get('projectId')
export const getProjectId = () => {
  return new URLSearchParams(window.location.search).get('projectId')
}
