import React from 'react';
import intl from 'react-intl-universal';

class AddFileNoPopInput extends React.Component {
    constructor(props) {
        super(props);
        this.modalAddFileInputRef = React.createRef();
        this.inputLock = false;
        this.state = {
            visible: {
                delete: false,
                addFolder: false,
                folderInputDisabled: false,
                folderValue: 'newFolder',
                addFile: false,
                fileInputDisabled: false,
                fileValue: 'newFile.idpnb',
                rename: false,
                renameInputDisabled: false,
                renameValue: '',
                displayWarning: 'none',
                fileNameValidator: '',
                confirmLoading: false,

            }

        }
    }

    getComplete = (value) => {
        const part = value.slice(value.indexOf('.') + 1);
        const suffixs = ['idpnb', 'ipynb', 'py', 'md', 'txt', 'sql', 'json', 'csv'];
        for (const suffix of suffixs) {
            if (suffix.startsWith(part)) {
                return suffix.slice(part.length);
            }
        }
        return null;
    }
    handleInput = (event) => {
        if ('deleteContentBackward' === event.nativeEvent.inputType || this.inputLock){
          this.props.setFileValue(event.currentTarget.value);
          return
        }
        const suffix = '.idpnb';
        let value = event.currentTarget.value;
        const index = event.currentTarget.selectionStart;
        if (value.indexOf('.') > 0) {
            const complete = this.getComplete(value);
            if (complete !== null) {
                event.currentTarget.value = value + complete;
                event.currentTarget.selectionStart = index;
            }
        } else {
            if (value.indexOf(suffix) === -1) {
                event.currentTarget.value = value + suffix;
                event.currentTarget.selectionStart = index;
            }
        }
        this.props.setFileValue(value);
    }

    handleKeyPress = (event) => {
        if ('Enter' === event.key) {
            this.props.setFileValue(event.currentTarget.value);
            if(this.checkFileName(this.state.visible, event.currentTarget.value, this.props.selectedKey,true,true)){
                this.props.onPressEnter();
            }
        }
    }
    findChildren = (key, tree) => {
        for (const node of tree) {
            if (!node.isLeaf) {
                if (key === node.key) {
                    return node.children ? node.children : [];
                } else if (node.children) {
                    const keys = this.findChildren(key, node.children);
                    if (keys.length > 0) return keys;
                }
            }
        }
        return [];
    }

    checkFileName = (visible, value, key, isLeaf, isFile) => {
        if (value.indexOf(' ') >= 0 || value == '') {
            // 加入下部代码 新建文件文件名无法加入空格
            // visible.displayWarning = '';
            // visible.fileNameValidator = intl.get('FILE_NAME_INVALID_2');
            // this.setState({ visible });
            // return false;
        } else if (null === value.match(/^(?!\.)[^\\:\*\?"<>\|]{1,255}$/)) {
            visible.displayWarning = '';
            visible.fileNameValidator = intl.get('FILE_NAME_INVALID_1');
            this.setState({ visible });
            return false;
        } else if(value.length>50){
            visible.displayWarning = ""
            visible.fileNameValidator = "文件或文件夹名长度不能超过50"
            this.setState({
            visible
        })
            return false
        }  else {
            const arr = this.props.treeData;
            this.getNearFile(key, value, arr, (flg)=> {
                if ( flg ) {
                    visible.displayWarning = '';
                    visible.fileNameValidator = isFile ? intl.get('FILE_NAME_INVALID_3') : intl.get('FILE_NAME_INVALID_4');
                    this.setState({ visible, treeData: this.state.treeData });
                }
            });
        }
        return true;
    }

    /**
     * 为了达到快速检索到指定的文件夹children, b 避免深度逐次递归文件名称, 优化此处代码， 思路：解析文件路径，遵循就近原则，检索目标文件
     * @param {*} filesPaths 文件路径 ’/files/xxxxx‘
     * @param {*} fileName 文件名称
     * @param {*} fileList 文件数组列表
     * @param {*} callback 回调处理视图逻辑
     */
    getNearFile = (filesPaths, fileName, fileList, callback) => {
        const fileArr = filesPaths.split('/').filter( it => it != ''); // 从此处判断新建文件夹的深度，数组长度作为深度值
        if ( fileArr.length !== 0 ) {
            let _index = 0; // 作为for循环深度计时器;
            let whileflg = true;
            const filterFiler = function(_fileName, _fileList) {
                if( whileflg ){
                    for( let i = 0; i < _fileList.length; i++ ) {
                        // 因为知道文件深度，直接判断类型， index 作为深度指针，指向当前文件深度对象
                        if ( _fileList[i]['fileType'] === 'DIRECTORY' && _fileList[i]['title'] === fileArr[_index]) {
                            if (  (_index + 1) == fileArr.length ) {
                                const fileFlg = _fileList[i]['children'].some( it => it.title === fileName);
                                _index = 0;
                                whileflg = false;
                                callback(fileFlg);
                                return;
                            } else {
                                _index++;
                                filterFiler(fileName, _fileList[i]['children']);
                            }
                        }
                    }
                }
            };
            filterFiler(fileName, fileList);
        } else {
            const fileFlg = fileList.some( (item) => item['fileType'] === 'FILE' && item['title'] === fileName );
            callback(fileFlg);
        }
    }

    onClick = (e) => {
        e.stopPropagation();
        return false
    };

    componentDidMount() {
        this.modalAddFileInputRef.current.focus();
    }
    componentWillReceiveProps= (nextProps) => {
        // console.log(nextProps);

    };

    render() {
        return(
            <><input
                className="ant-input"
                placeholder={this.props.placeholder}
                defaultValue={this.props.defaultValue}
                style={this.props.style}
                onInput={this.handleInput}
                onCompositionStart={() => this.inputLock = true}
                onCompositionEnd={() => this.inputLock = false}
                onKeyPress={this.handleKeyPress}
                disabled={this.props.disabled}
                onFocus={(event) => event.target.select()}
                onBlur={this.props.onBlur}
                onClick={this.onClick}
                ref={this.modalAddFileInputRef} />
                <div className="addfile-tips" style={{display:this.state.visible.displayWarning}}>{this.state.visible.fileNameValidator}</div>
                </>
        );
    }
}

export default AddFileNoPopInput;
