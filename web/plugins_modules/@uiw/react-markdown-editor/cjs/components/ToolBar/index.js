"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = ToolBar;

var _objectSpread2 = _interopRequireDefault(require("@babel/runtime/helpers/objectSpread2"));

var _typeof2 = _interopRequireDefault(require("@babel/runtime/helpers/typeof"));

var _toConsumableArray2 = _interopRequireDefault(require("@babel/runtime/helpers/toConsumableArray"));

var _objectWithoutProperties2 = _interopRequireDefault(require("@babel/runtime/helpers/objectWithoutProperties"));

var React = _interopRequireWildcard(require("react"));

var _commands = require("../../commands");

var _jsxRuntime = require("react/jsx-runtime");

var _excluded = ["prefixCls", "className", "onClick", "toolbars", "editor", "mode", "preview", "container", "containerEditor", "editorProps"];

function ToolBar(props) {
  var _props$prefixCls = props.prefixCls,
      prefixCls = _props$prefixCls === void 0 ? 'md-editor' : _props$prefixCls,
      className = props.className,
      onClick = props.onClick,
      _props$toolbars = props.toolbars,
      toolbars = _props$toolbars === void 0 ? [] : _props$toolbars,
      editor = props.editor,
      mode = props.mode,
      preview = props.preview,
      container = props.container,
      containerEditor = props.containerEditor,
      _props$editorProps = props.editorProps,
      editorProps = _props$editorProps === void 0 ? {} : _props$editorProps,
      htmlProps = (0, _objectWithoutProperties2.default)(props, _excluded);
  if (!toolbars || toolbars.length === 0) return null;

  function handleClick(execute) {
    if (execute && editor) {
      execute(editor, editor.getSelection(), editor.getCursor(), {
        preview: preview,
        container: container
      });
    }
  }

  return /*#__PURE__*/(0, _jsxRuntime.jsx)("div", (0, _objectSpread2.default)((0, _objectSpread2.default)({
    className: "".concat(prefixCls, "-toolbar ").concat(className || '', " ").concat(mode ? "".concat(prefixCls, "-toolbar-mode") : '')
  }, htmlProps), {}, {
    children: (0, _toConsumableArray2.default)(toolbars).map(function (command, key) {
      var buttonProps = {
        type: 'button'
      };
      var obj = typeof command === 'string' ? _commands.defaultCommands[command] : command;
      if (!obj) return null;
      buttonProps.children = obj.icon;

      buttonProps.onClick = function () {
        return handleClick(obj.execute);
      };

      if (obj.button && (0, _typeof2.default)(obj.button) === 'object') {
        var btn = obj.button;
        Object.keys(btn).forEach(function (key) {
          buttonProps[key] = btn[key];
        });
      } else if (typeof obj.button === 'function') {
        return /*#__PURE__*/React.cloneElement(obj.button(obj, editorProps, {
          preview: preview,
          container: container,
          containerEditor: containerEditor,
          editor: editor
        }), {
          key: key
        });
      }

      return /*#__PURE__*/(0, React.createElement)("button", (0, _objectSpread2.default)((0, _objectSpread2.default)({}, buttonProps), {}, {
        key: key
      }));
    })
  }));
}

module.exports = exports.default;
//# sourceMappingURL=index.js.map