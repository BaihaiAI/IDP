"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.header = void 0;

var _react = _interopRequireDefault(require("react"));

var _jsxRuntime = require("react/jsx-runtime");

var header = {
  name: 'header',
  keyCommand: 'header',
  button: {
    'aria-label': 'Add header text'
  },
  icon: /*#__PURE__*/(0, _jsxRuntime.jsx)("svg", {
    width: "12",
    height: "12",
    viewBox: "0 0 512 512",
    children: /*#__PURE__*/(0, _jsxRuntime.jsx)("path", {
      fill: "currentColor",
      d: "M496 80V48c0-8.837-7.163-16-16-16H320c-8.837 0-16 7.163-16 16v32c0 8.837 7.163 16 16 16h37.621v128H154.379V96H192c8.837 0 16-7.163 16-16V48c0-8.837-7.163-16-16-16H32c-8.837 0-16 7.163-16 16v32c0 8.837 7.163 16 16 16h37.275v320H32c-8.837 0-16 7.163-16 16v32c0 8.837 7.163 16 16 16h160c8.837 0 16-7.163 16-16v-32c0-8.837-7.163-16-16-16h-37.621V288H357.62v128H320c-8.837 0-16 7.163-16 16v32c0 8.837 7.163 16 16 16h160c8.837 0 16-7.163 16-16v-32c0-8.837-7.163-16-16-16h-37.275V96H480c8.837 0 16-7.163 16-16z"
    })
  }),
  execute: function execute(editor, selection, position) {
    var value = selection ? "# ".concat(selection) : '# ';
    editor.replaceSelection(value);
    position.ch = !!selection ? position.ch : position.ch + 2;
    editor.setCursor(position.line, position.ch);
    editor.focus();
  }
};
exports.header = header;
//# sourceMappingURL=header.js.map