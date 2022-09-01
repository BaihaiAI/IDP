import React, { Fragment, useEffect } from 'react';
import { configure } from 'mobx'; // 开启严格模式
import './index.less';
import Header from '@/idp/component/header';
import { Layout } from "antd"
import LeftNav from "@/pages/common/leftNav"
import Content from "@/layout/content"
import FooterBar from "@components/../layout/footer/FooterBar";
import { AliveScope } from 'react-activation'

// @ts-ignore
let pages = require.context("../idp/plugins", true, /\/.*config\.json$/);
pages.keys().map((key, index, arr) => {
    let config = pages(key);
    if ( Object.prototype.toString.call(config) === '[object Array]') {
        config.forEach(conf => {
            require("@/idp/plugins/" + conf.fileName + '/' + conf.entry);
        });
    } else {
        require("@/idp/plugins/" + config.fileName + '/' + config.entry);
    }
});

type Props = {};
configure({ enforceActions: 'never' }) // 开启严格模式

const App: React.FC<Props> = () => {
    return (
        <AliveScope>
            <Header />
            <Layout>
                <LeftNav />
                <Content />
            </Layout>
            <FooterBar />
        </AliveScope>
    )
}

export default App;
