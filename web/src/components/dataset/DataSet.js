import React from 'react';
import { useState, useEffect } from 'react';
import { Card, Button, Upload, message } from 'antd';

import './DataSet.less';
import intl from 'react-intl-universal';
import { noteApiPath } from '../../services/httpClient';

const DataSet = (props) => {
    const [uploading, setUploading] = useState(false);

    const uploadFile = (info) => {
        if (info.file.status === 'done') {
            message.success(`${info.file.name} ${intl.get('UPLOAD_SUCCEEDED')}`);
        } else if (info.file.status === 'error') {
            message.error(`${info.file.name} ${intl.get('UPLOAD_FAILED')}`);
        }
        setUploading(false);
    }

    return (
        <div>
            <Card size="small" title="集成数据集" extra={
                <Upload
                    action={`${noteApiPath}/note/uploadfile?filePath=/localData`}
                    showUploadList={false}
                    onChange={uploadFile}
                >
                    <Button loading={uploading} type="primary" >添加</Button>
                </Upload>
            } style={{ width: 300 }}>
                <p>您可以在文件管理器的 <b>localData</b> 目录中直接访问此次集成的数据集。</p>
            </Card>
        </div>
    );
}

export default DataSet;