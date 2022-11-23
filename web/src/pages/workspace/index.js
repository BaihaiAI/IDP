import { useEffect, useState } from "react";
import NoteBookTabContainer from "../../components/notebook/NoteBookTabContainer";
import WebTerminalTabs from "../../components/terminal/WebTerminalTabs";
import KeepAlive from 'react-activation';
import './index.less';
import globalData from "idp/global"
import { observer } from "mobx-react"
import Terminal from '@/idp/lib/terminal';
import { Tooltip } from 'antd';

import { selectActivePath } from '@/store/features/filesTabSlice';
import { useSelector } from "react-redux";
import intl from "react-intl-universal";

const iconTextArrays = {
  0: intl.get('TERMINAL_CLOSE'),
  1: intl.get('TERMINAL_OPEN'),
  2: intl.get('TERMINAL_MAXIMIZE')
}

function Workspace(props) {

    const { notebookTabRef } = globalData.appComponentData;
    const path = useSelector(selectActivePath);
    const openPathFile = Terminal.openFilePath;

    const [iconText, setIconText] = useState([iconTextArrays[1], iconTextArrays[2]]);

    useEffect(() => {
        Terminal.setTerminalVisabled(true);
        Terminal.setOpenFilePath(path);
    }, [path, openPathFile]);

    const handleResize = () => {
        Terminal.updateDocumentBodyClientWidth(document.body.clientWidth)
    };

    useEffect(() => {
        window.addEventListener('resize', handleResize);
        return () => window.removeEventListener('resize', handleResize)
    }, []);

    const next = (next) => {
        updateIconText(next);
        Terminal.setTerminalVisabled(true);
        Terminal.setNext(next);
    }

    const updateIconText = (next) => {
        if (next === 1) {
            setIconText([iconTextArrays[1], iconTextArrays[2]]);
        } else if (next === 2) {
            setIconText([iconTextArrays[0], iconTextArrays[2]]);
        } else {
            setIconText([iconTextArrays[0], iconTextArrays[1]]);
        }
    }

    const terminal = () => {
        let n = Terminal.next;
        if (Terminal.next == 1) n = 2;
        if (Terminal.next == 2) n = 1;
        if (Terminal.next == 3) n = 1;
        next(n);
    }

    const terminalTop = () => {
        next(3)
    }

    const terminalBottom = () => {
        next(2)
    }

    return (
        <div className="workspace_main" style={{ height: '100%' }}>
            <div style={Terminal.terminalVisabled ? { height: Terminal.workspaceHeight - 95, width: '100%' } : { height: '100%', width: '100%' }}>
                {
                    Terminal.next != 3 && <NoteBookTabContainer ref={notebookTabRef} />
                }
            </div>
            {
                Terminal.terminalVisabled ? <>
                    <div className="bar-group-icons" style={{ height: '36px' }}>
                        <div style={{ flex: 1, cursor: 'pointer' }} onClick={() => terminal()}>
                          <span className="terminal-title">{intl.get('TERMINAL')}</span>
                        </div>
                        <div className="workspace_icon">
                            <div style={{ marginRight: '10px', width: '15px', display: 'flex', justifyContent: 'center', alignItems: 'center', cursor: 'pointer' }}>
                                <Tooltip mouseEnterDelay={1} placement="top" title={iconText[0]}>
                                    <span onClick={() => next(1)} style={[1].includes(Terminal.next) ? { display: "none" } : {}}>
                                        <img style={{ height: '14px' }} src={require('../../assets/terminal/bp.png').default}></img>
                                    </span>
                                    <span onClick={() => next(2)} style={[2, 3].includes(Terminal.next) ? { display: 'none' } : {}}>
                                        <img style={{ height: '14px' }} src={require('../../assets/terminal/wzk.png').default}></img>
                                    </span>
                                </Tooltip>
                            </div>
                            <div style={{ width: '15px', display: 'flex', justifyContent: 'center', alignItems: 'center', cursor: 'pointer' }}>
                                <Tooltip mouseEnterDelay={1} arrowPointAtCenter={true} placement="topRight" title={iconText[1]}>
                                    {Terminal.next !== 3 && <span onClick={() => terminalTop()}> <img style={{ height: '13px' }} src={require('../../assets/terminal/qp.png').default}></img></span>}
                                    {Terminal.next === 3 && <span onClick={() => terminalBottom()}><img style={{ height: '14px' }} src={require('../../assets/terminal/wzk.png').default}></img></span>}
                                </Tooltip>
                            </div>
                        </div>
                    </div>
                    <div className="terminal" style={Terminal.terminalHeight === 0 ? { display: 'none' } : { height: Terminal.terminalHeight }}>
                        {Terminal.next != 1 && <KeepAlive><WebTerminalTabs /></KeepAlive>}
                    </div>
                </> : <> </>
            }
        </div>
    )
}
export default observer(Workspace)
