# 手柄成功绑定所需的最小配置

> 对应英文原文：[`pad-minimal-pairing.md`](pad-minimal-pairing.md)

## 可工作的流程

文档记录在真机上从最小 BlueZ/systemd 配置开始，使手柄进入配对模式、执行 `pad pair`、确认 bond 并在重启后自动重连的过程。

成功不只是当场出现输入事件，还包括 BlueZ 保存 bond、设备被标记 trusted、启动后有合适 agent/连接策略，以及 `padd` 能读取。

## 已观察失败

- 能发现但无法 bond；
- 当次连接成功但重启后不回连；
- agent 能力或默认角色不正确；
- `btd` 与配对流程争夺 adapter/agent；
- 把“建立 bond”和“保持连接”两类故障误读为一个问题。

## 结论

问题与 `btd` 运行和 BlueZ 默认 agent 的生命周期有关。配对时临时 agent 与日常手机 BLE 服务需要明确交接，不能依赖偶然启动顺序。

## 未测试差异

仍需对不同手柄、固件、BlueZ 版本、USB 适配器和旧 runtime 设置逐项对照；不要把单块板的成功直接推广到所有设备。

