# Microduck 命令速查

> 对应英文原文：[`cheatsheet.md`](cheatsheet.md)

## `robotctl` 基础

```bash
robotctl version
robotctl health
robotctl monitor
robotctl monitor --json --hz 50 > run.jsonl
sudo robotctl configure
```

`version` 查看实际运行 revision；`health` 给出统一健康结论；`monitor` 实时显示循环频率、电池、姿态、策略和传感器。

## 关节电源

```bash
sudo robotctl robot init
sudo robotctl robot relax --yes
```

`init` 初始化并站起；`relax` 释放扭矩。操作前保证机器人有支撑和足够空间。

## 手柄

```bash
robotctl pad status
sudo robotctl pad pair
sudo robotctl pad pair <MAC>
sudo robotctl pad forget <MAC>
```

常用映射包括移动、坐站、捡拾、左右踢球和前滚翻；具体型号和固件可能有差异。配对问题见 [`pair-a-gamepad_ZH.md`](pair-a-gamepad_ZH.md)。

## 声音与互动

```bash
robotctl quack
robotctl chorale
robotctl theremin
```

`quack` 播放声音；`chorale` 让多只已同意社交的机器人协同演唱；`theremin` 用 ToF 手部距离控制音高和嘴部。

## ToF

检查深度服务和硬件：

```bash
journalctl -u tofd -b
sudo i2cdetect -y -r 3
```

`robotctl monitor` 可观察深度区域状态。无目标、无效测量和传感器故障必须区分。

## Wi-Fi

```bash
robotctl net status
robotctl net scan
sudo robotctl net connect <ssid> --psk <passphrase>
sudo robotctl net connect <ssid> --psk-stdin
sudo robotctl net forget <ssid>
```

密码优先通过 stdin，避免进入 shell 历史。

## 身份和电源

```bash
robotctl system info
robotctl system pin
sudo robotctl system set-name <name>
sudo robotctl system set-pin <six-digits>
sudo robotctl system reboot
```

名称和 PIN 属于设备持久状态，不随 release 回滚。

## 更新

```bash
robotctl update status
robotctl update check daemon
sudo robotctl update apply daemon
sudo robotctl update rollback daemon
robotctl update log
robotctl update show
robotctl update watch
sudo robotctl update select daemon <version>
sudo robotctl update reset-to-golden daemon
```

更新会验证签名、运行安装钩子、重启服务和健康检查。应用过程中断线不一定是失败，应重连后查询状态。

## 日志

```bash
journalctl -u robotd -b
journalctl -u updaterd -b
journalctl -u mediad -b
```

排障同时记录 `robotctl version`、`robotctl health` 和服务启动身份。

