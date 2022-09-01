### 项目主要的配置：
- react 18+
- webpack 5+
- react-router-dom 6+

### 启动命令说明：
- dev `开发/插件环境调试`
- dll:build `生成线上环境代码，必须先执行npm run dll`
- dll `生成dll环境代码`
- build `npm run dll 和 npm run dll:build 同步执行，生成线上环境代码`
- build:plugs `生产插件环境代码`

### api接口说明：
- IdpMenus.registerIdpMenu('xxx', options: any):void `注册内部头部菜单栏功能`
> 例子：
```
IdpMenus.registerIdpMenu('idps', {
    menuType: 'Menu | Tool',
    content: <>...</>,
});
```
`编辑器显示行号`
```
IdpTools.utilslineNumbers, 
```
`折叠所有输入`
```
IdpTools.utils.collapseAllInput,
```
`折叠所有输出`
```
IdpTools.utils.collapseAllOutput,
```
`输出字符是否换行，默认换行`
```
IdpTools.utils.autoWarpOutput,
```
`快捷键组建是否显示`
```
IdpTools.utils.sneltoetsListVisible,
```
`反馈antd modal`
```
IdpTools.utils.feedbackView,
```
`对应修改值的api方法`
```
IdpTools.utils.openFeedbackView(),
IdpTools.utils.closeFeedbackView(),
IdpTools.utils.updateLineNumbers(str:boolean),
IdpTools.utils.updateCollapseAllInput(str:boolean),
IdpTools.utils.updateCollapseAllOutput(str:boolean),
IdpTools.utils.updateAutoWarpOutput(str:boolean),
IdpTools.utils.updateSneltoetsListVisible(str:boolean)
```
`获取菜单点击值（antd Menu组件）`
```
IdpTools.getMenuKey(): Promise<any>
```

### 项目代码支持配置：
- *.css
- *.less
- *.module.less
- js/jsx
- ts/tsx

### 项目检查配置：
- eslintrc
- tslint

### 将要实现的配置：
- antd (✔️)
- qiankun（微服务）| 等待集成...
- redux（reduxjs/toolkit）(✔️)
- nodejs（❌）
- 懒加载（❌）
- 插件化项目搭建（plugs）(✔️)
- 迁移idp项目，目前实现插件化开发 | 正在迁移重构...


### api说明：


### 目录概述：
```
idp-studio
├─.eslintrc.js 
├─.gitignore
├─babel.config.json
├─index.html
├─package-lock.json
├─package.json
├─server.js
├─tree.text
├─tsconfig.json
├─yarn.lock
├─webpack
|    ├─webpack.base.config.js
|    ├─webpack.dev.config.js
|    ├─webpack.dll.config.js
|    └─webpack.pro.config.js
├─typings
|    └typings.d.ts
├─src
|  ├─index.tsx
|  ├─pages
|  |   ├─index.tsx
|  ├─layouts
|  ├─components
|  ├─assets
├─extensions
├─dll
├─dist

```