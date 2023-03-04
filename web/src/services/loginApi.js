import { manageApiPath } from './httpClient';
import request from "./request";
import projectApi from './projectApi';
import md5 from 'md5';

async function login(username, pwd) {
    const url = `${manageApiPath}/user/account/login`;
    const result = await request.post(url, JSON.stringify(GetJsonData(username, pwd)), {
        headers: {
            'Content-Type': 'application/json; charset=utf-8'
        }
    });
    if (result.code == 200) {
        const teamResult = await projectApi.getProjectPage({ current: 1, size: 5, teamId: result.data.teamId, userId: result.data.userId });
        if (teamResult.code == 200) {
            const teamArray = teamResult.data.records.filter(it => it.name == '示例项目');
            if (teamArray.length > 0) {
                return Promise.resolve({
                    teamId: result.data.teamId,
                    userId: result.data.userId,
                    region: teamArray[0]['region'],
                    projectId: teamArray[0]['id'],
                });
            }
        }
    }
    return Promise.reject(false);
}

export function GetJsonData(username, pwd) {
    var md5password = md5(pwd);
    var json = {
        "account": username,
        "password": md5password,
    };
    return json;
}

const loginApi = {
    login
};

export default loginApi;
