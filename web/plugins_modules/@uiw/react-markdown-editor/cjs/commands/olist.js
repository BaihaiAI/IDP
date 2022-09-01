"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.olist = void 0;

var _react = _interopRequireDefault(require("react"));

var _jsxRuntime = require("react/jsx-runtime");

var olist = {
  name: 'olist',
  keyCommand: 'olist',
  button: {
    'aria-label': 'Add olist text'
  },
  icon: /*#__PURE__*/(0, _jsxRuntime.jsx)("svg", {
    width: "12",
    height: "12",
    viewBox: "0 0 32 32",
    children: /*#__PURE__*/(0, _jsxRuntime.jsx)("path", {
      fill: "currentColor",
      d: "M12 2h20v4h-20v-4zM12 14h20v4h-20v-4zM12 26h20v4h-20v-4zM0 4c0 2.209 1.791 4 4 4s4-1.791 4-4c0-2.209-1.791-4-4-4s-4 1.791-4 4zM0 16c0 2.209 1.791 4 4 4s4-1.791 4-4c0-2.209-1.791-4-4-4s-4 1.791-4 4zM0 28c0 2.209 1.791 4 4 4s4-1.791 4-4c0-2.209-1.791-4-4-4s-4 1.791-4 4z"
    })
  }),
  execute: function execute(editor, selection, position) {
    var value = selection ? "- ".concat(selection) : '- ';
    editor.replaceSelection(value);
    position.ch = !!selection ? position.ch : position.ch + 2;
    editor.setCursor(position.line, position.ch);
    editor.focus();
  }
};
exports.olist = olist;
//# sourceMappingURL=olist.js.map