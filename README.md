# DICOM-SIMULATOR-DESKTOP-APP

## 描述

![界面1](./dicom-interface.png)

![界面2](./dicom-simulator.png)

dicon-simulator 的桌面模拟器版本

1.支持拉取工作列表
2.支持MPPS
3.支持C-STORE
4.可配置
5.支持多语言，多主题

TODO:
1.自定义脚本内容
2.UPS


## 初始化项目

> NOTE: 本项目依赖rust开发环境，需要先安装rust环境
> NOTE: 本项目依赖python开发环境，需要先安装python环境，或者下载python312，解压到项目目录

```shell
# 安装前端依赖
pnpm install
# 启动测试(第一次启动会比较耗时间)
pnpm tauri dev
# 打包为可执行文件 支持 Windows mac 等
pnpm tauri build
```
