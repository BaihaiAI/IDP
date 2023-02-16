![](docs/logo_new.png)

[![LICENSE](https://img.shields.io/badge/licence-Apache%202.0-brightgreen)](https://github.com/BaihaiAI/IDP/blob/main/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-brightgreen)](https://www.rust-lang.org/)
[![Language](https://img.shields.io/badge/language-javascript-brightgreen)](https://www.javascript.com/)
[![Language](https://img.shields.io/badge/language-Python-brightgreen)](https://www.python.org/)


## 什么是IDP？
IDP是一款开源AI IDE，原生支持Python和SQL——这是在AI和数据科学领域中使用最广泛的两种语言。

IDP旨在让数据科学家和算法工程师的工作效率更加高效。此外，由于内核使用Rust语言编写，IDP具有出色的运行性能。

## 核心功能
IDP可以帮助数据科学家和算法工程师更专注于核心工作——算法开发。IDP可以优化AI工程全流程的工作，从环境和文件的管理到插件的开发和配置，等等...

IDP的核心功能如下：
* 支持混合语言: 在同一个notebook中能够同时支持Python、SQL和Markdown语言。
* 数据可视化: 可以直接使用内置的数据可视化工具生成数据分析结果图，如柱状图、散点图、线状图等。
* 自动化版本管理: 拥有跟踪和管理代码更改和清晰方便的版本比较功能。
* 辅助用户编码: 拥有强大的编码辅助功能，包括智能代码补全、检查代码错误和快捷修复。
* 包管理器: 轻松有效地搜索和管理Python软件包。
* 变量管理器: 可以交互式浏览和管理变量，方便比较不同的算法方法和参数设置。
* 环境管理: 可以方便地克隆Python环境/系统环境以复用，避免再次进行麻烦的环境配置。

![](docs/open.png)

## 快速上手指南

### 在Docker内启动IDP
``` bash
> docker pull baihaiopensource/idp-studio
> docker run -p 3000:3000 baihaiopensource/idp-studio
```
然后用在浏览器打开 http://localhost:3000。

### 使用预编译的二进制包启动IDP

- [Windows x86_64](http://baihai.cn-bj.ufileos.com/package/idp-studio-v1.0.0-win-x64.zip)
- [macOS arm64](http://baihai.cn-bj.ufileos.com/package/idp-studio-v1.0.0-darwin-arm64.tar.gz)
- [macOS x86_64](http://baihai.cn-bj.ufileos.com/package/idp-studio-v1.0.0-darwin-x64.tar.gz)
- [Linux x86_64](http://baihai.cn-bj.ufileos.com/package/idp-studio-v1.0.0-linux-x64.tar.gz)

注意：在Linux下用预编译的二进制包启动IDP，需要Python 3.7及以上版本（CPython和PyPy）。

```
wget http://baihai.cn-bj.ufileos.com/package/idp-studio-v1.0.0-linux-x64.tar.gz
tar zxf idp-studio-v1.0.0-linux-x64.tar.gz
cd idp-studio-v1.0.0-linux-x64
./idp
```

### 在Cloud上启动IDP:
快速了解IDP SaaS，请点击 <https://www.baihai.co/invitation.html>

### 通过编译源码部署IDP
请参阅[从源代码编译和部署IDP](/docs/compile-deploy-from-source.md)


## 贡献方式
请阅读[contribution.md](/docs/contributing.md)，了解向IDP提交issues和pull requests的详细过程。

## 社区行为守则
请阅读[社区行为守则](/docs/code-of-conduct.md)，该文档描述了在IDP开源社区需要遵守的行为准则。

## IDP演示
- [4分钟了解IDP核心亮点功能](https://www.bilibili.com/video/BV1Ja411o7to/?spm_id_from=333.337.search-card.all.click)
- [快速上手算法开发生产](https://www.bilibili.com/video/BV1Qa411f7as/?spm_id_from=333.337.search-card.all.click)
- [更多精彩内容](https://space.bilibili.com/1227589642)

## 怎样联系我们
如果你有任何关于IDP的问题，欢迎通过以下方式与我们取得联系：
 - [On Slack](https://join.slack.com/t/idp-tjo1834/shared_invite/zt-1kee8cd8x-iNZ0rvwClRfx7sLgmmKKyg)
 - [On Twitter](https://twitter.com/baihaiAI)
 - [On Linkedin](https://www.linkedin.com/company/80179567/admin/)
 - [Mail to us](https://baihai.co/contactus.html)


## 经典案例
IDP的[经典案例](https://www.baihai.co/case.html)。

## IDP使用的开源许可证
本项目使用 [Apache-2.0 License](LICENSE).
