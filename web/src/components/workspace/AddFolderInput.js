import React from 'react';
import { Input } from 'antd';

class AddFolderInput extends React.Component {
    constructor(props) {
        super(props);
        this.addFolderInputRef = React.createRef();
    }

    componentDidMount() {
        this.addFolderInputRef.current.focus();
    }
    render() {
        return (
            <Input 
                placeholder={this.props.placeholder}  
                defaultValue={this.props.defaultValue}
                onChange={(event) => this.props.onChange(event)}
                onPressEnter={this.props.onPressEnter}
                disabled={this.props.folderInputDisabled}
                ref={this.addFolderInputRef}
                onFocus={() => this.addFolderInputRef.current.focus({cursor: 'all'})}
            />
        );
    }
}

export default AddFolderInput;