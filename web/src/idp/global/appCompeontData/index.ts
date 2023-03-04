import { action, observable } from "mobx"
import React, { useId } from "react"
import projectApi from "../../../services/projectApi"
import { locationToProjectListPage } from "../../../utils"
import cookie from 'react-cookies';
import userInfoGlobal from '../userinfo';
import { saveHistoryOpenProject, historyOpenProject } from '../../../store/cookie';

export type IdpProject = {
  id?: string,
  name?: string,
  [x: string]: any
}

class AppComponentData {
  @observable projectInfo: IdpProject
  @observable workspaceRef: any
  @observable notebookTabRef: React.RefObject<any>
  @observable socketAlive: boolean

  constructor() {
    this.projectInfo = {}
    this.notebookTabRef = React.createRef()
    this.socketAlive = true
  }

  @action updateWorkspaceRef(workspaceRef: any) {
    this.workspaceRef = workspaceRef;
  }

  @action setSocketAlive(socketAlive: boolean) {
    this.socketAlive = socketAlive
  }

  @action async getProjectInfo(projectInfo: IdpProject) {
    const userInfo = await userInfoGlobal.getUserInfo();
    let projectId = new URLSearchParams(window.location.search).get("projectId");
    if (projectId) {
      // 等待接口
      if (Boolean(process.env.NODE_OPEN)) {
        this.projectInfo = { id: cookie.load('projectId'), name: cookie.load('projectId') };
      } else {
        projectApi
          .getProjectInfo(projectId)
          .then((res) => {
            const projectInfo = res.data
            this.projectInfo = projectInfo;
            saveHistoryOpenProject(projectId);
          })
          .catch((res) => {
            locationToProjectListPage()
          })
      }
    } else {
      let search = window.location.search
      projectId = historyOpenProject;
      let pathname = `./workspace`;
      if (userInfo.navType === 'AIGC') {
        pathname = './modelwarehouse/model_AIGC_Detail'
      }
      if (process.env.REACT_APP_VERSION === 'MODEL') {
        pathname = `./modelwarehouse/myModel`;
      }
      const url = (window.__POWERED_BY_QIANKUN__ ? window.location.pathname : pathname);
      if (projectId) {
        if (search) {
          search += `&projectId=${projectId}`
        } else {
          search = `?projectId=${projectId}`
        }
        window.location.href = `${url}${search}`;
      } else {
        const qs = new URLSearchParams(search)
        const shareId = qs.get("shareId")
        if (shareId) {
          // 打开分享链接中的文件
          projectApi.getProjectPage({ current: 1, size: 5, name: '' }).then((result) => {
            const { records: projectList } = result.data
            projectId = projectList[0].id
            if (search) {
              search += `&projectId=${projectId}`
            } else {
              search = `?projectId=${projectId}`
            }
            window.location.href = `${url}${search}`
          })
        } else {
          if (process.env.REACT_APP_VERSION === 'MODEL' || userInfo?.navType === 'AIGC') {
            if (cookie.load('userId') || userInfo?.navType === 'AIGC') {
              projectApi.getProjectPage({ current: 1, size: 1 }).then(res => {
                if (res.code == 200 && res.data.records.length > 0) {
                  if (userInfo?.navType === 'AIGC') {
                    window.location.href = `/studio/modelwarehouse/model_AIGC_Detail?projectId=${res.data.records[0].id}`;
                  } else {
                    window.location.href = `/studio/modelwarehouse/myModel?projectId=${res.data.records[0].id}`;
                  }
                } else {
                  locationToProjectListPage()
                }
              });
            } else {
              if (window.location.pathname !== '/studio/modelwarehouse/myModel') {
                locationToProjectListPage()
              }
            }
          } else {
            locationToProjectListPage()
          }
        }
      }
    }
  }

}


export default AppComponentData

