import * as services from '../../src/services';
import { unNeedRequestErrMsg } from "../../src/services/extraRequestConfig";
import { teamId, userId, projectId, isTraveler, region } from '../../src/store/cookie';
import usage from '../../src/idp/lib/usage';
import workspace from '../../src/idp/global/workspace';
import globalData from "../../src/idp/global";
import { getCurrentEnv, setCurrentEnv } from '../../src/store/config';
import { lspWebsocketSafeSend } from '../../src/idp/global/lspWebsocket';
import { useAvatarUrl } from "../../src/utils/hook/useAvatarUrl";
import { logout, locationToProjectListPage, mergeArray, goAccountRouter } from '../../src/utils';
import { gerModulePermissionList } from "../../src/utils/storage"
import { showApproveConfirm } from "../../src/utils/modalConfirm"
import Feedback from "../../src/components/Feedback/Feedback"
import request from "../../src/services/request"
import { saveHistoryOpenProject, removeHistoryOpenProject } from '../../src/store/cookie';
// import NoteBookTabContainer from '../../src/components/notebook/NoteBookTabContainer';
// import filesTabSlice from '../../src/store/features/filesTabSlice';

export default {
    isTraveler,
    getTeamById: () => teamId,
    getUserById: () => userId,
    getProjectId: () => projectId,
    getGlobalData: () => globalData,
    gotoWorkSpace: (params1, params2) => workspace.exeWorkSpaceRef(params1, params2),
    getWorkSpace: (callback) => workspace.getWorkSpace(callback),
    getUsage: () => usage,
    saveHistoryOpenProject,
    removeHistoryOpenProject,
    getCurrentEnv,
    setCurrentEnv,
    mergeArray,
    goAccountRouter,
    unNeedRequestErrMsg,
    StudioServices: services,
    lspWebsocketSafeSend,
    useAvatarUrl,
    logout,
    locationToProjectListPage,
    gerModulePermissionList,
    showApproveConfirm,
    Feedback,
    region,
    projectId,
    teamId,
    userId,
    request,
  // NoteBookTabContainer,
  // filesTabSlice,
}
