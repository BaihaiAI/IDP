import { useEffect, useRef } from "react";
import { observer } from "mobx-react";
import usage from 'idp/lib/usage';
import globalData from "idp/global";

function UsageFooterBar() {
    
    const cpuUsage = usage.cpuUsage;
    const memoryUsage = usage.memoryUsage;
    const gpuUsage = usage.gpuUsage;
    const storageUsage = usage.storageUsage;
    const intervalRef = useRef()

    const network = (event) => {
        globalData.appComponentData.setSocketAlive(navigator.onLine)
    }

    const getResource = () => {
        usage.getUsageThunk().then(() => {
            globalData.appComponentData.setSocketAlive(true)
        }).catch(() => {
            globalData.appComponentData.setSocketAlive(false)
        })
    }

    useEffect(() => {
        window.addEventListener("online", network)
        window.addEventListener("offline", network)

        intervalRef.current = window.setInterval(getResource, 5000)

        return function cleanup() {
            window.clearInterval(intervalRef.current)
            window.removeEventListener("online", network)
            window.removeEventListener("offline", network)
        }
    }, [])

    useEffect(() => {
        window.addEventListener('visibilitychange', () => {
            const { visibilityState } = document
            if (visibilityState === 'hidden') {
                window.clearInterval(intervalRef.current)
            } else {
                intervalRef.current = window.setInterval(getResource, 5000)
            }
        })
    }, [])


    return (
        <div>
            CPU: <span id="footerbar-cpu">{cpuUsage}%</span>
            GPU: <span id="footerbar-gpu">{gpuUsage}%</span>
            Mem: <span id="footerbar-mem">{memoryUsage}%</span>
            Storage: <span id="footerbar-storage">{storageUsage}%</span>
        </div>
    )
}

export default observer(UsageFooterBar)
