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
import globalData from "idpStudio/idp/global";

import { store } from '@/store';
import { ConfigProvider } from "antd";

const loadDevPlugins = [
  // require('../extension/ipynb_raw/src/index'),
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

(function () {
    const token = new URLSearchParams(window.location.search).get('token');
    cookie.save('token', token);
})()

ReactDOM.render(<ConfigProvider locale={cookie.load('locale') === 'zhCN' ? zhCN : enUS}>
    <Provider store={store}>
        <BrowserRouter>
            <PrepareApp >
                <App />
            </PrepareApp>
            <IdpIdle />
        </BrowserRouter>
    </Provider>
</ConfigProvider>, document.querySelector('#root'));