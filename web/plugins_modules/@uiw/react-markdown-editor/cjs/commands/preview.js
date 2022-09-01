"use strict";

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.preview = void 0;

var _objectSpread2 = _interopRequireDefault(require("@babel/runtime/helpers/objectSpread2"));

var _slicedToArray2 = _interopRequireDefault(require("@babel/runtime/helpers/slicedToArray"));

var _react = _interopRequireWildcard(require("react"));

var _jsxRuntime = require("react/jsx-runtime");

var Preview = function Preview(props) {
  var _props$editorProps = props.editorProps,
      preview = _props$editorProps.preview,
      containerEditor = _props$editorProps.containerEditor,
      initialMode = _props$editorProps.initialMode;

  var _useState = (0, _react.useState)(props.editorProps.visible),
      _useState2 = (0, _slicedToArray2.default)(_useState, 2),
      visible = _useState2[0],
      setVisible = _useState2[1];

  (0, _react.useEffect)(function () {
    setVisible(props.editorProps.visible);
  }, [props.editorProps.visible]);
  (0, _react.useEffect)(function () {
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
  return /*#__PURE__*/(0, _jsxRuntime.jsx)("button", {
    onClick: function onClick() {
      return setVisible(!visible);
    },
    type: "button",
    className: visible ? 'active' : '',
    children: props.command.icon
  });
};

var preview = {
  name: 'preview',
  keyCommand: 'preview',
  button: function button(command, props, opts) {
    return /*#__PURE__*/(0, _jsxRuntime.jsx)(Preview, {
      command: command,
      editorProps: (0, _objectSpread2.default)((0, _objectSpread2.default)({}, props), opts)
    });
  },
  icon: /*#__PURE__*/(0, _jsxRuntime.jsx)("svg", {
    width: "16",
    height: "16",
    viewBox: "0 0 32 32",
    children: /*#__PURE__*/(0, _jsxRuntime.jsx)("path", {
      fill: "currentColor",
      d: "M0 16c3.037-5.864 9.058-9.802 16-9.802s12.963 3.938 15.953 9.703l0.047 0.1c-3.037 5.864-9.058 9.802-16 9.802s-12.963-3.938-15.953-9.703l-0.047-0.1zM16 22.531c3.607 0 6.531-2.924 6.531-6.531s-2.924-6.531-6.531-6.531v0c-3.607 0-6.531 2.924-6.531 6.531s2.924 6.531 6.531 6.531v0zM16 19.265c-1.804 0-3.265-1.461-3.265-3.265s1.461-3.265 3.265-3.265v0c1.804 0 3.265 1.461 3.265 3.265s-1.461 3.265-3.265 3.265v0z"
    })
  })
};
exports.preview = preview;
//# sourceMappingURL=preview.js.map