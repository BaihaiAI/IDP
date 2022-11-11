"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
var _exportNames = {};
exports.default = void 0;

var _objectSpread2 = _interopRequireDefault(require("@babel/runtime/helpers/objectSpread2"));

var _slicedToArray2 = _interopRequireDefault(require("@babel/runtime/helpers/slicedToArray"));

var _objectWithoutProperties2 = _interopRequireDefault(require("@babel/runtime/helpers/objectWithoutProperties"));

var _react = _interopRequireWildcard(require("react"));

var _CodeMirror = _interopRequireDefault(require("./components/CodeMirror"));

var _reactMarkdownPreview = _interopRequireDefault(require("@uiw/react-markdown-preview"));

var _ToolBar = _interopRequireDefault(require("./components/ToolBar"));

var _commands = require("./commands");

Object.keys(_commands).forEach(function (key) {
  if (key === "default" || key === "__esModule") return;
  if (Object.prototype.hasOwnProperty.call(_exportNames, key)) return;
  if (key in exports && exports[key] === _commands[key]) return;
  Object.defineProperty(exports, key, {
    enumerable: true,
    get: function get() {
      return _commands[key];
    }
  });
});

var _jsxRuntime = require("react/jsx-runtime");

var _excluded = ["prefixCls", "className", "onChange", "toolbars", "toolbarsMode", "visible", "visibleEditor", "previewProps"];

var _default = /*#__PURE__*/_react.default.forwardRef(MarkdownEditor);

exports.default = _default;

function MarkdownEditor(props, ref) {
  var _props$prefixCls = props.prefixCls,
      prefixCls = _props$prefixCls === void 0 ? 'md-editor' : _props$prefixCls,
      className = props.className,
      _onChange = props.onChange,
      _props$toolbars = props.toolbars,
      toolbars = _props$toolbars === void 0 ? (0, _commands.getCommands)() : _props$toolbars,
      _props$toolbarsMode = props.toolbarsMode,
      toolbarsMode = _props$toolbarsMode === void 0 ? (0, _commands.getModeCommands)() : _props$toolbarsMode,
      _props$visible = props.visible,
      visible = _props$visible === void 0 ? true : _props$visible,
      _props$visibleEditor = props.visibleEditor,
      visibleEditor = _props$visibleEditor === void 0 ? true : _props$visibleEditor,
      _props$previewProps = props.previewProps,
      previewProps = _props$previewProps === void 0 ? {} : _props$previewProps,
      codemirrorProps = (0, _objectWithoutProperties2.default)(props, _excluded);

  var _useState = (0, _react.useState)(props.value || ''),
      _useState2 = (0, _slicedToArray2.default)(_useState, 2),
      value = _useState2[0],
      setValue = _useState2[1];

  var codeMirror = /*#__PURE__*/(0, _react.createRef)();
  var previewContainer = (0, _react.useRef)();

  var _useState3 = (0, _react.useState)(),
      _useState4 = (0, _slicedToArray2.default)(_useState3, 2),
      editor = _useState4[0],
      setEditor = _useState4[1];

  var container = (0, _react.useRef)(null);
  var containerEditor = (0, _react.useRef)(null);
  (0, _react.useImperativeHandle)(ref, function () {
    return {
      editor: editor,
      preview: previewContainer.current
    };
  });
  (0, _react.useEffect)(function () {
    if (codeMirror.current) {
      setEditor(codeMirror.current.editor);
    }
  }, [codeMirror.current]);
  var toolBarProps = {
    editor: editor,
    preview: previewContainer.current,
    container: container.current,
    containerEditor: containerEditor.current,
    editorProps: props
  };
  var codeEditor = (0, _react.useMemo)(function () {
    return /*#__PURE__*/(0, _jsxRuntime.jsx)(_CodeMirror.default, (0, _objectSpread2.default)((0, _objectSpread2.default)({
      visibleEditor: visibleEditor
    }, codemirrorProps), {}, {
      ref: codeMirror,
      onChange: function onChange(editor, data) {
        setValue(editor.getValue());
        _onChange && _onChange(editor, data, editor.getValue());
      }
    }));
  }, [visibleEditor, codemirrorProps]);
  return /*#__PURE__*/(0, _jsxRuntime.jsx)("div", {
    ref: container,
    children: /*#__PURE__*/(0, _jsxRuntime.jsxs)("div", {
      className: "".concat(prefixCls || '', " ").concat(className || ''),
      children: [/*#__PURE__*/(0, _jsxRuntime.jsx)(_ToolBar.default, (0, _objectSpread2.default)((0, _objectSpread2.default)({}, toolBarProps), {}, {
        toolbars: toolbarsMode,
        mode: true
      })), /*#__PURE__*/(0, _jsxRuntime.jsx)(_ToolBar.default, (0, _objectSpread2.default)((0, _objectSpread2.default)({}, toolBarProps), {}, {
        toolbars: toolbars
      })), /*#__PURE__*/(0, _jsxRuntime.jsxs)("div", {
        className: "".concat(prefixCls, "-content"),
        children: [/*#__PURE__*/(0, _jsxRuntime.jsx)("div", {
          className: "".concat(prefixCls, "-content-editor"),
          ref: containerEditor,
          children: visibleEditor && codeEditor
        }), /*#__PURE__*/(0, _jsxRuntime.jsx)(_reactMarkdownPreview.default, (0, _objectSpread2.default)((0, _objectSpread2.default)({}, previewProps), {}, {
          "data-visible": !!visible,
          className: "".concat(prefixCls, "-preview"),
          ref: function ref(instance) {
            if (instance && instance.mdp) {
              previewContainer.current = instance.mdp.current;
            }
          },
          source: value
        }))]
      })]
    })
  });
}
//# sourceMappingURL=index.js.map