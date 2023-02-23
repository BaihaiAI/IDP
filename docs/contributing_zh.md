---
title: Contributing
---

<div align="right">

  [English](contributing.md) | [简体中文](contributing_zh.md)

</div>


# 如何为IDP做贡献？

## 贡献流程

1. 提交一个issue
2. 向你分配（assign）issue
3. 创建一个包含issue编号的PR草案
4. 通过CI检查
5. 将PR的状态改为"Ready for review"
5. 经过3名以上成员审查和批准
6. 合并PR

## 编码规范

请阅读[编码规范](/docs/code-style.md)。

## PR分支的命名方式

分支命名方式为短横线式命名（kebab-case），要求前缀为以下类型之一：

- feature: 这个PR为代码库引入了一个新功能
- fix: 这个PR修复了代码库中的一个错误
- refactor: 这个PR修改了代码，但是没有引入新的功能或进行错误修复
- ci: 这个PR改变了 build/testing/ci 步骤
- docs: 这个PR改进了文档
- chore: 这个PR只有一些不需要记录的小改动，比如说编码风格

git 分支名称示例: chore/ci-add-typo-checking

## commit内容的风格

`[分支名](#issue编号): commit内容`

git commit内容示例: `[chore/ci-add-typo-checking](#1): CI add typos cli tools to check typo`
