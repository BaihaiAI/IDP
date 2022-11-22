import cookie from "react-cookies";
import { analysisUrl } from '../../../config/auth';

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

  cookie.remove("majorVersionUpdate", { path: "/", domain: primayDomain })
  cookie.remove("majorVersionUpdate", { path: "/", domain: domain })
  cookie.remove("majorVersionUpdate", { path: "/", domain: "localhost" })

  cookie.remove("region", { path: "/", domain: primayDomain })
  cookie.remove("region", { path: "/", domain: domain })
  cookie.remove("region", { path: "/", domain: "localhost" })
  window.localStorage.removeItem("historyOpenProject")

  window.localStorage.removeItem("historyOpenFile")
  window.localStorage.removeItem("avatar")
  window.localStorage.removeItem("permission_list")

  cookie.remove("code", { path: "/", domain: primayDomain })
  cookie.remove("code", { path: "/", domain: domain })
  cookie.remove("code", { path: "/", domain: "localhost" })

  cookie.remove("scope", { path: "/", domain: primayDomain })
  cookie.remove("scope", { path: "/", domain: domain })
  cookie.remove("scope", { path: "/", domain: "localhost" })

  cookie.remove("state", { path: "/", domain: primayDomain })
  cookie.remove("state", { path: "/", domain: domain })
  cookie.remove("state", { path: "/", domain: "localhost" })

  // 退出登录
  analysisUrl()
}
