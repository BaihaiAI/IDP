import React, { useState, useEffect } from 'react';
import { observer } from 'mobx-react';
import { useHotkeys } from "react-hotkeys-hook";
import sneltoets from '@idp/global/sneltoets';

import './shortcutlist.less'

function ShortcutList(props) {
    const [active, setActive] = useState(0)
    const [crtlOrcmd, setCrtlOrCmd] = useState('')
    const [altOropt, SetaltOropt] = useState('')
    useEffect(() => {
        const isMac = /macintosh|mac os x/i.test(navigator.userAgent);
        if (isMac) {
            setCrtlOrCmd('⌘')
            SetaltOropt('⌥')
        } else {
            setCrtlOrCmd('Ctrl')
            SetaltOropt('Alt')
        }
    }, [])
    const state = {
        hotKey: [
            {
                title: '编辑模式',
                explain: [
                    "注释整行/撤销注释，仅代码状态有效",
                    "切换到命令模式",
                    "缩进 向右缩进",
                    "解除缩进 向左缩进",
                    "全选",
                    "撤销",
                    "跳到单元开头",
                    "跳到单元末尾",
                    "跳到左边一个字首",
                    "跳到右边一个字首",
                    "删除本行",
                    "运行本单元，选中下一单元",
                    "运行本单元",
                    "保存当前 NoteBook",
                ],
                key: [
                    `${crtlOrcmd}-/`,
                    "Esc",
                    `${crtlOrcmd}-]`,
                    `${crtlOrcmd}-[`,
                    `${crtlOrcmd}-A`,
                    `${crtlOrcmd}-Z`,
                    `${crtlOrcmd}-Up`,
                    `${crtlOrcmd}-Down`,
                    `${crtlOrcmd}-Left`,
                    `${crtlOrcmd}-Right`,
                    `${crtlOrcmd}-Backspace`,
                    `Shift-Enter`,
                    `${crtlOrcmd}-Enter`,
                    `${crtlOrcmd}-S`,
                ]
            },
            {
                title: '命令模式',
                explain: [
                    "保存当前 NoteBook",
                    "运行本单元，选中下一单元",
                    "运行本单元",
                    "转入编辑模式",
                    "选中上方单元",
                    "选中下方单元",
                    "在下方插入新单元",
                    "在上方插入新单元",
                    "在下方插入新单元",
                    "删除选中的单元",
                    "中断 NoteBook 内核",
                    "重启 NoteBook 内核",
                    "向上滚动",
                    "向下滚动",
                    "Cell撤回",
                    "Cell反向撤回",
                    "全局搜索"
                ],
                key: [
                    `${crtlOrcmd}-S`,
                    `Shift-Enter`,
                    `${crtlOrcmd}-Enter`,
                    `Enter`,
                    `Up`,
                    `Down`,
                    `B`,
                    `A`,
                    `${crtlOrcmd}-${altOropt}-Enter`,
                    `D,D`,
                    `I,I`,
                    `0,0`,
                    `Shift-Space`,
                    `Space`,
                    `Z`,
                    `Shift-Z`,
                    `${crtlOrcmd}-p`
                ]
            }
        ]
    }
    const closeModle = (e) => {
        e.stopPropagation()
        sneltoets.updateSneltoetsListVisible(false);
    }
    useHotkeys('esc', closeModle)

    return (
        <React.Fragment>
            {sneltoets.sneltoetsListVisible ? (
                <div className='shortcut-warrp' onClick={closeModle}>
                    <div className='shortcut' onClick={(e) => e.stopPropagation()}>
                        <div className='title'>
                            {state.hotKey.map((prop, index) => (
                                <p
                                    key={prop.title}
                                    onClick={() => setActive(index)}
                                    className={active === index ? "title-active" : ""}
                                >{prop['title']}</p>
                            ))}
                        </div>
                        {state.hotKey.map((prop, index) => (
                            active === index ? (
                                <div className='content' key={prop.title}>
                                    <div className='content-title'>
                                        （任意单元格处于编辑状态时为编辑模式  /  没有单元格处于编辑状态时为命令模式）
                                    </div>
                                    <div className='explain'>
                                        {prop.explain.map((item, n) => (
                                            <p key={n}>{item}</p>
                                        ))}
                                    </div>
                                    <div className='key'>
                                        {prop.key.map((item, n) => (
                                            <p key={n}>{item}</p>
                                        ))}
                                    </div>
                                </div>
                            ) : null
                        ))}
                    </div>
                </div>
            ) : null}
        </React.Fragment>
    )
}

export default observer(ShortcutList)

