// CodeMirror, copyright (c) by Marijn Haverbeke and others
// Distributed under an MIT license: https://codemirror.net/LICENSE

// Depends on jshint.js from https://github.com/jshint/jshint

(function(mod) {
  if (typeof exports == "object" && typeof module == "object") // CommonJS
    mod(require("../../lib/codemirror"));
  else if (typeof define == "function" && define.amd) // AMD
    define(["../../lib/codemirror"], mod);
  else // Plain browser env
    mod(CodeMirror);
})(function(CodeMirror) {
"use strict";
CodeMirror.registerHelper("lint", "python", remoteValidator);
function remoteValidator  (text, callback, options, cm) {
	var text = text;
	var found = [];
	function result_cb(obj){
	if(obj && obj.result) {  
	  var  error_list = obj.result; // arr
		for(var i=0; i< error_list.length; i ++ )
		{
			var errorkinds = error_list[i]['edit']['documentChanges']; //外层
			var message = error_list[i]['title']; // added
			for(var j=0;  j< errorkinds.length; j++ ){
			var lastarr = errorkinds[j]['edits'];
			for(var k=0;  k < lastarr.length; k++ ){
				var range = lastarr[k]['range'];
				var start_line = range.start.line;
				var start_char = range.start.character;
				var end_line = range.start.line;
				var end_char = range.start.character;
				var severity;
					if(typeof(range.severity) != "undefined"){
						severity = range.severity;
					}
					else{
						severity = 'error';
					}
					found.push({
						from: CodeMirror.Pos(start_line - 1, start_char),
						to: CodeMirror.Pos(end_line - 1, end_char),
						message: message, //added
						severity: severity // "error", "warning"
					});
			}
			}
		}
		}
		
	//updatelint(found);//更新error
	//	updatelint(found);//更新error
	//	updateLinting(cm, found);
	}

	options.check_cb(result_cb)//调用外部回调
	return found;//更新error
}

});
