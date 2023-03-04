import React, { Component, useMemo, useState } from 'react';
import { useHistory, useLocation } from "react-router";

function userHeaderIdp() {

    const location = useLocation();

    const pathName = location.pathname.split('/').filter(it => it != '');

    const useLoadIdp = useMemo(() => {
        if (pathName.indexOf('workspace') == -1) {
            return false
        } else {
            return true;
        }
    }, [pathName]);

    return useLoadIdp
}

export default userHeaderIdp;