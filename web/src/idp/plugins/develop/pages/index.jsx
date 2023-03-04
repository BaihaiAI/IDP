import React, { useEffect, useMemo, useState } from 'react';
import { Tabs, message, Popover, Spin, Empty, Tooltip, Badge, notification, Progress, Modal } from "antd";
import { developApi } from '@/services';
import { CheckCircleOutlined, DownloadOutlined, DeleteOutlined } from '@ant-design/icons';
import globalData from '@/idp/global';
import VersionsModal from './version';
import cookie from 'react-cookies';
const localPlugins = require('../../../../../config/local_plugins')

import './index.less';
import { toJS } from 'mobx';

const { TabPane } = Tabs;

const AppEmpty = ({ title }) => <div style={{ width: '100%' }}><Empty description={title} image={Empty.PRESENTED_IMAGE_SIMPLE} /></div>;

const installV = ['安装其他版本', '当前版本已是最新版本'];

let oldtime = "";

function App() {

    const [installPlugins, setInstallPlugins] = useState([]);
    const [recommendedList, setRecommendedList] = useState([]);
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [versionList, setVersionList] = useState([]);
    const [currentVersion, setCurrentVersion] = useState('');
    const [record, setRecord] = useState('');
    const [updateModalOpen, setUpdateModalOpen] = useState(false);
    const [installPluginsRecord, setInstallPluginsRecord] = useState('');
    const [installNewPluginsRecord, setInstallNewPluginsRecord] = useState('');
    const [percent, setPercent] = useState(0);
    const [updateVersionTitle, setUpdateVersionTitle] = useState('');
    const [status, setStatus] = useState('');
    const [loadProgress, setLoadProgress] = useState(false);

    const [loadSpin, setLoadSpin] = useState(false);

    useEffect(() => {
        initApi();
    }, []);

    const initApi = () => {
        installedListApi();
        recommendedListApi();
    }

    const installedListApi = async () => {
        const data = await developApi.installedList();
        let timeFlg = true;
        if (data.code === 21000000) {
            let result = data?.data || [];
            Object.keys(result).forEach(it => {
                const flg = localPlugins.some(its => result[it]['name'] == its.name);
                if (flg) {
                    result[it].url = result[it].name;
                    // result[it].version = '2.0.0';
                    result[it].optionalVersion = [];
                    result[it].visible = false;
                }
            });
            if (process.env.NODE == 'pro') {
                result = data?.data.filter(it => it.visible && it.url);
            } else {
                result = data?.data.filter(it => it.url);
            }
            const flg = result.some(it => it.optionalVersion.length > 0);
            const time = new Date().getTime();
            const expiresTime = cookie.load('pluginTime');
            if (expiresTime) {
                timeFlg = (time - expiresTime) >= 1000 * 1 * 60 * 60 * 1 // 1个小时后重新提示插件更新通知
            }
            flg && timeFlg && openNotification();
            setInstallPlugins(result);
        }
    }

    const recommendedListApi = async () => {
        const data = await developApi.recommendedList();
        if (data.code === 21000000) {
            Object.keys(data.data).forEach(it => {
                const flg = localPlugins.some(its => data.data[it]['name'] == its.name);
                if (flg) {
                    data.data[it].url = data.data[it].name;
                    // data.data[it].version = '2.0.0';
                    data.data[it].optionalVersion = [];
                    data.data[it].visible = false;
                }
            });
            if (process.env.NODE == 'pro') {
                setRecommendedList(data.data.filter(it => it.visible && it.url))
            } else {
                setRecommendedList(data.data)
            }
        }
    }

    const installVersion = (item, title) => {
        const recode = installPlugins.filter(it => it.name == item.name);
        setInstallNewPluginsRecord(item);
        if (recode.length > 0) {
            setUpdateModalOpen(true);
            setInstallPluginsRecord(recode[0]);
        } else {
            setLoadSpin(true);
            install(item, title, (res, title) => {
                if (res) {
                    setLoadSpin(false);
                    message.success(`安装成功`);
                } else {
                    setLoadSpin(false);
                    message.error(`${title}失败`);
                }
            });
        }
    }

    const install = async (item, title, callback) => {
        const flg = localPlugins.some(its => item['name'] == its.name);
        if (flg) {
            item.local = true; // 置为空，则过滤不加载
            item.url = item.name;
            // item.version = '2.0.0';
            item.optionalVersion = [];
        }
        try {
            const data = await developApi.install(item);
            if (data.code === 21000000) {
                Object.assign(item, { url: data.data });
                var cur_script = document.createElement("script");
                cur_script.type = 'text/javascript';
                cur_script.src = flg ? developApi.loadLocalSystemScript(item) : developApi.loadScript(item);
                cur_script.async = true;
                cur_script.addEventListener('load', function () {
                    if (window.hasOwnProperty(item.name)) {
                        window[item.name].activate(globalData);
                        initApi();
                        callback(true);
                    }
                }, false);
                document.head.appendChild(cur_script);
            } else {
                callback(false);
            }
        } catch (error) {
            callback(false, title);
        }
    }

    const pluginInfo = (item) => {
        return (
            <div className='pluginInfo-main'>
                <div>
                    <span className='pluginInfo-publisher'>publisher: </span>
                    <span className='pluginInfo-p-info'>{item.publisher}</span>
                </div>
                <div>
                    <span className='pluginInfo-description'>description: </span>
                    <span className='pluginInfo-d-info'>{item.description}</span>
                </div>
            </div>
        )
    }

    const updatePlugins = (item) => {
        setCurrentVersion(item.version);
        setIsModalOpen(true);
        setVersionList(item.optionalVersion);
        setRecord(item);
    }

    const updateInstallVersion = async () => {
        console.log(installPluginsRecord);
        console.log(installNewPluginsRecord);
        setUpdateVersionTitle('正在安装中...');
        setPercent(20);
        // 先远端卸载老版本插件
        removePlugins(installPluginsRecord, (res) => {
            setPercent(30);
            if (res) {
                setPercent(60);
                install(installNewPluginsRecord, '', (res) => {
                    setPercent(80);
                    if (res) {
                        setPercent(100);
                        setStatus('success');
                        setUpdateVersionTitle(`更新完成`);
                        setTimeout(() => {
                            setUpdateModalOpen(false)
                        }, 2000);
                    } else {
                        setStatus('exception');
                        setUpdateVersionTitle(`安装${installPluginsRecord.version}版本失败`);
                    }
                });
            } else {
                setStatus('exception');
                setUpdateVersionTitle(`卸载${installPluginsRecord.version}旧版本失败`);
            }
        });
    }

    const removePlugins = async (item, callback) => {
        const data = await developApi.unInstalledList(item);
        if (data.code === 21000000) {
            const pl = toJS(globalData.rightSideControl.rightSideList)
                .concat(toJS(globalData.footerBarMenuControl.footerBarMenuList))
                .concat(toJS(globalData.routerMenuControl.currentRoutes))
                .concat(toJS(globalData.headerMenu.headerMeunList))
                .concat(toJS(globalData.headerTool.headerToolList));
            const unpl = pl.filter((it) => it.configJson && it.configJson.fileName == item.name);
            if (unpl.length > 0) {
                globalData.unRegister(unpl[0]['type'], unpl[0]['key'])
            }
            callback(true);
        } else {
            callback(false);
        }
    }

    const openNotification = () => {
        var millisecond = new Date().getTime();
        var expiresTime = new Date(millisecond + 60 * 1000 * 1);
        const key = `open${Date.now()}`;
        cookie.save('pluginTime', millisecond, { path: '/' });
        notification.warning({
            message: '扩展库版本通知',
            description: <>
                <span>检测到有最新插件或应用版本号，为了不影响您的正常功能使用，请及时更新到最新版本,</span>
                {/* <span onClick={goDevelop} style={{ color: '#1890ff', marginRight: '10px', cursor: 'pointer' }}>前往更新</span> */}
                <span style={{ color: '#1890ff', cursor: 'pointer' }} onClick={() => notification.destroy(key)}>暂不更新</span>
            </>,
            onClick: () => { notification.destroy(key) },
            key,
            duration: 6
        });
    };

    const preventclick = (msc = 2000) => {
        if (oldtime == '') {
            oldtime = new Date().getTime();
            return true;
        } else {
            let newtime = new Date().getTime();
            if (newtime - oldtime > msc) {
                oldtime = new Date().getTime();
                return true;
            } else {
                message.info(`请勿频繁点击安装`);
                return false;
            }
        }
    }

    const installUnVersion = (item) => {
       const flg = preventclick();
       flg && installVersion(item, '安装插件');
    }

    return (
        <div id="develop-root">
            <Tabs
                hideAdd
                type="editable-card"
            >
                <TabPane tab="扩展库" closable={false}>
                    <Spin spinning={loadSpin} size="small">
                        <div className='develop-main'>
                            <div>
                                <div className='develop-title'>已安装的插件或者应用</div>
                                <div className='develop-text'>点击卡片，<span className='develop-action'>卸载</span>或者<span className='develop-action'>更新</span>已安装的插件或者应用。</div>
                                <div style={{ display: 'flex', width: '100%' }}>
                                    <div className='develop-map'>
                                        {
                                            installPlugins.length > 0 ? installPlugins.map((item, index) => {
                                                return <Popover
                                                    key={`${item.name}-${index}`}
                                                    placement="rightTop"
                                                    content={() => pluginInfo(item)}
                                                    title={<span>{item.title}@{item.version}</span>}
                                                    trigger="hover"
                                                >
                                                    <div key={`${item.name}-${index}`} className='develop-card'>
                                                        <div className='develop-recommended'>
                                                            <div className='develop-icon-mobx'>
                                                                <div className='develop-r-icon'>
                                                                    <img src={item.icon ? developApi.laodImgs(item) : require('../assets/plugins_default.png').default}></img>
                                                                </div>
                                                            </div>
                                                            <div className='develop-r-group'>
                                                                <div className='develop-r-name'>{item.title}</div>
                                                                <div className='develop-r-version'>
                                                                    <span>v{item.version}</span>
                                                                </div>
                                                                <div className='develop-r-description'>{item.description}</div>
                                                                <div className='develop-r-publisher'>
                                                                    <CheckCircleOutlined style={{ fontSize: '13px', position: 'relative', top: '1px', marginRight: '3px', color: '#3793ef' }} />
                                                                    {item.publisher}
                                                                </div>
                                                            </div>
                                                            <div className='develop-icons'>
                                                                <Tooltip title={'更新版本'}>
                                                                    {
                                                                        item.optionalVersion.length === 0 ?
                                                                            <DownloadOutlined onClick={() => updatePlugins(item)} className='develop-ic develop-downloadOutlined' /> :
                                                                            <Badge color={'#1890ff'} dot>
                                                                                <DownloadOutlined style={{ position: 'absolute', right: '0px' }} onClick={() => updatePlugins(item)} className='develop-ic develop-downloadOutlined' />
                                                                            </Badge>
                                                                    }
                                                                </Tooltip>
                                                                <Tooltip title={`卸载${item.version}版本`}>
                                                                    <DeleteOutlined onClick={() => removePlugins(item, (res) => {
                                                                        if (res) {
                                                                            setLoadSpin(true);
                                                                            const time = setTimeout(() => {
                                                                                initApi();
                                                                                setLoadSpin(false);
                                                                                clearTimeout(time);
                                                                                message.success('卸载成功');
                                                                            }, 1000)
                                                                        } else {
                                                                            setLoadSpin(false);
                                                                            message.error('卸载失败')
                                                                        }
                                                                    })} className='develop-ic develop-deleteOutlined' />
                                                                </Tooltip>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </Popover>
                                            }) : <AppEmpty title='未安装插件' />
                                        }
                                    </div>
                                </div>
                            </div>
                            <div style={{ marginTop: '30px', marginBottom: '100px' }}>
                                <div className='develop-title'>全部插件或者应用</div>
                                <div className='develop-text'>点击卡片，<span className='develop-action'>下载</span>插件或者应用。</div>
                                <div style={{ display: 'flex', width: '100%' }}>

                                    <div className='develop-map'>
                                        {
                                            recommendedList.length > 0 ? recommendedList.map((item, index) => {
                                                return <Popover
                                                    key={index}
                                                    placement="rightTop"
                                                    content={() => pluginInfo(item)}
                                                    title={<span>{item.title}@{item.version}</span>}
                                                    trigger="hover"
                                                >
                                                    <div className='develop-card'>
                                                        <div className='develop-recommended'>
                                                            <div className='develop-icon-mobx'>
                                                                <div className='develop-r-icon'>
                                                                    <img src={item.icon ? developApi.laodImgs(item) : require('../assets/plugins_default.png').default}></img>
                                                                </div>
                                                            </div>
                                                            <div className='develop-r-group'>
                                                                <div className='develop-r-name'>{item.title}</div>
                                                                <div className='develop-r-version'>
                                                                    <span>v{item.version}</span>
                                                                </div>
                                                                <div className='develop-r-description'>{item.description}</div>
                                                                <div className='develop-r-publisher'>{item.publisher}</div>
                                                            </div>
                                                            <div className='develop-action' onClick={() => installUnVersion(item)}>
                                                                <span className='develop-install' style={{ fontSize: '13px', padding: '0px 8px' }}>安装</span>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </Popover>

                                            }) : <AppEmpty title='暂无插件' />
                                        }
                                    </div>

                                </div>
                            </div>
                        </div>
                    </Spin>
                </TabPane>
            </Tabs>
            <VersionsModal
                currentVersion={currentVersion}
                visible={isModalOpen}
                showModal={setIsModalOpen}
                versionList={versionList}
                record={record}
                initApi={initApi}
            />
            <UpdateInstallVersion
                updateModalOpen={updateModalOpen}
                setUpdateModalOpen={setUpdateModalOpen}
                installPluginsRecord={installPluginsRecord}
                installNewPluginsRecord={installNewPluginsRecord}
                updateInstallVersion={updateInstallVersion}
                percent={percent}
                status={status}
                updateVersionTitle={updateVersionTitle}
                setPercent={setPercent}
                setLoadProgress={setLoadProgress}
                loadProgress={loadProgress}
            />
        </div>
    )
};

function UpdateInstallVersion({ loadProgress, updateModalOpen = false, setUpdateModalOpen, installPluginsRecord,
    installNewPluginsRecord, percent = 0, updateInstallVersion, status, updateVersionTitle, setPercent, setLoadProgress }) {

    const handleOk = () => {
        // setUpdateModalOpen(false);
        setLoadProgress(true);
        setPercent(10);
        updateInstallVersion();
    };
    const handleCancel = () => {
        setUpdateModalOpen(false);
    };

    const laodInstallProgress = useMemo(() => {
        return (
            loadProgress ? (
                <div style={{ display: 'flex' }}>
                    <div style={{ width: '65px' }}>更新进度:</div>
                    <Progress
                        percent={percent}
                        showInfo={false}
                        width={320}
                        status={status}
                        style={{ width: '285px' }}
                        format={(number) => `进行中，已完成${number}%`}
                    />
                    <div style={{ width: '140px' }}>{updateVersionTitle}</div>
                </div>
            ) : (
                <div>
                    <span>检测到您已经安装了</span>
                    <span style={{ fontWeight: '500', color: '#1890ff' }}>{installPluginsRecord.title}@{installPluginsRecord.version}</span>
                    版本，是否手动更新到<span style={{ fontWeight: '500', color: '#1890ff' }}>{installNewPluginsRecord.version}</span>版本号吗？
                </div>
            )
        )
    }, [loadProgress, status, percent, installPluginsRecord, installNewPluginsRecord, updateVersionTitle]);

    return (
        <div>
            <Modal
                title="更新插件或应用"
                visible={updateModalOpen}
                onOk={handleOk}
                onCancel={handleCancel}
                okText={'更新'}
            >
                <div style={{ display: 'flex' }}>
                    {laodInstallProgress}
                </div>
            </Modal>
        </div>
    )
}

export default App;