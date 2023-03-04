import { Input, Button, Layout, List, Avatar } from 'antd';
import { SendOutlined, UserOutlined } from '@ant-design/icons';
import RightIcons from 'idpStudio/components/Icons/RigftIcons';
import { useState, useEffect } from 'react';
import cookie from 'react-cookies';
import ReactMarkdown from 'react-markdown';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { useClipboard } from 'use-clipboard-copy';
import intl from 'react-intl-universal';
import { ChatGPTProvider, getChatGPTAccessToken, sendMessageFeedback } from './api/chatgpt';
import { OpenAIProvider } from './api/openai';
import ChatGPTSetting from './ChatGPTSetting';
import openaiProxy from './api/openaiProxy';
import './index.less';

const { Sider } = Layout;
const { TextArea, Group } = Input;

const ChatGPT = (props) => {
  const [data, setData] = useState([]);
  const [question, setQuestion] = useState('');
  const [providerType, setProviderType] = useState('GPT3');
  const [provider, setProvider] = useState(null);
  const [end, setEnd] = useState(true);
  const [token, setToken] = useState('');
  const [showSetting, setShowSetting] = useState(false);
  const [displayCopy, setDisplayCopy] = useState('none');
  const [copyName, setCopyName] = useState(intl.get('CHATGPT_COPY'));
  const clipboard = useClipboard();
  
  const receiveMsg = (event) => {
    if (event.type === 'done') {
      setEnd(true);
    } else {
      setData([...data, {
        id: event.data.messageId,
        user: 'chatbox',
        text: event.data.text.trim(),
        messageId: event.data.messageId,
        conversationId: event.data.conversationId,
      }]);
    }
  }

  const sendMsg = () => {
    if (question === '') return;
    const newData = [...data, {
      id: `${new Date().getTime()}`,
      user: 'me',
      text: question,
    },{
      id: `${new Date().getTime()}`,
      user: 'chatbox',
      text: '',
    }];
    setData(newData);
    setQuestion('');
    setEnd(false);
    
    if (token) {
      provider.generateAnswer({
        prompt: question,
        onEvent: receiveMsg,
      }).catch((err) => {
        console.log(err);
        setData([...data, {
          id: `${new Date().getTime()}`,
          user: 'chatbox',
          text: 'The server had an error while processing your request. Sorry about that! You can retry your request.',
        }]);
        setEnd(true);
      });
    } else {
      openaiProxy.generateAnswer({
        prompt: question,
        onEvent: (({ target }) => {
          const { responseText } = target;
          setData([...data, {
            id: `${new Date().getTime()}`,
            user: 'chatbox',
            text: responseText,
          }]);
        }),
      }).then((res) => {
        setEnd(true);
      }).catch((err) => {
        console.log(err);
        setEnd(true);
      });
    }
  }

  const handleInputTextChange = (e) => {
    setQuestion(e.target.value);
  }

  const initProvider = async () => {
    if (providerType === 'chatGPT') {
      const token = await getChatGPTAccessToken();
      const provider = new ChatGPTProvider(token);
      setProvider(provider);
    } else if (providerType === 'GPT3') {
      const provider = new OpenAIProvider(token, 'text-davinci-003');
      setProvider(provider);
    }
  }

  const listItemMeta = (item) => {
    if (item.user === 'me') {
      return (
        <List.Item.Meta
          avatar={<Avatar shape="square" icon={<UserOutlined style={{ color: 'white' }} />} style={{ backgroundColor: '#1890ff' }} size={24} />}
          description={<div style={{ wordBreak: 'break-all' }}>{item.text}</div>}
        />
      );
    } else {
      return (
        <List.Item.Meta
          avatar={<RightIcons.ChatGPTUserIcon />}
          description={<div>{item.text.indexOf('```') !== -1 ?
            <>
              <ReactMarkdown
                children={item.text}
                components={{
                  code({ node, inline, className, children, ...props }) {
                    const match = /language-(\w+)/.exec(className || '')
                    return !inline && match ? (
                      <div className="code-box"
                        onMouseOver={() => setDisplayCopy('')}
                        onMouseOut={() => {
                          setDisplayCopy('none');
                          setCopyName(intl.get('CHATGPT_COPY'));
                        }} >
                        <Button size="small"
                          className="copy-button"
                          type="link"
                          onClick={() => {
                            clipboard.copy(String(children));
                            setCopyName(intl.get('CHATGPT_COPIED'));
                          }} 
                          style={{ display: displayCopy }}
                        >{copyName}</Button>
                        <SyntaxHighlighter
                          children={String(children).replace(/\n$/, '')}
                          // style={dark}
                          language={match[1]}
                          PreTag="div"
                          {...props}
                        />
                      </div>
                    ) : (
                      <code className={className} {...props}>
                        {children}
                      </code>
                    )
                  }
                }}
              />
            </> : item.text.indexOf('\n') !== -1 ? <pre style={{ color: '#374151'}}>{item.text}</pre> :
              <span style={{ wordBreak: 'break-all', color: '#374151' }}>{item.text}</span>}</div>}
        />
      );
    }
  }
  
  useEffect(() => {
    setData([{
      id: `${new Date().getTime()}`,
      user: 'chatbox',
      text: '欢迎使用chatGPT，系统默认密钥因为使用人数较多，使用可能会受限，您可通过右上角设置使用自己的密钥。',
    }]);
    const chatGPTAppKey = cookie.load('chatGPTAppKey');
    if (chatGPTAppKey) {
      setToken(chatGPTAppKey);
    }
  }, []);

  useEffect(() => {
    if (data.length > 0) {
      const id = data[data.length - 1].id;
      document.getElementById(id).scrollIntoView();
    }
  }, [data]);

  useEffect(() => {
    if (token) {
      initProvider();
    }
  }, [token])
  

  return (
    <Sider
      theme="light"
      width="300"
      style={{ height: document.body.clientHeight - 40 }}
    >
      <div className="chat-top-bar">
        <Button
          type="link"
          className="setting-button"
          onClick={() => setShowSetting(true)}
        >{intl.get('CHATGPT_SETTING')}</Button>
      </div>
      <List
        className="chat-box"
        dataSource={data}
        renderItem={(item) => (
          <List.Item key={item.id} id={item.id}>
            {listItemMeta(item)}
          </List.Item>
        )}
        style={{ height: document.body.clientHeight - 148, width: 295, paddingLeft: 8, overflow: 'scroll' }}
      >
      </List>
      <div className="chat-input-box">
        <Group compact>
          <TextArea
            rows={2}
            onChange={handleInputTextChange}
            value={question}
          />
          <Button
            icon={<SendOutlined rotate={315} />}
            type="text"
            // size="small"
            onClick={sendMsg}
            style={{ borderTop: '1px #d9d9d9 solid' }}
            loading={!end}
          ></Button>
        </Group>
      </div>
      <ChatGPTSetting
        visible={showSetting}
        handleOk={({ apiKey }) => {
          setToken(apiKey);
          cookie.save('chatGPTAppKey', apiKey);
          setShowSetting(false);
        }}
        handleCancel={() => setShowSetting(false)}
        apiKey={token}
      />
    </Sider>
  );
}

export default ChatGPT;