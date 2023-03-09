const fs = require('fs');
const path = require('path');
const { version } = require('./package.json');

const filePath = `dist`;
const pluginFilePath = `src`;

let pluginVersion = version;
if (Boolean(process.env.LOCAL)) {
  pluginVersion = '';
}

fs.readdir(path.resolve(__dirname, filePath), { withFileTypes: true }, (err, files) => {
    for (const file of files) {
        if (file.isDirectory()) {
            const src_f = fs.readdirSync(path.join(__dirname, `${pluginFilePath}`), { withFileTypes: true });
            const dist_f = fs.readdirSync(path.join(__dirname, `${filePath}/${file.name}`), { withFileTypes: true });
            const config = src_f.filter(it => it.name === 'config.json' && it.isFile());
            if (config.length > 0) {
                const configData = fs.readFileSync(path.join(__dirname, `${pluginFilePath}/config.json`), 'utf-8');
                const cData = JSON.parse(configData);
                const data = Object.assign({}, cData, { entry: 'index.js', name: cData.fileName, version: pluginVersion });
                delete data.fileName;
                setTimeout(() => {
                    console.log(`${filePath}/${file.name}/${pluginVersion}/config.json`);
                    fs.writeFileSync(path.join(__dirname, `${filePath}/${file.name}/${pluginVersion}/config.json`), JSON.stringify(data));
                    if (data.icon && /\.(png|jpe?g|gif|svg)(\?.*)?$/.test(data.icon)) {
                        const icons = data.icon.split('/').filter(it => it != '');
                        let src_icon_path = '';
                        let src_icon_name = '';
                        if (icons.length > 1) {
                            for (let i = 0; i < icons.length; i++) {
                                if (i === (icons.length - 1)) {
                                    src_icon_name = icons[i];
                                } else {
                                    src_icon_path += `/${icons[i]}`;
                                    createDri(path.resolve(__dirname, `${filePath}/${file.name}/${pluginVersion}${src_icon_path}`), icons.length - 2 == i ? (cdata) => {
                                        fs.copyFileSync(path.join(__dirname, `${pluginFilePath}${src_icon_path}/${src_icon_name}`), path.join(__dirname, `${filePath}/${file.name}/${pluginVersion}${src_icon_path}/${src_icon_name}`));
                                    } : undefined);
                                }
                            }
                        } else {
                            fs.copyFileSync(path.join(__dirname, `${pluginFilePath}/${data.icon}`), path.join(__dirname, `${filePath}/${file.name}/${pluginVersion}/${data.icon}`));
                        }
                    }
                });
            }
        }
    };
});

function createDri(pathDir, callback) {
    const _path = path.resolve(__dirname, pathDir);
    fs.readdir(_path, { withFileTypes: true }, (err, files) => {
        if (!files) {
            fs.mkdirSync(_path);
            callback && callback(true);
        }
    });
}