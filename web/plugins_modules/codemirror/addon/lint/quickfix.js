// CodeMirror, copyright (c) by Marijn Haverbeke and others
// Distributed under an MIT license: https://codemirror.net/LICENSE

(function(mod) {
  if (typeof exports == "object" && typeof module == "object") // CommonJS
    mod(require("../../lib/codemirror"));
  else if (typeof define == "function" && define.amd) // AMD
    define(["../../lib/codemirror"], mod);
  else // Plain browser env
    mod(CodeMirror);
})(function(CodeMirror) {
  "use strict";
  var GUTTER_ID = "CodeMirror-lint-markers";
  var LINT_LINE_ID = "CodeMirror-lint-line-";
  var hasTooltip = null;
  var hasQuickBox = null;

  function showTooltip(cm, e, content, posdata) {
    var target = e.target; //传递给clearMark
    var pos = target.getBoundingClientRect();
    var tt = document.createElement("div");
    var bd= document.createElement("div");
    var id = 'T_T_ID' + Math.floor(Math.random(10000)*10000);
    tt.className = "CodeMirror-lint-tooltip cm-quickfix-box cm-s-" + cm.options.theme;
    tt.id = id;
    bd.className = "cm-quickfix-bd" ;
    tt.appendChild(bd);
    bd.appendChild(content.cloneNode(true));
    var quickFixNode = document.createElement('div')
    quickFixNode.className = 'quickfix-item CodeMirror-lint-message'
    quickFixNode.innerHTML ='快速修复'
    quickFixNode.style.color = '#3793EF'

    bd.appendChild(quickFixNode)
    CodeMirror.on(tt,  'click',  function(e){
      var current = e.target;
      var len = content.children.length;
      var data = null;
      if(current.className.includes("quickfix-item")){
        data = posdata;
      }else{
        for(var i=0; i < len; i++){
          if(content.children[i].innerText == current.innerText){
            data = posdata[i];
            break;
          }
        }
      }

      var showbox = createQuickfixBox(id);
      showbox.showLoading(pos);

      cm.state.quickfix.options.quickfix_callback(cm, content, data, function (data, type) {
        //    showQuickFixList(data,cm, pos, target);
        if ('install' === type) {
          showbox.addInstall(data, target, pos, cm);
        } else {
          showbox.addContent(data, target, pos, cm);
        }

      });
      hideTooltip(tt);
      //clearMarks(cm); //清楚标记由change事件重新添加mark
    });

    if (cm.state.quickfix.options.selfContain){
      cm.getWrapperElement().appendChild(tt);
    }else{
      document.body.appendChild(tt);
    }

    tt.style.top = Math.max(pos.top - tt.offsetHeight - 5 ) + "px";
    tt.style.left =Math.max(pos.left) + "px";
    CodeMirror.on(tt, 'mouseleave',function (e) {
      rm(tt);
    });

    hasTooltip = tt;
    return tt;
  }


  //构建quickfix提示框控件
  function createQuickfixBox (id) {
    if (hasQuickBox) rm(hasQuickBox);
    var box = document.createElement('div');
    box.id = id;
    document.body.appendChild(box)
    hasQuickBox = box;
    function showLoading(pos){
      box.innerHTML = '<div class="loading"><div class="circle circle1"><span></span><span></span><span></span><span></span></div><div class="circle circle2"><span></span><span></span><span></span><span></span></div><div class="circle circle3"><span></span><span></span><span></span><span></span></div></div>'
      box.className = 'cm-quickfix-box cm-quickfix-box-pos';
      box.style.top = Math.max(pos.top - box.offsetHeight - 5 ) + "px";
      box.style.left =Math.max(pos.left) + "px";
      box.style.maxHeight = "152px";
      box.style.minWidth = "200px";
      box.style.overflow = "auto";
      return this;
    }
    function addContent(info, elm, pos, cm){
      if(typeof info == 'string') {
        box.innerHTML = info;
        setTimeout(function(){rm(box)}, 2500);
        return;
      }
      var ul = document.createElement('ul');
      var data = null;
      var litimer = null;
      box.innerHTML = '';
      box.appendChild(ul);
      data = info.result;
      for(var i=0; i< data.length; i++){
        (function(i){
          var li = document.createElement('li');
          li.innerHTML = data[i]['title'];
          CodeMirror.on(li , 'click', function(e){
            cm.focus();
            var acitons = data[i]['command']['arguments'];
            for(var j=0; j<acitons.length; j++){
              var edits = acitons[j]['edits'];
              for(var k=0; k<edits.length; k++){
                cm.doc.replaceRange(edits[k]['newText'],
                  {line: edits[k].range.start.line, ch: edits[k].range.start.character},
                  {line: edits[k].range.end.line, ch: edits[k].range.end.character} );
                clearMark(cm, elm); //清楚标记由change事件重新添加mark
                rm(box); //删除弹层
              }
            }
          });
          CodeMirror.on(li, 'mouseout', function(){
            if(litimer) clearTimeout(litimer);
            litimer = setTimeout(function(){
              rm(box); //删除弹层
            }, 1000)
          });
          CodeMirror.on(li, 'mouseover', function(){
            if(litimer) clearTimeout(litimer);
          });
          ul.appendChild(li);
        })(i)
      }
      box.style.top = Math.max(pos.top - box.offsetHeight - 5 ) + "px";
      box.style.left =Math.max(pos.left) + "px";
      return this;
    }
    function addInstall(info, elm, pos, cm) {
      var ul = document.createElement('ul');
      var litimer = null;
      box.innerHTML = '';
      box.appendChild(ul);
      var versions = info.versions;
      for (var i = 0; i < versions.length; i++) {
        (function (i) {
          var li = document.createElement('li');
          if ('latest' === versions[i]) {
            li.innerHTML = `pip install ${info['packageName']}`;
          } else if (info.stableVersion === versions[i]) {
            li.innerHTML = `<img style="margin-right: 1px" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAYAAAAfSC3RAAAAAXNSR0IArs4c6QAAAERlWElmTU0AKgAAAAgAAYdpAAQAAAABAAAAGgAAAAAAA6ABAAMAAAABAAEAAKACAAQAAAABAAAADqADAAQAAAABAAAADgAAAAC98Dn6AAABRklEQVQoFa2RP0sDQRDF5x2XixBUrhAsLNQQG/HPCVpZpLEQsbDRTrGwN34ZQfALKGIhWgtiIWk0YCdEjShYpYjGrPHGmcQ7NexVurC3zLz9vZnZI/rvZS5G1kwxu5rkC5vApTHf1Ou3BGYv1T2I4LLaec/pTGhs3l4LTNzDTL3G1DZtd35V5KvxTLPxMhESnSikAEBVcV9wM10ljF7XIhNpJu+a4sMOMc8SKCuAtQsxEAllMJ17M8G6A5w2mWhIdi4J0ipKyWeYQf3A/kfLPd03MA/gKGoj6ZSqh2mfFlWPZ2y3XNkT5yUbKMYH3nSwotVUj+fRlsXl3gZpTqrdRZDGMahByDSlp22FIU/+zMcgM4sp2iLwSA4KsrdkmCcFYu2Ldr9dlsWEz+SZj1M+7SJ301CNy/nt9+fKhrzmXMsc8kP+sj4BJItxPsOh+9cAAAAASUVORK5CYII=" /><span>pip install ${info['packageName']}=${versions[i]}</span>`;
            li.style.paddingLeft = '0px';
          } else {
            li.innerHTML = `pip install ${info['packageName']}=${versions[i]}`;
          }
          CodeMirror.on(li, 'click', function (e) {
            cm.state.quickfix.options.installPackage(info['packageName'], versions[i]);
          });
          CodeMirror.on(li, 'mouseout', function () {
            if (litimer) clearTimeout(litimer);
            litimer = setTimeout(function () {
              rm(box); //删除弹层
            }, 500)
          });
          CodeMirror.on(li, 'mouseover', function () {
            if (litimer) clearTimeout(litimer);
          });
          ul.appendChild(li);
        })(i)
      }
      box.style.top = Math.max(pos.top - box.offsetHeight - 5) + "px";
      box.style.left = Math.max(pos.left) + "px";
      return this;
    }
    return { showLoading, addContent, addInstall } ;
  }

  function rm(elt) {
    if (elt.parentNode) elt.parentNode.removeChild(elt);
  }

  function hideTooltip(tt) {
    if (!tt.parentNode) return;
    if (tt.style.opacity == null) rm(tt);
    tt.style.opacity = 0;
    setTimeout(function() { rm(tt); }, 200);
  }

  //for quickfix
  function showTooltipFor(cm, e, content, node, pos) {
    var timer = null;
    if(hasTooltip) rm(hasTooltip);
    // if(hasQuickBox) rm(hasQuickBox);
    var tooltip =   showTooltip(cm, e, content, pos);
    function hide() {
      CodeMirror.off(node, "mouseout", hide);
      if (tooltip) { hideTooltip(tooltip); tooltip = null; }
    }
/*    var poll = setInterval(function() {
      if (tooltip) for (var n = node;; n = n.parentNode) {
        if (n && n.nodeType == 11) n = n.host;
        if (n == document.body) return;
        if (!n) { hide(); break; }
      }
      if (!tooltip) return clearInterval(poll);
    }, 1500);*/
    CodeMirror.on(node, "mouseout", function(){
      if(timer) clearTimeout(timer);
      timer=setTimeout(function(){ hide()}, 500);
    }); //删除tips
    CodeMirror.on(tooltip, 'mouseover', function (e) {
      if(timer) clearTimeout(timer)
    });

  }

  function LintState(cm, conf, hasGutter) {
    this.marked = [];
    if (conf instanceof Function) conf = {getAnnotations: conf};
    if (!conf || conf === true) conf = {};
    this.options = {};
    this.linterOptions = conf.options || {};
    for (var prop in defaults) this.options[prop] = defaults[prop];
    for (var prop in conf) {
      if (defaults.hasOwnProperty(prop)) {
        if (conf[prop] != null) this.options[prop] = conf[prop];
      } else if (!conf.options) {
        this.linterOptions[prop] = conf[prop];
      }
    }
    this.timeout = null;
    this.hasGutter = hasGutter;
    this.onMouseOver = function(e) { onMouseOver(cm, e); };
    this.waitingFor = 0
  }

  var defaults = {
    highlightLines: false,
    tooltips: true,
    delay: 500,
    lintOnChange: true,
    getAnnotations: null,
    async: false,
    selfContain: null,
    formatAnnotation: null,
    onUpdateLinting: null,
    quickfix_callback: null,
    installPackage: null
  }

  //清除所有marker gutter&errorline，并重置state.marked.length
  function clearMarks(cm) {
    var state = cm.state.quickfix;
    if (state.hasGutter) cm.clearGutter(GUTTER_ID);
    if (state.options.highlightLines) clearErrorLines(cm);
    for (var i = 0; i < state.marked.length; ++i)
      state.marked[i].clear();
    state.marked.length = 0;
  }

  //清除单个marker
  function clearMark(cm, elm){
    var state = cm.state.quickfix;
    if (!/\bCodeMirror-lint-mark-/.test(elm.className)) return;
    var box = elm.getBoundingClientRect(), x = (box.left + box.right) / 2, y = (box.top + box.bottom) / 2;
    var spans = cm.findMarksAt(cm.coordsChar({left: x, top: y}, "client"));
    for (var i = 0; i < spans.length; ++i) {
      spans[i].clear();
    }

  }

  //清楚行内文本marker
  function clearErrorLines(cm) {
    cm.eachLine(function(line) {
      var has = line.wrapClass && /\bCodeMirror-lint-line-\w+\b/.exec(line.wrapClass);
      if (has) cm.removeLineClass(line, "wrap", has[0]);
    })
  }

  //为所有报错位置创建marker并注册tooptip
  function makeMarker(cm, labels, severity, multiple, tooltips, pos) {
    var marker = document.createElement("div"), inner = marker;
    marker.className = "CodeMirror-lint-marker CodeMirror-lint-marker-" + severity;
    if (multiple) {
      inner = marker.appendChild(document.createElement("div"));
      inner.className = "CodeMirror-lint-marker CodeMirror-lint-marker-multiple";
    }

    if (tooltips != false) CodeMirror.on(inner, "mouseover", function(e) {
      showTooltipFor(cm, e, labels, inner, pos);
    });

    return marker;
  }

  //获取severity error || warning
  function getMaxSeverity(a, b) {
    if (a == "error") return a;
    else return b;
  }

  function groupByLine(annotations) {
    var lines = [];
    for (var i = 0; i < annotations.length; ++i) {
      var ann = annotations[i], line = ann.from.line;
      (lines[line] || (lines[line] = [])).push(ann);
    }
    return lines;
  }

  function annotationTooltip(ann) {
    var severity = ann.severity;
    if (!severity) severity = "error";
    var tip = document.createElement("div");
    tip.className = "CodeMirror-lint-message CodeMirror-lint-message-" + severity;
    if (typeof ann.messageHTML != 'undefined') {
      tip.innerHTML = ann.messageHTML;
    } else {
      tip.appendChild(document.createTextNode(ann.message));
    }
    return tip;
  }

  //是否异步lint, 实例绑定change
  function lintAsync(cm, getAnnotations) {
    var state = cm.state.quickfix
    var id = ++state.waitingFor
    function abort() {
      id = -1
      cm.off("change", abort)
    }
    cm.on("change", abort)
    getAnnotations(cm.getValue(), function(annotations, arg2) {
      cm.off("change", abort)
      if (state.waitingFor != id) return
      if (arg2 && annotations instanceof CodeMirror) annotations = arg2
      cm.operation(function() {updateLinting(cm, annotations)})
    }, state.quickfixerOptions, cm , CodeMirror);
  }


  function startLinting(cm) {
    var state = cm.state.quickfix;
    if (!state) return;
    var options = state.options;
    /*
     * Passing rules in `options` property prevents JSHint (and other linters) from complaining
     * about unrecognized rules like `onUpdateLinting`, `delay`, `lintOnChange`, etc.
     */
    var getAnnotations = options.getAnnotations || cm.getHelper(CodeMirror.Pos(0, 0), "lint");
    if (!getAnnotations) return;
    if (options.async || getAnnotations.async) {
      lintAsync(cm, getAnnotations)
    } else {
      var annotations = getAnnotations(cm.getValue(), state.quickfixerOptions, cm);
      if (!annotations) return;
      if (annotations.then) annotations.then(function(issues) {
        cm.operation(function() {updateLinting(cm, issues)})
      });
      else cm.operation(function() {updateLinting(cm, annotations)})
    }
  }

  function updateLinting(cm, annotationsNotSorted) {
    var state = cm.state.quickfix;
    if (!state) return;
    var options = state.options;
    clearMarks(cm);

    var annotations = groupByLine(annotationsNotSorted);

    for (var line = 0; line < annotations.length; ++line) {
      var anns = annotations[line];
      if (!anns) continue;

      // filter out duplicate messages
      var message = [];
      anns = anns.filter(function(item) { return message.indexOf(item.message) > -1 ? false : message.push(item.message) });

      var maxSeverity = null;
      var tipLabel = state.hasGutter && document.createDocumentFragment();

      for (var i = 0; i < anns.length; ++i) {
        var ann = anns[i];
        var severity = ann.severity;
        if (!severity) severity = "error";
        maxSeverity = getMaxSeverity(maxSeverity, severity);

        if (options.formatAnnotation) ann = options.formatAnnotation(ann);
        if (state.hasGutter) tipLabel.appendChild(annotationTooltip(ann));

        if (ann.to) state.marked.push(cm.markText(ann.from, ann.to, {
          className: "CodeMirror-lint-mark CodeMirror-lint-mark-" + severity,
          __annotation: ann, datas : ann
        }));
      }
      // use original annotations[line] to show multiple messages
      if (state.hasGutter)
        cm.setGutterMarker(line, GUTTER_ID, makeMarker(cm, tipLabel, maxSeverity, annotations[line].length > 1,
          options.tooltips, annotations[line]));

      if (options.highlightLines)
        cm.addLineClass(line, "wrap", LINT_LINE_ID + maxSeverity);
    }
    if (options.onUpdateLinting) options.onUpdateLinting(annotationsNotSorted, annotations, cm);
  }

  function onChange(cm) {
    var state = cm.state.quickfix;
    if (!state) return;
    clearTimeout(state.timeout);
    state.timeout = setTimeout(function(){startLinting(cm);}, state.options.delay);
  }

  function popupTooltips(cm, annotations, e) {
    var target = e.target || e.srcElement;
    var tooltip = document.createDocumentFragment();
    for (var i = 0; i < annotations.length; i++) {
      var ann = annotations[i];
      tooltip.appendChild(annotationTooltip(ann));
    }
    showTooltipFor(cm, e, tooltip, target, annotations);
  }

  //textMarker注册事件
  function onMouseOver(cm, e) {
    var target = e.target || e.srcElement;
    if (!/\bCodeMirror-lint-mark-/.test(target.className)) return;
    var box = target.getBoundingClientRect(), x = (box.left + box.right) / 2, y = (box.top + box.bottom) / 2;
    var spans = cm.findMarksAt(cm.coordsChar({left: x, top: y}, "client"));


    var annotations = [];
    for (var i = 0; i < spans.length; ++i) {
      var ann = spans[i].__annotation;
      if (ann) annotations.push(ann);
    }
    if (annotations.length) popupTooltips(cm, annotations, e);
  }

  CodeMirror.defineOption("quickfix", false, function(cm, val, old) {
    if (old && old != CodeMirror.Init) {
      clearMarks(cm);
      if (cm.state.quickfix.options.lintOnChange !== false)
        cm.off("change", onChange);
      CodeMirror.off(cm.getWrapperElement(), "mouseover", cm.state.quickfix.onMouseOver);
      clearTimeout(cm.state.quickfix.timeout);
      delete cm.state.quickfix;
    }

    if (val) {
      var gutters = cm.getOption("gutters"), hasLintGutter = false;
      for (var i = 0; i < gutters.length; ++i) if (gutters[i] == GUTTER_ID) hasLintGutter = true;
      var state = cm.state.quickfix = new LintState(cm, val, hasLintGutter);
      if (state.options.lintOnChange)
        cm.on("change", onChange);
      if (state.options.tooltips != false && state.options.tooltips != "gutter")
        CodeMirror.on(cm.getWrapperElement(), "mouseover", state.onMouseOver);

      startLinting(cm);
    }
  });

  CodeMirror.defineExtension("quickfixLint", function() {
    startLinting(this);
  });
});
