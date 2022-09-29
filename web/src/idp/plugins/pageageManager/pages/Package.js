import React from 'react';
import axios from "axios"
import packageApi from 'idpServices/packageApi';
import { Input, Collapse, Card, List, message, Button, Tooltip } from 'antd';
import { LoadingOutlined } from '@ant-design/icons';
import intl from 'react-intl-universal';
import "./Package.less";
import { noteApiPath2 } from '@/services/httpClient'
import { projectId, teamId } from '@/store/cookie'

const { Panel } = Collapse;
const { Search } = Input;

class Package extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            installedList: [],
            searchList: [],
            usefulList: [],
            searchDisable: false,
            activeKey: ['1'],
            installedListLoading: true,
        };
        this.searchListRef = React.createRef();
    }

    componentDidMount() {
        const _this = this;    //先存一下this，以防使用箭头函数this会指向我们不希望它所指向的对象。
        packageApi.list()
            .then(function (response) {
                _this.setState({
                    installedList: response.data.sort(_this.sortList),
                    installedListLoading: false,
                });
            })
            .catch(function (error) {
                console.log(error);
                _this.setState({ installedListLoading: false })
                // _this.setState({
                //     installedList: [{
                //         packageName: intl.get('PACKAGE_ERROR_LIST'),
                //         version: ''
                //     }],
                // });
                message.error(intl.get('PACKAGE_ERROR_LIST'));
            })
    }

    sortList = (a, b) => {
        const nameA = a.packageName.toLowerCase();
        const nameB = b.packageName.toLowerCase();
        if (nameA > nameB) {
            return 1;
        } else if (nameA === nameB) {
            if (a.version.toLowerCase() > b.version.toLowerCase()) {
                return 1;
            } else {
                return -1
            }
        } else {
            return -1;
        }
    }

    search = (value) => {
        const _this = this;    //先存一下this，以防使用箭头函数this会指向我们不希望它所指向的对象。
        const keyword = value.trim();
        if ("" === keyword) {
            _this.setState({
                searchList: [],
            });
            return;
        }

        this.setState({ searchDisable: true });

        packageApi.searchV3({ packageName: keyword })
            .then(function (response) {
                const records = response.data.records
                let list = [];
                for (let item of records) {
                    item.istate = item.installed ? intl.get('PACKAGE_INSTALLED') : intl.get('PACKAGE_INSTALL');
                    list.push(item);
                }
                _this.setState({
                    searchList: list,
                    searchDisable: false,
                    searchboxState: true,
                    activeKey: []
                });
                _this.searchListRef.current.scrollTop = 0;//回到容器顶部
            })
            .catch(function (error) {
                console.log(error);
                message.error(intl.get('PACKAGE_ERROR_SERACH'));
                _this.setState({ searchDisable: false });
            })
    }

    setSearchList(_this, packageName, version, istate, error = '') {
        let searchList = _this.state.searchList.slice();
        for (let item of searchList) {
            if (packageName == item.packageName && version == item.stableVersion) {
                item.istate = istate;
                item.installed = intl.get('PACKAGE_INSTALLED') == istate;
                if (item.istate === '安装失败') {
                    item.error = error
                }
            }
        }
        _this.setState({
            searchList: searchList,
        });
    }
    dataClone = (obj) => {
        // 先检测是不是数组和Object
        // let isArr = Object.prototype.toString.call(obj) === '[object Array]';
        let isArr = Array.isArray(obj);
        let isJson = Object.prototype.toString.call(obj) === '[object Object]';
        if (isArr) {
            // 克隆数组
            let newObj = [];
            for (let i = 0; i < obj.length; i++) {
                newObj[i] = this.dataClone(obj[i]);
            }
            return newObj;
        } else if (isJson) {
            // 克隆Object
            let newObj = {};
            for (let i in obj) {
                newObj[i] = this.dataClone(obj[i]);
            }
            return newObj;
        }
        // 不是引用类型直接返回
        return obj;
    };
    onChange = (e) => {
        const _this = this;

        if (e.target.value == '') {
            _this.setState({
                searchboxState: false,
                activeKey: ['1']
            })
        }
        // const _this = this;
        // let evt = e || e.target;
        // let timer = null;
        // if(timer) clearTimeout(timer);
        // timer = setTimeout(function(){
        //     _this.search(evt)
        // },500);

    };

    install(packageName, version) {
        const _this = this;    //先存一下this，以防使用箭头函数this会指向我们不希望它所指向的对象。
        const params = {
            packageName: packageName,
            version: version
        };

        _this.setSearchList(_this, packageName, version, intl.get('PACKAGE_INSTALLING'));
        axios.post(`${noteApiPath2}/package/install`, {
            packageName: packageName,
            version: version,
            projectId: projectId,
            teamId,
        }).then(response => {
            let status = response.status;
            let data = response.data;
            if (status == 200) {
                const _data = JSON.stringify(data)
                console.log(_data);
                if (data.indexOf('keep_alive') !== -1) {
                    const newD = JSON.parse(_data).split('\n');
                    if (newD.length > 0 && newD[0] == "keep_alive") {
                        const res = JSON.parse(newD[newD.length - 1]);
                        if (res.code === 21000000) {
                            let newData = _this.dataClone(_this.state.installedList);
                            _this.setSearchList(_this, packageName, version, "已安装");
                            newData.unshift(params);
                            _this.setState({installedList: newData});
                        } else {
                            _this.setSearchList(_this, packageName, version, intl.get('PACKAGE_FAILED'), res.message);
                        }
                    }
                } else {
                    _this.setSearchList(_this, packageName, version, intl.get('PACKAGE_FAILED'), '安装失败');
                }
            }
        }).catch(function (error) {
            _this.setSearchList(_this, packageName, version, intl.get('PACKAGE_FAILED'), error.message);
        })
    }

    installAction(props) {
        const item = props.item;
        if (intl.get('PACKAGE_INSTALL') == item.istate || intl.get('PACKAGE_FAILED') == item.istate) {
            return (
                <a key="search-install" onClick={() => props.onClick()}>
                    {item.istate}
                </a>
            );
        } if (intl.get('PACKAGE_INSTALLING') == item.istate) {
            return (
                <LoadingOutlined></LoadingOutlined>
            );
        } else {
            return (
                <span>{item.istate}</span>
            );
        }
    }

    setInstalledListState = (installedList, packageName, version, state) => {
        for (let item of installedList) {
            if (packageName === item.packageName && version === item.version) {
                item.state = state;
            }
        }
        this.setState({ installedList });
    }

    uninstall = (packageName, version) => {
        let installedList = [...this.state.installedList];
        this.setInstalledListState(installedList, packageName, version, 'uninstalling');
        const _this = this;
        packageApi.uninstall({ packageName, version })
            .then(function (response) {
                let installedList = [..._this.state.installedList];
                let index = -1;
                for (let i = 0; i < installedList.length; i++) {
                    if (packageName === installedList[i].packageName && version === installedList[i].version) {
                        index = i;
                        installedList[i].state = 'uninstalled';
                        break;
                    }
                }
                if (index !== -1) {
                    installedList.splice(index, 1);
                    _this.setState({ installedList });
                }
            })
            .catch(function (err) {
                console.log(err);
                _this.setInstalledListState(installedList, packageName, version, '');
                message.error(intl.get('PACKAGE_UNINSTALL_FAILED'));
            });
    }
    collapsePanelChange = (key) => {
        this.setState({
            activeKey: key
        })

    };

    render() {
        return (
            <div id="package-manager">
                {/* <Input
                    placeholder={intl.get('PACKAGE_SERACH')}
                    disabled={this.state.searchDisable}
                    onPressEnter={(e) => this.search(e)}
                    onChange={(e) => this.onChange(e)}
                    suffix={suffix}
                    style={{
                    margin: '5px 0',
                    width: 299,
                    borderRadius:0,
                    paddingLeft:this.state.searchDisable?30:10
                    }}
                /> */}
                <Search
                    className={'search-package-input'}
                    placeholder={intl.get('PACKAGE_SERACH')}
                    disabled={this.state.searchDisable}
                    allowClear
                    onSearch={this.search}
                    onChange={this.onChange}
                    style={{
                        width: 301,
                        borderRadius: 0,
                    }}
                    loading={this.state.searchDisable}
                />

                {/* <div style={{
                    position:"absolute",
                    width:10,
                    top:10,
                    left:5,
                    zIndex:99,
                    display: this.state.searchDisable ? '': 'none',
                    textAlign: 'center'
                    }}>
                    <LoadingOutlined />
                </div> */}
                <div
                    className="package-list-box"
                    style={{
                        display: "flex",
                        flexDirection: "column",
                        minHeight: 250,
                        height: document.body.clientHeight - 95
                    }}
                >

                    <div style={{
                        display: this.state.searchboxState ? '' : 'none',
                        flexGrow: 1,
                        maxHeight: document.body.clientHeight - 165,
                        overflow: "auto"
                    }} ref={this.searchListRef}>
                        <List
                            size="small"
                            itemLayout="horizontal"
                            dataSource={this.state.searchList}
                            className='list-serch'
                            renderItem={item => (
                                <List.Item actions={[<this.installAction item={item} onClick={() => this.install(item.packageName, item.stableVersion)}></this.installAction>]}>
                                    <List.Item.Meta
                                        title={<strong>{item.packageName + "    " + item.stableVersion}</strong>}
                                        description={
                                            item.error ? (
                                                <div>
                                                    <div>{item.description}</div>
                                                    <Tooltip
                                                        placement='left'
                                                        title={item.error}
                                                        mouseEnterDelay={1}
                                                        overlayClassName='tipname'
                                                    >
                                                        <div style={{ color: 'red', cursor: 'pointer' }}>
                                                            {item.error.slice(0, 70)}...
                                                        </div>
                                                    </Tooltip>
                                                </div>
                                            ) : (
                                                <div>
                                                    {item.description}
                                                </div>
                                            )
                                        }
                                    />
                                </List.Item>
                            )}
                        />
                    </div>
                    <Collapse
                        bordered={false}
                        // defaultActiveKey={this.state.activeKey}
                        activeKey={this.state.activeKey}
                        accordion
                        style={{
                            display: "flex",
                            flexDirection: "column"

                        }}
                        onChange={this.collapsePanelChange}
                    >

                        <Panel
                            className={'installed-Panel'}
                            header={intl.get('PACKAGE_INSTALLED')}
                            key="1"
                            style={{
                                display: "flex",
                                flexDirection: "column",
                                flexGrow: 2,
                                // maxHeight:document.body.clientHeight - 137,
                            }}
                        >
                            <List
                                size="small"
                                style={{
                                    flexGrow: 2,
                                    maxHeight: document.body.clientHeight - (this.state.searchboxState ? 442 : 165),
                                    overflow: "auto"
                                }}
                                itemLayout="horizontal"
                                loading={this.state.installedListLoading}
                                dataSource={this.state.installedList}
                                renderItem={item => (
                                    <List.Item style={{ paddingTop: 4, paddingBottom: 0 }}
                                        extra={
                                            <Button type="link"
                                                onClick={() => this.uninstall(item.packageName, item.version)}
                                                loading={'uninstalling' === item.state}>
                                                {intl.get('PACKAGE_UNINSTALL')}
                                            </Button>
                                        }>
                                        <List.Item.Meta
                                            title={item.packageName + "    " + item.version}
                                        />
                                    </List.Item>
                                )}
                            />
                        </Panel>
                        <Panel header={intl.get('PACKAGE_RECOMMEND')} key="2">
                            <List
                                size="small"
                                itemLayout="horizontal"
                                dataSource={this.state.usefulList}
                                style={{
                                    flexGrow: 2,
                                    maxHeight: document.body.clientHeight - (this.state.searchboxState ? 442 : 172),
                                    overflow: "auto"
                                }}
                                renderItem={item => (
                                    <List.Item>
                                        <List.Item.Meta
                                            title={item.packageName + "    " + item.version}
                                        />
                                    </List.Item>
                                )}
                            />
                        </Panel>
                    </Collapse>
                </div>
            </div>
        );
    }
}

export default Package;
