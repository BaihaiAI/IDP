import { action, observable } from 'mobx';

class LoadPlugins {

    constructor() { };

    @observable currentLoadPluginRecord: any = { name: '', version: '' };
    @observable currentLoadPluginSize: number = 0;
    @observable pluginSize: number = 0;

    @action updateCurrentLoadPluginRecord(currentLoadPluginRecord: any) {
        this.currentLoadPluginRecord = currentLoadPluginRecord;
    }

    @action updateCurrentLoadPluginSize(currentLoadPluginSize: number = 0) {
        this.currentLoadPluginSize = currentLoadPluginSize;
    }

    @action updatePluginSize(pluginSize: number = 0) {
        this.pluginSize = pluginSize;
    }
}

export default new LoadPlugins()