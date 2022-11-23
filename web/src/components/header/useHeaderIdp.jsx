import React, { Component, useMemo, useState } from 'react';
import { useHistory, useLocation } from "react-router";

function userHeaderIdp() {

    const location = useLocation();
    const loadIdpKeyWorkspace = ['idps_team', 'idps_files', 'idps_view', 'idps_run', 'idps_kernel', 'idps_tool', 'idps_help'];
    const loadIdpKeyOther = ['idps_team', 'idps_help'];

    const pathName = location.pathname.split('/').filter(it => it != '');

    const useLoadIdp = useMemo(() => {
        if (pathName.indexOf('workspace') == -1) {
            return loadIdpKeyOther
        } else {
            return loadIdpKeyWorkspace;
        }
    }, [pathName]);

    return useLoadIdp
}

export default userHeaderIdp;