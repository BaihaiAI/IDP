const extensions = require('./config.json')

const loadExamplePlugins = (function () {
    extensions.forEach(async extension => {
        await require('./' + extension.entry);
    });
})()

export default loadExamplePlugins
