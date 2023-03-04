import { useCallback, useEffect, useState } from 'react';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import { AttachAddon } from 'xterm-addon-attach';
import intl from "react-intl-universal";
import './terminal.less';
import { setCurrentEnv, getCurrentEnv, terminalWsUrl } from '@/store/config';
import { message } from 'antd';
import terminalApi from '@/services/terminalApi';
import { observer } from 'mobx-react';
import IdpTerminal from '@/idp/lib/terminal';
import environmentAPI from '@/services/environment';

// const socketURL = `ws://127.0.0.1:8089/terminal/v1/socket/`;
function WebTerminal({ terminalId }) {

    const [size, setSize] = useState({ height: 0, width: 0 });

    const [fitAddon, setFitAddon] = useState(null);
    const [ws, setWs] = useState(null);
    let wsInit = false; // 防止同时建立多个websocket连接

    // 计算终端高度和宽度
    const computeSize = useCallback(() => {
        let height = IdpTerminal.terminalClientHeight;
        let width = IdpTerminal.terminalWidth;
        setSize({ height, width });
        const rows = Math.floor(height / 16);
        const cols = Math.floor(width / 8) - 2;
        return { rows, cols };
    }, [IdpTerminal.sideWidth, IdpTerminal.terminalClientHeight, IdpTerminal.terminalWidth, IdpTerminal.terminalHeight, IdpTerminal.next, IdpTerminal.leftFileManageWidth]);

    //初始化当前系统环境，返回终端的 pid，标识当前终端的唯一性
    const [pid, setPid] = useState(0);
    const initSysEnv = async (currentEnv) => {
        const { rows, cols } = computeSize();
        return await terminalApi.getTerminal({ rows, cols, env: currentEnv }).then(res => {
            if (res.code === 20000000) {
                return res.data.pid
            } else {
                message.error(res.message);
                return null
            }
        }).catch(() => {
            message.error(intl.get('TERMINAL_ERROR_1'));
            return null
        });
    }

    useEffect(() => {
        const { rows, cols } = computeSize();
        let term = new Terminal({
            fontFamily: 'Menlo, Monaco, "Courier New", monospace',
            fontWeight: 400,
            fontSize: 14,
            scrollback: 2000,
            rows: rows,
            cols: cols,
        });
        term.open(document.getElementById(terminalId));
        term.focus();
        let ws = null;
        async function asyncInitSysEnv() {
          wsInit = true;
            let currentEnv = getCurrentEnv();
            await environmentAPI.getEnvironmentName()
              .then(res => {
                const data = res.data
                setCurrentEnv(data)
                currentEnv = data
              })
              .catch(err => {
                console.log(err)
              })
            const pid = await initSysEnv(currentEnv);
            if (!pid) {
              wsInit = false;
              return;
            }
            setPid(pid);
          
            ws = new WebSocket(terminalWsUrl + pid);
            setWs(ws);
            ws.onopen = () => {
              wsInit = false;
                if (ws.readyState === 1 && currentEnv) {
                    // ws.send(`source /root/.bashrc\n`)
                    // ws.send(`source /root/.bash_profile\n`)
                    // ws.send(`source activate ${currentEnv} \n`);
                    // ws.send(`clear\n`)
                }
            }
            ws.onclose = (e) => {
              console.log(e);
              wsInit = false;
            }
            const attachAddon = new AttachAddon(ws);
            term.loadAddon(attachAddon);
        }
        asyncInitSysEnv();
        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        setFitAddon(fitAddon);
        term.onData(() => {
            // if (data.startsWith('stty rows')) return;
            if ((!ws || ws.readyState !== 1) && !wsInit) {
                console.log('ws is not ready');
                ws && ws.close();
                asyncInitSysEnv();
            }
        })
        return () => {
            //组件卸载，清除 Terminal 实例
            term.dispose();
            ws && ws.close();
        };
    }, []);

    // resize terminal
    const resize = (rows, cols) => {
        ws.send(`stty rows ${rows} columns ${cols}\r`);
        // ws.send('clear\r');
    }

    useEffect(() => {
        if (pid === 0 || IdpTerminal.terminalHeight === 0 || IdpTerminal.terminalWidth === 0 || IdpTerminal.sideWidth === 0) return
        const { rows, cols } = computeSize();
        resize(rows, cols);
    }, [IdpTerminal.sideWidth, IdpTerminal.terminalWidth, IdpTerminal.terminalClientHeight, IdpTerminal.terminalHeight, IdpTerminal.next, IdpTerminal.leftFileManageWidth]);

    useEffect(() => {
      console.log({size});
        if (fitAddon) {
            fitAddon.fit();
        }
    }, [size])

    console.log('@terminalClientHeight:', IdpTerminal.terminalClientHeight);
    console.log('@workspaceHeight:', IdpTerminal.workspaceHeight)

    return <div style={{ backgroundColor: 'black', width: IdpTerminal.terminalWidth, height: IdpTerminal.terminalHeight }} >
        <div id={terminalId} style={{ width: IdpTerminal.terminalWidth, height: IdpTerminal.terminalClientHeight }}></div>
    </div>;
}

export default observer(WebTerminal);
