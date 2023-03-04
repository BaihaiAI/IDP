import React from 'react';

class AddFileInput extends React.Component {
    constructor(props) {
        super(props);
        this.modalAddFileInputRef = React.createRef();
        this.inputLock = false;
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
        if ('deleteContentBackward' === event.nativeEvent.inputType || this.inputLock) {
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
        this.props.setFileValue(event.currentTarget.value);
    }

    handleKeyPress = (event) => {
        if ('Enter' === event.key) {
            this.props.setFileValue(event.currentTarget.value);
            this.props.onPressEnter();
        }
    }

    componentDidMount() {
        this.modalAddFileInputRef.current.focus();
    }

    render() {
        return(
            <input
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
              ref={this.modalAddFileInputRef}
            />
        );
    }
}

export default AddFileInput;
