const fs = require('fs');
const path = require('path');

function copyDir(srcDir, desDir) {
    // console.log(srcDir);
    fs.readdir(srcDir, { withFileTypes: true }, (err, files) => {
        if (files) {
            for (const file of files) {
                if (file.isDirectory()) {
                    const dirS = path.resolve(srcDir, file.name);
                    const dirD = path.resolve(desDir, file.name);
                    if (!fs.existsSync(dirD)) {
                        fs.mkdir(dirD, (err) => {
                            if (err) console.log(err);
                        });
                    }
                    copyDir(dirS, dirD);
                } else {
                    const srcFile = path.resolve(srcDir, file.name);
                    const desFile = path.resolve(desDir, file.name);
                    fs.copyFileSync(srcFile, desFile);
                }
            }
        }
    })
}

function createDistExtensionFiles(callback) {
    let distFilesFlg = false;
    fs.readdir(__dirname, { withFileTypes: true }, (err, files) => {
        const flg = files.some(it => it.name === 'dist_extension');
        if (!flg) {
            fs.mkdirSync(path.relative(__dirname, 'dist_extension'));
            distFilesFlg = true;
        }
    })
    callback(distFilesFlg)
}

const createPluginDistFiles = () => {
    fs.mkdir(path.resolve(__dirname, 'extension'), () => {
        fs.readdir('extension', { withFileTypes: true }, (err, files) => {
            for (const file of files) {
                if (file.isDirectory()) {
                    copyDir(path.resolve(`extension/${file.name}/dist`), `dist_extension`)
                }
            }
        })
    });
}

createDistExtensionFiles((data) => {
    createPluginDistFiles();
})

