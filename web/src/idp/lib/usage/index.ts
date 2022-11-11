import { action, observable } from 'mobx';
import dashboardApi from "@/services/dashboard";

class Usage {

    @observable cpuUsage: Number | String = 0;
    @observable memoryUsage: Number | String = 0;
    @observable gpuUsage: Number | String = 0;
    @observable storageUsage: Number | String = 0;

    @action async getUsageThunk() {
        const result = await dashboardApi.taskMonitorTotal();
        const data = result.data;
        const cpuUsage = (data.items[0].localUsedPercent * 100).toFixed(0);
        const gpuUsage = (data.items[1].localUsedPercent * 100).toFixed(0);
        const memoryUsage = (data.items[2].localUsedPercent * 100).toFixed(0);
        const storageUsage = (data.items[3].localUsedPercent * 100).toFixed(0);
        this.cpuUsage = cpuUsage;
        this.memoryUsage = memoryUsage;
        this.gpuUsage = gpuUsage;
        this.storageUsage = storageUsage;
        return result
    }

    @action selectCpuUsage() {
        return this.cpuUsage
    }

    @action selectGpuUsage() {
        return this.gpuUsage;
    }

    @action selectMemoryUsage() {
        return this.memoryUsage;
    }

    @action selectStorageUsage() {
        return this.storageUsage;
    }
}

export default new Usage()