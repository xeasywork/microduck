# 文档索引

> 对应英文原文：[`README.md`](README.md)

## `robot/`：手边有一台机器人

| 中文文档 | 内容 |
|---|---|
| [`cheatsheet_ZH.md`](robot/cheatsheet_ZH.md) | `robotctl` 日常命令速查 |
| [`pair-a-gamepad_ZH.md`](robot/pair-a-gamepad_ZH.md) | 配对手柄和故障处理 |
| [`cheatsheet-dev_ZH.md`](robot/cheatsheet-dev_ZH.md) | 开发板、候选发布和分支构建 |
| [`dev-push_ZH.md`](robot/dev-push_ZH.md) | 从电脑构建并通过 SSH 安装 |
| [`duckctl_ZH.md`](robot/duckctl_ZH.md) | 通过蓝牙从电脑访问机器人 |
| [`install-dev_ZH.md`](robot/install-dev_ZH.md) | 从空白系统准备开发板 |
| [`install-by-hand_ZH.md`](robot/install-by-hand_ZH.md) | 手工拆解安装步骤 |

## `design/`：修改守护进程

| 中文文档 | 负责的机制 |
|---|---|
| [`architecture_ZH.md`](design/architecture_ZH.md) | 服务划分、IPC、状态所有权、安全和权限 |
| [`robotd-design_ZH.md`](design/robotd-design_ZH.md) | 控制循环、传感器、观测、ONNX 和安全 |
| [`updater-design_ZH.md`](design/updater-design_ZH.md) | 更新、签名、健康门和回滚 |
| [`restart-order_ZH.md`](design/restart-order_ZH.md) | 更新和启动时的重启顺序 |
| [`app-path-design_ZH.md`](design/app-path-design_ZH.md) | 手机、BLE、`btd` 与 `configd` |
| [`remote-webrtc_ZH.md`](design/remote-webrtc_ZH.md) | WebRTC 会话、信令和控制通道 |
| [`webrtc-console_ZH.md`](design/webrtc-console_ZH.md) | 浏览器控制台和设备发现 |
| [`boot-recovery-net_ZH.md`](design/boot-recovery-net_ZH.md) | 启动失败后的黄金版本恢复 |

## `project/` 和 `ideas/`

`project/` 保存带时间背景的硬件实验、故障复盘和路线图；`ideas/` 保存尚未完成设计的想法。后者描述的是规划，不代表功能已经实现。

