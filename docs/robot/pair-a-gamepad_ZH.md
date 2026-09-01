# 配对游戏手柄

> 对应英文原文：[`pair-a-gamepad.md`](pair-a-gamepad.md)

## 进入配对模式

按手柄型号操作，使指示灯快速闪烁。不同 Xbox、8BitDo 或兼容手柄步骤不同。

## 配对

```bash
sudo robotctl pad pair
```

多设备时指定 MAC：

```bash
sudo robotctl pad pair <MAC>
```

成功流程同时完成 paired 和 trusted，保证重启后自动重连。

## 检查和遗忘

```bash
robotctl pad status
sudo robotctl pad forget <MAC>
```

## 完全无法 bond

`btd` 的默认 BlueZ agent 可能与配对流程冲突。可使用 provisioning 的 `--pause-btd-on-pair`，或在明确诊断下临时停止 `btd`、重置 Bluetooth adapter、完成配对后再启动服务。

## 每次都失败

重新运行板卡 setup，重启后检查 BlueZ Privacy、agent、trusted 记录和 journal。`btmon` 可抓取底层握手。

## 驾驶中断线

使用 `robotctl monitor`、`pad-link-test.sh` 和 journal 区分无线掉线、输入卡住、`padd` 重启和控制循环问题。停止收到新输入时机器人应安全停止。

## 比较两块板

运行 `pad-stack-report.sh --fingerprint`，比较内核、BlueZ、配置、固件、服务和 USB/BT 硬件，避免只看 `pad status` 得出错误结论。

