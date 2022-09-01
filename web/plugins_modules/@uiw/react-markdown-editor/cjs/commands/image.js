"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault").default;

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.image = void 0;

var _react = _interopRequireDefault(require("react"));

var _jsxRuntime = require("react/jsx-runtime");

var image = {
  name: 'image',
  keyCommand: 'image',
  button: {
    'aria-label': 'Add image text'
  },
  icon: /*#__PURE__*/(0, _jsxRuntime.jsx)("svg", {
    width: "14",
    height: "14",
    viewBox: "0 0 20 20",
    children: /*#__PURE__*/(0, _jsxRuntime.jsx)("path", {
      fill: "currentColor",
      d: "M15 9c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm4-7H1c-.55 0-1 .45-1 1v14c0 .55.45 1 1 1h18c.55 0 1-.45 1-1V3c0-.55-.45-1-1-1zm-1 13l-6-5-2 2-4-5-4 8V4h16v11z"
    })
  }),
  execute: function execute(editor, selection, position) {
    var value = selection ? "".concat(selection, " ![]()") : '![]()\n';
    editor.replaceSelection(value); // position.ch = !!selection ? position.ch : position.ch + 1;

    editor.setCursor(position.line, position.ch);
    editor.focus();
  }
};
exports.image = image;
//# sourceMappingURL=image.js.map