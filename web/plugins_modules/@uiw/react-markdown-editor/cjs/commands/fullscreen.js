"use strict";

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.fullscreen = void 0;

var _objectSpread2 = _interopRequireDefault(require("@babel/runtime/helpers/objectSpread2"));

var _slicedToArray2 = _interopRequireDefault(require("@babel/runtime/helpers/slicedToArray"));

var _react = _interopRequireWildcard(require("react"));

var _jsxRuntime = require("react/jsx-runtime");

var Fullscreen = function Fullscreen(props) {
  var _props$editorProps = props.editorProps,
      container = _props$editorProps.container,
      preview = _props$editorProps.preview,
      editor = _props$editorProps.editor;

  var _useState = (0, _react.useState)(false),
      _useState2 = (0, _slicedToArray2.default)(_useState, 2),
      full = _useState2[0],
      setFull = _useState2[1];

  var initEditorHeight = (0, _react.useRef)(0);
  var containerRef = (0, _react.useRef)();
  var editorRef = (0, _react.useRef)();

  function handleResize() {
    if (containerRef.current && editorRef.current) {
      editorRef.current.setSize('initial', containerRef.current.clientHeight - 35);
    }
  }

  (0, _react.useEffect)(function () {
    window && window.addEventListener('resize', handleResize, true);
    return function () {
      window && window.removeEventListener('resize', handleResize, true);
    };
  }, []);
  (0, _react.useEffect)(function () {
    if (editor) {
      editorRef.current = editor;

      var _editor$getScrollInfo = editor.getScrollInfo(),
          clientHeight = _editor$getScrollInfo.clientHeight;

      initEditorHeight.current = clientHeight;
    }
  }, [editor]);
  (0, _react.useEffect)(function () {
    if (!document) return;
    containerRef.current = container;
    document.body.style.overflow = full ? 'hidden' : 'initial';

    if (container && full) {
      container.style.zIndex = '999';
      container.style.position = 'fixed';
      container.style.top = '0px';
      container.style.bottom = '0px';
      container.style.left = '0px';
      container.style.right = '0px';
      editor.setSize('initial', container.clientHeight - 35);
    } else if (container) {
      container.style.position = 'initial';
      container.style.top = 'initial';
      container.style.bottom = 'initial';
      container.style.left = 'initial';
      container.style.right = 'initial';
      editor.setSize('initial', initEditorHeight.current);
    }
  }, [full, container, preview]);
  return /*#__PURE__*/(0, _jsxRuntime.jsx)("button", {
    onClick: function onClick() {
      return setFull(!full);
    },
    type: "button",
    className: full ? 'active' : '',
    children: props.command.icon
  });
};

var fullscreen = {
  name: 'fullscreen',
  keyCommand: 'fullscreen',
  button: function button(command, props, opts) {
    return /*#__PURE__*/(0, _jsxRuntime.jsx)(Fullscreen, {
      command: command,
      editorProps: (0, _objectSpread2.default)((0, _objectSpread2.default)({}, props), opts)
    });
  },
  icon: /*#__PURE__*/(0, _jsxRuntime.jsxs)("svg", {
    width: "16",
    height: "16",
    viewBox: "0 0 1024 1024",
    children: [/*#__PURE__*/(0, _jsxRuntime.jsx)("path", {
      fill: "currentColor",
      d: "M189.75 428.89a36.87 36.87 0 0 0 36.84-36.85V228.12h164a36.85 36.85 0 1 0 0-73.7H189.75a36.82 36.82 0 0 0-36.8 36.85v200.8a36.83 36.83 0 0 0 36.8 36.82zM834.26 595.06a36.82 36.82 0 0 0-36.8 36.84v164H633.41a36.85 36.85 0 0 0 0 73.7h200.85a36.87 36.87 0 0 0 36.84-36.85V631.9a36.86 36.86 0 0 0-36.84-36.84zM797.46 228.12v179.31a36.82 36.82 0 1 0 73.64 0V191.24a36.86 36.86 0 0 0-36.84-36.85H602.33a36.85 36.85 0 0 0 0 73.7zM421.62 795.9H226.54V616.56a36.82 36.82 0 1 0-73.64 0v216.19a36.83 36.83 0 0 0 36.85 36.85h231.87a36.85 36.85 0 0 0 0-73.7z"
    }), /*#__PURE__*/(0, _jsxRuntime.jsx)("path", {
      fill: "currentColor",
      d: "M306.5 307.94m32.95 0l345.1 0q32.95 0 32.95 32.95l0 342.22q0 32.95-32.95 32.95l-345.1 0q-32.95 0-32.95-32.95l0-342.22q0-32.95 32.95-32.95Z"
    })]
  })
};
exports.fullscreen = fullscreen;
//# sourceMappingURL=fullscreen.js.map