# Microduck 项目说明

> 对应英文原文：[`README.md`](README.md)

Microduck 是一台约 25 cm 高、约 800 g 的双足机器人，由强化学习策略驱动。当前仓库是机器人的运行端：它在 RK3566 板卡上运行 50 Hz 控制循环，管理 15 个舵机、无线通信、摄像头、声音和更新系统。

动作策略在相邻的 `microduck_rl` 仓库中训练，导出为 ONNX 后由本仓库的 `robotd` 加载。

## 已有能力

- 手柄控制双足行走和滑轮移动；
- 捡拾、坐下/站起、左右踢球和前滚翻；
- 被推倒后等待稳定并起身；
- 摄像头通过 WebRTC 推流；
- ToF 深度传感器和麦克风互动；
- 蓝牙配网、状态查询和更新；
- 带签名验证、健康检查和自动回滚的更新系统。

## 文档入口

- 日常使用：[`docs/robot/cheatsheet_ZH.md`](docs/robot/cheatsheet_ZH.md)
- 系统原理：[`docs/design/architecture_ZH.md`](docs/design/architecture_ZH.md)
- 开发板安装：[`docs/robot/install-dev_ZH.md`](docs/robot/install-dev_ZH.md)
- 从电脑推送分支：[`docs/robot/dev-push_ZH.md`](docs/robot/dev-push_ZH.md)
- 中文文档索引：[`docs/README_ZH.md`](docs/README_ZH.md)

## 进程概览

`robotd` 负责运动控制；`mediad` 负责摄像头、音频和 WebRTC；`tofd` 负责深度；`padd` 负责手柄；`btd` 负责蓝牙入口；`configd` 负责配置；`updaterd` 负责更新。

这些进程通过统一 JSON-RPC 合同协作，手机、网页、手柄和脚本最终都发送同一种机器人意图。

