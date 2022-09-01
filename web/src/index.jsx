import './public-path';
import ReactDOM from 'react-dom';
import '../config/open-config';
import { Provider } from 'react-redux';
import { BrowserRouter } from "react-router-dom";
import PrepareApp from './PrepareApp';
import App from '@/pages/App'
import './index.less';
import zhCN from 'antd/lib/locale/zh_CN';
import enUS from 'antd/lib/locale/en_US';
import cookie from 'react-cookies';

import { store } from '@/store';
import { ConfigProvider } from "antd";
import IdpIdle from './components/IdleDetector';
import { loadModule } from '@/public-modules';

// @ts-ignore
let pages = require.context("../extension", true, /\/.*config\.json$/);
pages.keys().map((key, index, arr) => {
    let config = pages(key);
    let module = require("../extension/" + config.fileName + '/' + config.entry);
    if (!module.hasOwnProperty('default')) {
        const { plugin } = require("../extension/" + config.fileName + '/' + config.entry);
        module = plugin;
    } else {
        module = module.default;
    }
    if (module.autoStart) {
        module.activate(loadModule(module.type))
    }
});

function render(props) {
    const { container } = props;
    ReactDOM.render(<ConfigProvider locale={cookie.load('locale') === 'zhCN' ? zhCN : enUS}>
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
    const userId = cookie.load('userId');
    const region = cookie.load('region');
    let projectId = new URLSearchParams(window.location.search).get('projectId');
    if (!projectId) {
        projectId = projectId = window.localStorage.getItem("historyOpenProject");
    }
    if ((!userId || !region)) {
        history.pushState({}, null, '/login')
        return;
    }
    if (userId && region && !projectId) {
        history.pushState({}, null, '/team')
    }
    if (userId && region && projectId) {
        render(props);
    }
}

export async function unmount(props) {
    const { container } = props;
    ReactDOM.unmountComponentAtNode(container ? container.querySelector('#root') : document.querySelector('#root'));
}
