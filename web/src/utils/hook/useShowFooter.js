import {useLocation} from "react-router"
import {useMemo} from "react"


const useShowFooter = ()=>{
  const location = useLocation()
  const isShowFooter = useMemo(() => {
    const pathname = location.pathname
    let isShow = false
    if (pathname.endsWith("/workspace") || pathname.endsWith("/terminal")) {
      isShow = true
    }
    return isShow
  }, [location.pathname])

  return isShowFooter
}

export default useShowFooter
