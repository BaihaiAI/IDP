import Icons from './static/icon';
import { StudioServices } from 'idp-studio';
import { useEffect, useState } from 'react';

const configJson = require("./config.json");

const IpynbRaw = (props) => {
  const [content, setContent] = useState('');

  useEffect(() => {
    StudioServices.contentApi.cat({ path: '/demo.idpnb' }).then((res) => {
      const value = res.data.content;
      setContent(JSON.stringify(value, null, 2));
    })
  }, []);

  return (
    <div style={{ height: document.body.clientHeight - 50, overflow: 'scroll' }}>
      <pre>
        {content}
      </pre>
    </div>
  );
}

const routeConfig = {
  key: 'ipynbRaw',
  name: () => 'ipynbRaw',
  iconUnChecked: <Icons.rocketIcon style={{ fontSize: 30 }} />,
  iconChecked: false,
  menuClassName: '',
  flg: true,
  component: IpynbRaw,
  configJson,
  notNeedExact: true,
  weight: 20,
  needCache: true
}

export default {
  id: 'ipynbRaw',
  autoStart: true,
  type: 'router',
  activate: (dom) => {
    dom.register('menu', {
      routeConfig,
      id: `${configJson.fileName}/${configJson.entry}`
    })
  }
}
