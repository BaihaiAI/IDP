
import React, { useState, useEffect } from 'react';
import { useDispatch } from 'react-redux';
import { Select } from 'antd';
import './SqlCell.less';
import dataSetApi from '@/services/dataSetApi';
import { updateCellMetadata, contentUpdateCellSource } from '@/store/features/notebookSlice';

const defaultDataSource = 'local_csv'
const DataSourceSelect = ({  cellId, placeholder, dataSource,path }) => {
  const dispatch = useDispatch();
  const [dataSourceList, setDataSourceList] = useState([]);

  // 获取数据源
  const getDataSourceList = () => {
    dataSetApi.getActiveDataBaseList().then(res => {
      if (res.code === 200 && res?.data) {
        let arr = []
        for (const item of res.data) {
          if (item.type !== 'FileSystem') {
            arr.push({
              id: item.alias,
              value: item.alias,
              label: item.alias
            })
          }
        }
        arr.push({
          id: defaultDataSource,
          value: defaultDataSource,
          label: defaultDataSource
        });
        setDataSourceList(arr)
      }
    })
  }

  // 选择数据源
  const onSelectChange = (value) => {
    dispatch(updateCellMetadata({
      path,
      cellId, metadata: {
        dataSource: value
      }
    }));
    dispatch(contentUpdateCellSource({ path, cellId }))
  }

  useEffect(() => {
    getDataSourceList();
    if (!dataSource) {
      onSelectChange(defaultDataSource)
    }
  }, [])

  return (
    <div key={`${cellId}-datasource`} className='select-wrap'>
      <Select
        style={{ width: '100%' }}
        placeholder={placeholder}
        className='select-class'
        value={dataSource}
        bordered={false}
        options={dataSourceList}
        onChange={onSelectChange}
        onDropdownVisibleChange={(open) => {
          if (open) {
            getDataSourceList();
          }
        }}
      >
      </Select>
    </div>
  )
}

export default DataSourceSelect;
