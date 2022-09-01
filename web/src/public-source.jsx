import '../config/open-config';
import './index.less';

import { store } from '@/store';

import intl from "react-intl-universal";
import * as stores from './store/features';
import * as storageUtil from './utils/storage';
import * as layoutUtil from './utils/logout';
import * as services from './services';

import RightIcons from "idpStudio/components/Icons/RigftIcons";
import Icons from "@/components/Icons/TeamIcons";
import CommonIcons from "@/components/Icons/Icons"

import { useAvatarUrl } from "../src/utils/hook/useAvatarUrl";
import { loadModule } from './public-modules';
import Feedback from "idpStudio/components/Feedback/Feedback";

import { RegisterApi } from "idpStudio/idp/register";

import { OperatorFocus, OperatorBlur } from 'idpStudio/components/workspace/keymap';
import { LspWebsocket } from "idpStudio/components/notebook/lib/LspWebsocket";
import DataSetIcon from "./components/dataSet/DataSetIcon";

import { toFixedFileName, filterType } from 'idpUtils/index';
import { unNeedRequestErrMsg } from "idpServices/extraRequestConfig";

import appContext from 'idpStudio/context';
import DAG from "idpStudio/components/DAG/DAG";
import { HtmlView } from "idpStudio/components/DAG/pages/component-config-panel/form/html-view";
import { UnControlled as CodeMirror } from 'react-codemirror2';
import PipeLineHome from '@/components/pipeLine/PipeLineHome';
import PublishModel from '@/components/publishmodel/PublishModel';
import UsageFooterBar from '@/idp/component/usageFooterBar'

import { teamId, userId, projectId } from '@/store/cookie';
import globalData from "idp/global";
import { setCurrentEnv } from '@/store/config';
import { locationToProjectListPage, refreshPage } from "@/utils";

// 强制使用中文

export default {
    StudioStore: store,
    StudioIntl: intl,
    DAG,
    PipeLineHome,
    HtmlView,
    getTeamById: () => teamId,
    getUserById: () => userId,
    getProjectId: () => projectId,
    getGlobalData: () => globalData,
    CodeMirror,
    PublishModel,
    LspWebsocket,
    AppContext: appContext,
    IdpComponents: (type) => loadModule(type),
    StudioDispatch: store.dispatch,
    FeaturesStores: stores,
    StudioUtils: {
        storage: storageUtil,
        layoutUtil,
        useAvatarUrl,
        toFixedFileName,
        filterType,
        unNeedRequestErrMsg,
        setCurrentEnv,
        locationToProjectListPage,
        refreshPage
    },
    StudioServices: services,
    StudioIcons: Icons,
    CommonIcons,
    Feedback,
    StudioRightIcons: RightIcons,
    RegisterApi,
    OperatorFocus,
    OperatorBlur,
    DataSetIcon,
    UsageFooterBar
}