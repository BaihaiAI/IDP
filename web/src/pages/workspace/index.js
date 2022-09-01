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
    const workspaceWidth = Terminal.getWorkspaceWidth();
    console.log(workspaceWidth);

    useEffect(() => {
        Terminal.setTerminalVisabled(true);
        Terminal.setOpenFilePath(path);
    }, [path]);

    const handleResize = () => {
        Terminal.updateDocumentBodyClientWidth(document.body.clientWidth)
    };

    const isOpenRightBar = (openFile) => {
        let flg = false;
        const file = openFile.slice(openFile.lastIndexOf(".") + 1);
        if (['ipynb', 'idpnb'].includes(file)) {
            Terminal.setRightSideWidth(48, false);
            flg = true;
        } else if (['py'].includes(file)) {
            Terminal.setRightSideWidth(0, false);
            flg = true;
        }
        return flg;
    }

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
        <div className="workspace_main">
            <div className="workspace">
                <div style={Terminal.terminalVisabled ? { height: Terminal.workspaceHeight, width: Terminal.workspaceWidth } : { height: '100%' }}>
                    {
                        Terminal.next != 3 && <NoteBookTabContainer ref={notebookTabRef} />
                    }
                </div>
            </div>
            {
                Terminal.terminalVisabled ? <>
                    <div className="bar-group-icons" style={{ height: '46px' }}>
                        <div className="left-bottom-icon">
                            <span onClick={() => next(1)} style={[1].includes(Terminal.next) ? { display: "none" } : {}}><CaretDownOutlined style={{ fontSize: '16px' }} /></span>
                            <span onClick={() => next(2)} style={[2, 3].includes(Terminal.next) ? { display: 'none' } : {}}><CaretRightOutlined style={{ fontSize: '16px' }} /></span>
                            <span className="terminal-title" onClick={() => terminal()}>终端</span>
                        </div>
                        <div className="workspace_icon">
                            {Terminal.next !== 3 && <span onClick={() => terminalTop()}><VerticalAlignTopOutlined style={{ fontSize: '16px' }} /></span>}
                            {Terminal.next === 3 && <span onClick={() => terminalBottom()}><VerticalAlignBottomOutlined style={{ fontSize: '16px' }} /></span>}
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
export default withErrorBoundary(observer(Workspace), { FallbackComponent: ErrorView })
