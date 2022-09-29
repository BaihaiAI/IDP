import { useEffect, useMemo, useState } from "react";
import NoteBookTabContainer from "../../components/notebook/NoteBookTabContainer";
import Icons from "../../components/Icons/Icons";
import WebTerminalTabs from "../../components/terminal/WebTerminalTabs";
import KeepAlive from 'react-activation';
import { CaretDownOutlined, CaretRightOutlined, VerticalAlignTopOutlined, VerticalAlignBottomOutlined } from "@ant-design/icons"
import './index.less';
import globalData from "idp/global"
import { observer } from "mobx-react"
import { withErrorBoundary } from "react-error-boundary"
import ErrorView from "@components/errorView"
import Terminal from '@/idp/lib/terminal';

import { selectActivePath } from '@/store/features/filesTabSlice';
import { useSelector } from "react-redux";

function Workspace(props) {

    const { notebookTabRef } = globalData.appComponentData;
    const path = useSelector(selectActivePath);
    const openPathFile = Terminal.openFilePath;

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
        Terminal.setTerminalVisabled(true);
        Terminal.setNext(next);
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
                        <div className="left-bottom-icon">
                            <span onClick={() => next(1)} style={[1].includes(Terminal.next) ? { display: "none" } : {}}><CaretDownOutlined style={{ fontSize: '12px' }} /></span>
                            <span onClick={() => next(2)} style={[2, 3].includes(Terminal.next) ? { display: 'none' } : {}}><CaretRightOutlined style={{ fontSize: '12px' }} /></span>
                            <span className="terminal-title" onClick={() => terminal()}>终端</span>
                        </div>
                        <div className="workspace_icon">
                            {Terminal.next !== 3 && <span onClick={() => terminalTop()}><VerticalAlignTopOutlined style={{ fontSize: '12px' }} /></span>}
                            {Terminal.next === 3 && <span onClick={() => terminalBottom()}><VerticalAlignBottomOutlined style={{ fontSize: '12px' }} /></span>}
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
