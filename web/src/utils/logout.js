import cookie from "react-cookies"

const redirectUrl = "/login"

export function logout() {
  const domain = window.location.host === "localhost:3000" ? "test.baihai.co" : window.location.host
  const primayDomain = window.location.host === "localhost:3000" ? "baihai.co" : window.location.host.substring(window.location.host.indexOf(".") + 1)
  cookie.remove("userId", { path: "/", domain: primayDomain })
  cookie.remove("userId", { path: "/", domain: domain })
  cookie.remove("userId", { path: "/", domain: "localhost" })

  cookie.remove("teamId", { path: "/", domain: primayDomain })
  cookie.remove("teamId", { path: "/", domain: domain })
  cookie.remove("teamId", { path: "/", domain: "localhost" })

  cookie.remove("token", { path: "/", domain: primayDomain })
  cookie.remove("token", { path: "/", domain: domain })
  cookie.remove("token", { path: "/", domain: "localhost" })

  cookie.remove("region", { path: "/", domain: primayDomain })
  cookie.remove("region", { path: "/", domain: domain })
  cookie.remove("region", { path: "/", domain: "localhost" })

  cookie.remove("majorVersionUpdate", { path: "/", domain: primayDomain })
  cookie.remove("majorVersionUpdate", { path: "/", domain: domain })
  cookie.remove("majorVersionUpdate", { path: "/", domain: "localhost" })

  window.localStorage.removeItem("historyOpenFile")
  window.localStorage.removeItem("historyOpenProject")
  window.localStorage.removeItem("avatar")
  window.localStorage.removeItem("permission_list")
  
  window.location.href = redirectUrl
}
