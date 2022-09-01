// 接受远程代码检测语法的插件

(function(mod) {
  if (typeof exports == "object" && typeof module == "object") // CommonJS
    mod(require("../../lib/codemirror"));
  else if (typeof define == "function" && define.amd) // AMD
    define(["../../lib/codemirror"], mod);
  else // Plain browser env
    mod(CodeMirror);
})(function(CodeMirror) {
  "use strict";
  // declare global: JSHINT

  function validator(err, options) {
	var  result = [];
    if (err) parseErrors(err, result);
    return result;
  }

  CodeMirror.registerHelper("lint", "python", validator); //先注册给python mode=python时可以直接用lint：true打开

  function parseErrors(errors, output) {
    for ( var i = 0; i < errors.length; i++) {
      var error = errors[i];
      if (error) {
        var start = error.character - 1, end = start + 1;
        if (error.evidence) {
            end += index;
        }
        // Convert to format expected by validation service
        var hint = {
          message: error.reason,
          severity: error.code ? (error.code.startsWith('W') ? "warning" : "error") : "error",
          from: CodeMirror.Pos(error.line - 1, start),
          to: CodeMirror.Pos(error.line - 1, end)
        };

        output.push(hint);
      }
    }
  }
});
