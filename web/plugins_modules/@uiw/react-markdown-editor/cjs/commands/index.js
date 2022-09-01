"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.getModeCommands = exports.getCommands = exports.defaultCommands = void 0;

var _bold = require("./bold");

var _italic = require("./italic");

var _header = require("./header");

var _strike = require("./strike");

var _underline = require("./underline");

var _olist = require("./olist");

var _ulist = require("./ulist");

var _link = require("./link");

var _todo = require("./todo");

var _image = require("./image");

var _fullscreen = require("./fullscreen");

var _preview = require("./preview");

var defaultCommands = {
  bold: _bold.bold,
  italic: _italic.italic,
  header: _header.header,
  strike: _strike.strike,
  underline: _underline.underline,
  olist: _olist.olist,
  ulist: _ulist.ulist,
  link: _link.link,
  todo: _todo.todo,
  image: _image.image,
  fullscreen: _fullscreen.fullscreen,
  preview: _preview.preview
};
exports.defaultCommands = defaultCommands;

var getCommands = function getCommands() {
  return Object.keys(defaultCommands).filter(function (key) {
    return !/^(fullscreen|preview)/.test(key);
  }).map(function (key) {
    return defaultCommands[key];
  });
};

exports.getCommands = getCommands;

var getModeCommands = function getModeCommands() {
  return [_preview.preview, _fullscreen.fullscreen];
};

exports.getModeCommands = getModeCommands;
//# sourceMappingURL=index.js.map