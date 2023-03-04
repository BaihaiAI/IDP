import { Button, Input, Modal, Form } from 'antd';
import './index.less';

const ChatGPTSetting = (prop) => {
  const { visible, handleOk, handleCancel, apiKey } = prop;
  const [form] = Form.useForm();

  return (
    <Modal
      className="chat-setting"
      title="设置"
      visible={visible}
      onCancel={handleCancel}
      footer={<>
        <Button htmlType="button" onClick={() => {
          handleOk(form.getFieldsValue());
        }}>
          确定
        </Button>
      </>}
    >
      <Form layout="vertical"
        form={form}
        initialValues={{
          apiKey: apiKey,
        }}
      >
        <Form.Item name="apiKey" label={<><b style={{marginRight: '2px'}}>API Key: </b> 
          (登录
          <a
            href="https://openai.com/api/"
            target="_blank"
            style={{ padding: '0px 2px' }}
          >https://openai.com/api/</a>
          从用户信息中获取API Key)
        </>} >
          <Input onChange={(e) => form.setFieldsValue({ apiKey: e.target.value })} />
        </Form.Item>
      </Form>
    </Modal>
  );
}

export default ChatGPTSetting;