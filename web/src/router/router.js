import React from "react"
import { Route, Redirect } from "react-router-dom"
import { CacheRoute, CacheSwitch } from 'react-router-cache-route'
import { Layout } from "antd"
import globalData from "idp/global"
import navConfig from "@/pages/common/navConfig"
import { toJS } from "mobx"
import {isTraveler} from "@/store/cookie"
import {travelRoutes} from "@/pages/common/leftNav"


// isTravel函数是判断是否在游客模式下
function RouterConfig() {

    const toPath = () => {
        if(!isTraveler()){
          const search = window.location.search
          if (search) {
            return `./workspace${search}`
          }
          return './workspace'
        }
        return "/modelwarenhouse"
    }
    let finallyRoutes = navConfig.concat(toJS(globalData.routerMenuControl.currentRoutes))
    if(isTraveler()){
      finallyRoutes = finallyRoutes.filter(item=>travelRoutes.includes(item.key))
    }

    return (
        <Layout.Content
            className="site-layout-background"
            style={{
                // minHeight: clientHeight - 40,
                borderRight: "1px solid rgb(214, 222, 230)",
                background: "#fff",
                position: "relative",
                zIndex: "1",
            }}
        >
            <CacheSwitch>
                {
                  finallyRoutes.map(route => {
                        if (route.needCache) {
                            return <CacheRoute exact={true} path={'/' + route.key} key={route.key} component={route.component} />
                        }
                        return <Route
                            key={route.key}
                            exact={!route.notNeedExact}
                            path={'/' + route.key}
                            component={Object.prototype.toString.call(route.component) === '[object, Function]' ? route.component() : route.component}
                        />
                    })
                }
                <Redirect exact from="/" to={toPath()} />
            </CacheSwitch>
        </Layout.Content>
    )
}

export default RouterConfig
