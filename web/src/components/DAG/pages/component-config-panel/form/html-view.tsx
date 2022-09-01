import React, { useEffect } from 'react';

export interface Props {
  id?: string
  html: string
}

export const HtmlView: React.FC<Props> = ({id, html}) => {
  const domId = id ? id : 'html-content';

  const addScript = (parentId: string, scripts: string[]) => {
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

    for (const text of scripts) {
      if (text.trim().startsWith('<script')) {
        script = document.createElement('script');
        const t = text.trim();
        const arr = t.slice(8, t.indexOf('>')).split(' ');
        for (const prop of arr) {
          if (prop.indexOf('=') > 0) {
            script.setAttribute(prop.slice(0, prop.indexOf('=')), JSON.parse(prop.slice(prop.indexOf('=') + 1)))
          }
        }

        if (t.indexOf('</script>') > 0) {
          result.push(script);
          start = false;
          textCach = '';
          script = undefined;
        } else {
          start = true;
        }
      } else if (text.trim().startsWith('</script')) {
        script.text = textCach;
        result.push(script);

        start = false;
        textCach = '';
        script = undefined;
      } else {
        if (start) {
          textCach = `${textCach}\n${text}`;
        }
      }
    }
    return result;
  }

  const runScripts = (dom: HTMLElement, scripts: any) => {
    if (scripts && scripts.length !== 0) {
      let script = scripts[0];
      let nextScripts = scripts.slice(1);
      if (script.src) {
        script.onload = () => {
          runScripts(dom, nextScripts);
        }
        dom.appendChild(script);
      } else {
        dom.appendChild(script);
        runScripts(dom, nextScripts);
      }
    }
  }

  const runScript = (domId: string, scriptArr: string[]) => {
    let dom = document.getElementById(domId);
    let scripts = addScript(domId, scriptArr);
    runScripts(dom, scripts);
  }

  useEffect(() => {
    const arr: string[] = html.split('\n')
    let scripts: string[] = [];
    let scriptFlag = false;
    for (const text of arr) {
      if (text.indexOf('<script') >= 0) {
        if (text.indexOf('</script>') < 0) {
          scriptFlag = true;
        }
        scripts.push(text);
        continue;
      } else if (text.indexOf('</script>') >= 0) {
        scripts.push(text);
        scriptFlag = false;
        continue;
      }
      if (scriptFlag) {
        scripts.push(text);
      }
    }
    if (scripts.length  !== 0) {
      runScript(domId, scripts)
    }
  }, [html])

  return (
    <div dangerouslySetInnerHTML={{ __html: `<div id="${domId}">${html}<div>` }}></div>
  )
}