import React, { Component, useCallback, useEffect, useMemo, useState } from 'react';
import { Button, Modal, Select, Steps, Tag } from 'antd';
import { LoadingOutlined, SmileOutlined, SolutionOutlined, UserOutlined } from '@ant-design/icons';
import globalData from '@/idp/global';
import { developApi } from '@/services';
import { toJS } from 'mobx';

const { Step } = Steps;

import './version.less';
import { observer } from 'mobx-react';
import { message } from 'antd';

const Option = Select.Option;

const loadMsgTitle = {
    1: '',
    2: '正在安装，请稍后...',
    3: '安装成功',
    4: '安装失败，请重试安装'
}

function Version({ visible = false, record, currentVersion, showModal, versionList, initApi }) {

    const [installV, setInstallV] = useState(currentVersion);
    const [stepVisible, setStepVisible] = useState(false);
    const [stepsNext, setStepsNext] = useState({ next: -1, icon: '', descriptionIcon: '' });
    const [status, setStatus] = useState('');
    const [loadMsg, setLoadMsg] = useState(loadMsgTitle[1]);

    useEffect(() => {
        setInstallV(currentVersion);
    }, [visible])

    const initVersionData = () => {
        setInstallV('');
        setStepsNext({ next: -1, icon: '', descriptionIcon: '' });
        setStepVisible(false);
        setStatus('');
        setLoadMsg('');
    }

    const handleOk = () => {
        initVersionData();
        showModal(false);
        setStepVisible(false);
    };

    const handleCancel = () => {
        initVersionData();
        showModal(false);
        setStepVisible(false);
    };

    const handleVersion = (version) => {
        setLoadMsg('');
        initVersionData();
        setInstallV(version);
    }

    const removePlugins = async (item, callback) => {
        setStepsNext({ next: 0, icon: <LoadingOutlined />, descriptionIcon: '' });
        try {
            const data = await developApi.unInstalledList(item);
            if (data.code === 21000000) {
                const pl = toJS(globalData.rightSideControl.rightSideList)
                    .concat(toJS(globalData.footerBarMenuControl.footerBarMenuList))
                    .concat(toJS(globalData.routerMenuControl.currentRoutes))
                    .concat(toJS(globalData.headerMenu.headerMeunList))
                    .concat(toJS(globalData.headerTool.headerToolList));
                const unpl = pl.filter((it) => it.configJson && it.configJson.fileName == item.name);
                if (unpl.length > 0) {
                    globalData.unRegister(unpl[0]['type'], unpl[0]['key']);
                };
                setStatus('finish');
                callback(true)
            } else {
                setStatus('error');
                callback(false);
            }
        } catch (error) {
            setStatus('error');
            callback(false);
        }
    }

    const useSteps = useMemo(() => {
        return (
            <Steps status={status} size='small' current={stepsNext.next}>
                <Step icon={stepsNext.next == 0 ? stepsNext.icon : ''} title={`${stepsNext.next == 0 ? '正在' : ''}卸载${currentVersion}版本...`} description={<></>} />
                <Step icon={stepsNext.next == 1 ? stepsNext.icon : ''} title={`${stepsNext.next == 1 ? '正在' : ''}安装${installV ? installV : currentVersion}版本...`} description={<></>} />
                <Step icon={stepsNext.next == 2 ? stepsNext.icon : ''} title={`安装完成`} description={<></>} />
            </Steps>
        )
    }, [stepVisible, stepsNext]);

    const install = () => {
        setStepVisible(true);
        setLoadMsg(loadMsgTitle[2]);
        removePlugins(record, async (result) => {
            if (result) {
                setStepsNext({ next: 0, icon: '' });
                installNewVersion(record, (res) => {
                    if (res) {
                        setStepsNext({ next: 3, icon: '' });
                        initApi();
                    } else {
                        setLoadMsg(loadMsgTitle[4]);
                        setStatus('error');
                        setStepsNext({ next: 1, icon: '' });
                    }
                });
            } else {
                setStatus('error');
                setLoadMsg(loadMsgTitle[4]);
                setStepsNext({ next: 0, icon: '' });
            }
            setStepVisible(false);
        })
    }

    const installNewVersion = async (item, callback) => {
        setStepsNext({ next: 1, icon: <LoadingOutlined></LoadingOutlined>, descriptionIcon: '' });
        Object.assign(item, { version: installV ? installV : currentVersion });
        try {
            const data = await developApi.install(item);
            if (data.code === 21000000) {
                Object.assign(item, { url: data.data });
                var cur_script = document.createElement("script");
                cur_script.type = 'text/javascript';
                cur_script.src = developApi.loadScript(item);
                cur_script.async = true;
                cur_script.addEventListener('load', function () {
                    if (window.hasOwnProperty(item.name)) {
                        window[item.name].activate(globalData);
                        setLoadMsg(loadMsgTitle[3]);
                        callback(true);
                    }
                }, false);
                document.head.appendChild(cur_script);
            } else {
                setLoadMsg(loadMsgTitle[4]);
                callback(false);
            }
        } catch (error) {
            setLoadMsg(loadMsgTitle[4]);
            callback(false);
        }
    }

    return (
        <div className='version-main'>
            <Modal
                visible={visible}
                onOk={handleOk}
                onCancel={handleCancel}
                footer={[
                    <Button disabled={stepVisible} onClick={install} type="primary">安装</Button>
                ]}
                width={800}
            >
                <div style={{ padding: '20px 10px' }}>
                    <span>请选择安装版本：</span>
                    <span>
                        <Select disabled={stepVisible} onChange={handleVersion} value={installV} style={{ width: 300 }}>
                            <Option key={currentVersion}>v{currentVersion} (当前安装版本)</Option>
                            {
                                versionList?.map((it, index) => {
                                    return <Option key={it}>v{it}</Option>
                                })
                            }
                        </Select>
                    </span>
                    <span className='version-msg' style={loadMsg ? { opacity: 1 } : { opacity: 0 }}>
                        <Tag color="cyan">{loadMsg}</Tag>
                    </span>
                    <div className='version-steps' style={stepVisible ? { opacity: 1 } : { opacity: 1 }}>
                        {useSteps}
                    </div>
                </div>
            </Modal>
        </div>
    )
}

export default observer(Version)