import _extends from "@babel/runtime/helpers/extends";
import React, { useEffect, useState } from 'react';
import { jsx as _jsx } from "react/jsx-runtime";

var Preview = props => {
  var {
    editorProps: {
      preview,
      containerEditor,
      initialMode
    }
  } = props;
  var [visible, setVisible] = useState(props.editorProps.visible);
  useEffect(() => {
    setVisible(props.editorProps.visible);
  }, [props.editorProps.visible]);
  useEffect(() => {
    if (preview) {
      preview.style.borderBottomRightRadius = '3px';
    }

    if(!initialMode){
      if (preview && visible) {
        preview.style.width = '100%';
        preview.style.overflow = 'auto';
        preview.style.padding = '20px';
  
        if (containerEditor) {
          containerEditor.style.width = '100%';
        }
      } else if (preview) {
        preview.style.overflow = 'hidden';
        preview.style.borderLeft = '0px';
        preview.style.padding = '0';
  
        if (containerEditor) {
          containerEditor.style.width = '100%';
        }
      }
    }else{
      if (preview && visible) {
        preview.style.width = '50%';
        preview.style.overflow = 'auto';
        preview.style.padding = '20px';
        preview.style.display = "block"
        if (containerEditor) {
          containerEditor.style.width = '50%';
        }
      } else if (preview) {
        preview.style.overflow = 'hidden';
        preview.style.borderLeft = '0px';
        preview.style.padding = '0';
        preview.style.display = "none"
        
        if (containerEditor) {
          containerEditor.style.width = '100%';
        }
      }
    }
  }, [preview, containerEditor, visible]);
  return /*#__PURE__*/_jsx("button", {
    onClick: () => setVisible(!visible),
    type: "button",
    className: visible ? 'active' : '',
    children: props.command.icon
  });
};

export var preview = {
  name: 'preview',
  keyCommand: 'preview',
  button: (command, props, opts) => /*#__PURE__*/_jsx(Preview, {
    command: command,
    editorProps: _extends({}, props, opts)
  }),
  icon: /*#__PURE__*/_jsx("svg", {
    width: "16",
    height: "16",
    viewBox: "0 0 32 32",
    children: /*#__PURE__*/_jsx("path", {
      fill: "currentColor",
      d: "M0 16c3.037-5.864 9.058-9.802 16-9.802s12.963 3.938 15.953 9.703l0.047 0.1c-3.037 5.864-9.058 9.802-16 9.802s-12.963-3.938-15.953-9.703l-0.047-0.1zM16 22.531c3.607 0 6.531-2.924 6.531-6.531s-2.924-6.531-6.531-6.531v0c-3.607 0-6.531 2.924-6.531 6.531s2.924 6.531 6.531 6.531v0zM16 19.265c-1.804 0-3.265-1.461-3.265-3.265s1.461-3.265 3.265-3.265v0c1.804 0 3.265 1.461 3.265 3.265s-1.461 3.265-3.265 3.265v0z"
    })
  })
};
//# sourceMappingURL=preview.js.map