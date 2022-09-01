import React from 'react';
import rescriptsrc from '../../../../../config/rescriptsrc';

function addScript(parentId, scripts) {
  let result = [];
    let start = false;
      let textCach = '';
      let script = undefined;
      let dom = document.getElementById(parentId);
      let scriptDom = [];
      for (let i = 0; i < dom.childNodes.length; i++) {
        if ("SCRIPT" === dom.childNodes[i].nodeName) {
          scriptDom.push(dom.childNodes[i]);
        }
      }
      for (let i = 0; i < scriptDom.length; i++) {
        dom.removeChild(scriptDom[i]);
      }
  
      for (let text of scripts) {
        // 替换lets-plot.min.js
        if (text.indexOf('https://cdn.jsdelivr.net/gh/JetBrains/lets-plot@v2.2.1/js-package/distr/lets-plot.min.js') > -1) {
          const path = process.env.NODE == 'dev' ? `//${window.location.hostname}:${rescriptsrc.devServer().port}/` : '/child/idpStudio-idp/';
          text = text.replace('https://cdn.jsdelivr.net/gh/JetBrains/lets-plot@v2.2.1/js-package/distr/lets-plot.min.js', `${path}static/lib/lets-plot.min.js`);
        }
        if (text.trim().startsWith('<script')) {
          script = document.createElement('script');
          const t = text.trim();
          const arr = t.slice(8, t.indexOf('>')).split(' ');
          for (const prop of arr) {
            if (prop.indexOf('=') > 0) {
              const p = prop.split('=');
              script.setAttribute(p[0], JSON.parse(p[1]))
            }
          }
  
          if (t.indexOf('</script>') > 0) {
            script.text = t.slice(t.indexOf('>') + 1, t.indexOf('</script>'));
            result.push(script);
            start = false;
            textCach = '';
            script = undefined;
          } else {
            start = true;
            if (t.indexOf('>') > 0) {
              textCach = t.slice(t.indexOf('>') + 1);
            }
          }
        } else if (text.trim().startsWith('</script')) {
          script.text = textCach;
          result.push(script);
  
          start = false;
          textCach = '';
          script = undefined;
        } else {
          if (start) {
            textCach += text;
          }
        }
    }
  return result;
}

class TextPlain extends React.Component {
    constructor(props) {
      super(props);
      this.scriptFlag = false;
      this.state = {
        cellId: props.cellId,
        scripts: props.scripts,
        textPlain: props.textPlain,
      };
    }

  runScripts = (dom, scripts) => {
    if (scripts && scripts.length !== 0) {
      let script = scripts[0];
      let nextScripts = scripts.slice(1);
        if (script.src) {
          script.onload = () => {
            this.runScripts(dom, nextScripts);
          }
          dom.appendChild(script);
        } else {
          dom.appendChild(script);
          this.runScripts(dom, nextScripts);
        }
      }
    }

  runScript = () => {
    let dom = document.getElementById(`text-${this.state.cellId}`);
    let scripts = addScript(`text-${this.state.cellId}`, this.state.scripts);
    this.runScripts(dom, scripts);
  }

    componentDidMount() {
      this.runScript();
    }

    componentDidUpdate() {
      if (this.scriptFlag) return false;
      this.runScript();
    }

    componentWillReceiveProps(nextProps) {
      this.scriptFlag = this.state.textPlain === nextProps.textPlain;
      this.setState({
        cellId: nextProps.cellId,
        scripts: nextProps.scripts,
        textPlain: nextProps.textPlain,
      });
      // let runScript = addScript(`text-${nextProps.cellId}`, nextProps.scripts);
      // for (let n = runScript.next(); !n.done; n = runScript.next()) {
      //   console.log(n.value);
      // }
    }

    render() {
        return (
            <div>
                <div dangerouslySetInnerHTML={{__html: `<div id="text-${this.state.cellId}">${this.state.textPlain}<div>`}}></div>
            </div>
        );
    }
}

export default TextPlain;