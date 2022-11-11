export function refreshPage(queryString) {
  const baseUrl = window.location.href.split("?")[0]
  window.location.href = baseUrl + queryString
}

