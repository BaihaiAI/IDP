import './public-path';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import { BrowserRouter } from "react-router-dom";
import PrepareApp from './PrepareApp';
import App from '@/pages/App'
import './index.less';
import zhCN from 'antd/lib/locale/zh_CN';
import enUS from 'antd/lib/locale/en_US';
import cookie from 'react-cookies';
import { store } from './store';
import { ConfigProvider } from "antd";
import globalData from "idpStudio/idp/global";
import IdpIdle from '@/components/IdleDetector';

// 用那个放开那个插件
const loadDevPlugins = [
    require('../extension/centre/src/index'),
    // require('../extension/contact/src/index'),
    require('../extension/dataSet/src/index'),
    // require('../extension/environment/src/index'),
    // require('../extension/feedback/src/index'),
    // require('../extension/monitor/src/index'),
    // require('../extension/networkstatus/src/index'),
    // require('../extension/notification/src/index'),
    // require('../extension/teams/src/index'),
    // require('../extension/tensorboard/src/index'),
    // require('../extension/usage/src/index'),
    // require('../extension/workflow/src/index'),
    require("../extension/modelwarenhouse/src/index"),
    // require("../extension/optuna/src/index"),
    // require("../extension/colony/src/index"),
];

loadDevPlugins.forEach(module => {
    if (!module.hasOwnProperty('default')) {
        module = module.plugin;
    } else {
        module = module.default;
    }
    if (module.autoStart) {
        module.activate(globalData);
    }
});

function render(props) {
    const { container } = props;
    ReactDOM.render(<ConfigProvider locale={cookie.load('locale') === 'enUS' ? enUS : zhCN}>
        <Provider store={store}>
            <BrowserRouter basename={window.__POWERED_BY_QIANKUN__ ? '/studio' : (process.env.NODE == 'dev' ? '/' : '/child/idpStudio-idp/')}>
                <PrepareApp >
                    <App />
                </PrepareApp>
                <IdpIdle />
            </BrowserRouter>
        </Provider>
    </ConfigProvider>, container ? container.querySelector('#root') : document.querySelector('#root'));
}

if (!window.__POWERED_BY_QIANKUN__) {
    render({});
}

export async function bootstrap() { }

export async function mount(props) {
    render(props)
}

export async function unmount(props) {
    const { container } = props;
    ReactDOM.unmountComponentAtNode(container ? container.querySelector('#root') : document.querySelector('#root'));
}
