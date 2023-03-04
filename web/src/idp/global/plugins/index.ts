export type PluginsConfigInfo = {
    fileName: string;
    entry: string;
    version: string;
    description: string;
    publisher: string;
    icon: string
}

class PluginsConfig {

    pluginsConfig: PluginsConfigInfo;

    constructor(pluginsConfig: PluginsConfigInfo) {
        this.pluginsConfig = pluginsConfig;
    };
}

export default PluginsConfig;