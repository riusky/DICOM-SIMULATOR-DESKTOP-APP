# DICOM-SIMULATOR

## 描述

dicon-simulator 的桌面模拟器版本

1.支持拉取工作列表
2.支持MPPS
3.支持C-STORE
4.可配置

TODO:
自定义脚本内容

## 配置资源文件

> 在发行版下载资源文件，并解压到对应的位置
> Python312.zip 解压缩到安装目录, python 环境是安装之后解压到对应的位置

## 初始化项目

> NOTE: 本项目依赖rust开发环境，需要先安装rust环境

```shell
# 安装前端依赖
pnpm install
# 启动测试(第一次启动会比较耗时间)
pnpm tauri dev
# 打包为可执行文件 支持 Windows mac 等
pnpm tauri build
```
