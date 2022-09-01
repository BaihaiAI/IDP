import { projectId, region, teamId } from '../store/cookie';
import request from "./request";

/**
 * workspace/pipeline
 */
function getTerminal(options) {
    let url = `${window.__POWERED_BY_QIANKUN__ ? window.location.origin : ''}/${region}/api/v1/terminal/pid?rows=${options.rows}&cols=${options.cols}&teamId=${teamId}&projectId=${projectId}`;
    return request.get(url);
}

const terminalApi = {
    getTerminal,
};

export default terminalApi;
