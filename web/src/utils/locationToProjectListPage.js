import qs from "querystring";
import { hasModulePermission } from "./storage";
import cookie from 'react-cookies';

const locationUrl = "/team"
export function locationToProjectListPage({ queryStringObj, path = "", open, userInfo } = {}) {
    if (!hasModulePermission("left_sidebar")) { path = "" };
    const queryStringParse = qs.encode(queryStringObj);
    const finalUrl = locationUrl + (path ? `/${path}` : "") + (queryStringParse ? `?${queryStringParse}` : "")
    if (open) {
        window.open(finalUrl)
    } else {
        window.location.href = finalUrl
    }
}
