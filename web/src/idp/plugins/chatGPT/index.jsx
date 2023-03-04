import globalData from 'idpStudio/idp/global';
import { RegisterApi } from 'idpStudio/idp/register';
import ChatGPT from './pages/ChatGPT';
import RightIcons from 'idpStudio/components/Icons/RigftIcons';

const configJson = require('./config.json');

const rightSide = {
  key: 'chatGPT',
  title: () => 'ChatGPT',
  icon: <RightIcons.ChatGPTIcon style={{ fontSize: 30 }} />,
  menuItemStyle: {
    paddingLeft: '9px',
    paddingTop: '1px',
  },
  component: <ChatGPT />,
  weight: 6
}

if (process.env.REACT_APP_VERSION === 'SAAS') {
  globalData.register(RegisterApi.right_side_api, {
    rightSide,
    id: `${configJson.fileName}/${configJson.entry}`,
    title: 'ChatGPT'
  })
}