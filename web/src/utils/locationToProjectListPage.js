import qs from "querystring"

const locationUrl = "/team"
export function locationToProjectListPage({ queryStringObj, path = "",open } = {}) {
  const queryStringParse = qs.encode(queryStringObj)
  const finalUrl =
    locationUrl + (path ? `/${path}` : "")
    + (queryStringParse ? `?${queryStringParse}` : "")

  if(open){
    window.open(finalUrl)
  }else{
    window.location.href = finalUrl
  }

}
