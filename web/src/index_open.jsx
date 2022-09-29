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
import IdpIdle from './components/IdleDetector';

import { store } from '@/store';
import { ConfigProvider } from "antd";

(function(){
    const token = new URLSearchParams(window.location.search).get('token');
    cookie.save('token', token);
})()

function render(props) {
    const { container } = props;
    ReactDOM.render(<ConfigProvider locale={cookie.load('locale') === 'zhCN' ? zhCN : enUS}>
        <Provider store={store}>
            <BrowserRouter basename={window.__POWERED_BY_QIANKUN__ ? '/studio' : (process.env.NODE == 'dev' ? '/' : './')}>
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
    render(props);
}

export async function unmount(props) {
    const { container } = props;
    ReactDOM.unmountComponentAtNode(container ? container.querySelector('#root') : document.querySelector('#root'));
}
