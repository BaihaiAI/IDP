![](docs/logo_new.png)

[![LICENSE](https://img.shields.io/badge/licence-Apache%202.0-brightgreen)](https://github.com/BaihaiAI/IDP/blob/main/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-brightgreen)](https://www.rust-lang.org/)
[![Language](https://img.shields.io/badge/language-javascript-brightgreen)](https://www.javascript.com/)
[![Language](https://img.shields.io/badge/language-Python-brightgreen)](https://www.python.org/)


## What is IDP?

IDP is an open source web IDE with strong support for Python & SQL language, contains code assistance, file and environment management at the same time.
IDP is designed for data scientists, algorithm engineers and big data engineers to maximize working efficiency all through the AI development process.
It is Written in Rust to exhibits excellent performance and security. 

## Key Features
IDP aims to help data scientists and algorithm engineers focus more on their own work,  algorithm development by taking care of all the engineering work, from environment and file management to plug-in development and configuration, etc..

The core features of IDP are as follows:
* Mixed language support: deeply support Python, SQL and Markdown language in the same notebook.
* Data visualization: generating data insights directly using in-built data visualization tools, e.g.,bar charts, scatter charts, line charts, etc..
* Automatic versioning: automatic tracking and managing of code changes; clear and convenient version comparison. 
* Coding assistance: powerful coding assistance functions including intelligent code completion, hovor, diagnostic and quickfix.
* Package manager: search and manage python packages easily and efficiently.
* Variable manager: interactively browse and manage variables and conveniently compare different algorithm approaches and parameter settings.
* Managing environment: conveniently cloning a Python/system environment for reuse to avoid tedious repeated configuration.


![](docs/open.png)

## Quick Start

### Start IDP within Docker
``` bash
> docker pull baihaiopensource/idp-studio
> docker run -p 3000:3000 baihaiopensource/idp-studio
```
Then open http://localhost:3000 with your browser.

### Start IDP with pre-built binary

- [Windows x86_64](http://baihai.cn-bj.ufileos.com/package/idp-studio-v1.0.0-win-x64.tar.gz)
- [macOS macOS](http://baihai.cn-bj.ufileos.com/package/idp-studio-v1.0.0-darwin-arm64.tar.gz)
- [Linux x86_64](http://baihai.cn-bj.ufileos.com/package/idp-studio-v1.0.0-linux-x64.tar.gz)

### Start with IDP on Cloud:
Quickly check out IDP SaaS with <https://www.baihai.co/invitation.html>

### Build from source
See [Compile And Deploy IDP From Source](/docs/compile-deploy-from-source.md)


## Contributing
Please read [contributing.md](/docs/contributing.md) for details on the process for submitting issues and pull requests to us.

## Code of Conduct
Please refer to the [Code of Conduct](/docs/code-of-conduct.md), which describes the expectations for interactions within the community.


## Shows
- [Understand IDP in 4 minutes](https://www.bilibili.com/video/BV1Ja411o7to/?spm_id_from=333.337.search-card.all.click)
- [Getting start with IDP](https://www.bilibili.com/video/BV1Qa411f7as/?spm_id_from=333.337.search-card.all.click)
- [Others](https://space.bilibili.com/1227589642)

## Community
If you have any question, feel free to reach out to us in the following ways:
 - [On Slack](https://join.slack.com/t/idp-tjo1834/shared_invite/zt-1kee8cd8x-iNZ0rvwClRfx7sLgmmKKyg)
 - [On Twitter](https://twitter.com/baihaiAI)
 - [On Linkedin](https://www.linkedin.com/company/80179567/admin/)
 - [Mail to us](https://baihai.co/contactus.html)


## Case Study
[Case Study](https://www.baihai.co/case.html) list our use cases.

## License
IDP is licensed under [Apache-2.0](LICENSE).
