import React from 'react';
import { Input } from 'antd';
import intl from 'react-intl-universal';

class AddFolderNoPopInput extends React.Component {
    constructor(props) {
        super(props);
        this.addFolderInputRef = React.createRef();
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

    componentDidMount() {
        this.addFolderInputRef.current.focus();
    }
    onClick = (e) => {
        e.stopPropagation();
        return false
    };
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
        } else {
            const arr = this.props.treeData;
            let childArr = [...arr];
            if ('/' !== key) {
                const dir = isLeaf ? key.slice(0, key.lastIndexOf('/')) : key;
                if (dir !== '') {
                    childArr = this.findChildren(dir, arr);
                }
            }
            for (const item of childArr) {
                if (value === item.title) {
                    visible.displayWarning = '';
                    visible.fileNameValidator = isFile ? intl.get('FILE_NAME_INVALID_3') : intl.get('FILE_NAME_INVALID_4');
                    this.setState({ visible });

                    return false;
                }
            }
        }
        return true;
    }
    onPress = (e) => {
        if(this.checkFileName(this.state.visible, e.currentTarget.value, this.props.selectedKey,this.props.isLeaf,false)){
            this.props.onPressEnter(e)
        }
    };

    render() {
        return (
        <>
            <Input
                placeholder={this.props.placeholder}
                defaultValue={this.props.defaultValue}
                onChange={(event) => this.props.onChange(event)}
                onPressEnter={this.onPress}
                disabled={this.props.folderInputDisabled}
                ref={this.addFolderInputRef}
                onBlur={this.props.onBlur}
                onFocus={() => this.addFolderInputRef.current.focus({cursor: 'all'})}
                style={this.props.style}
                onClick={this.onClick}
            />
            <div className="addfile-tips" style={{ display: this.state.visible.displayWarning }}>{this.state.visible.fileNameValidator}</div>
        </>
        );
    }
}

export default AddFolderNoPopInput;
