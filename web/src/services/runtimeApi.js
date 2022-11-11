import { noteApiPath2 } from './httpClient';
import { projectId } from '../store/cookie';
import request from "./request"


function resourceInfo() {
  const url = `${noteApiPath2}/resource/info?projectId=${projectId}`;
    return request.get(url);
}

const runtimeApi = {
    resourceInfo
};

export default runtimeApi;
