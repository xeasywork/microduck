# 安装开发板

> 对应英文原文：[`install-dev.md`](install-dev.md)

## 刷写和准备

刷入项目支持的板卡系统，准备 SSH、网络、仓库读取 token 和团队开发公钥。确认目标板卡地址，避免在错误设备上 provision。

## 自动安装

```bash
./scripts/provision-board.sh --pause-btd-on-pair --name <name> radxa@<ip>
```

`--pause-btd-on-pair` 用于避免当前硬件/BlueZ 组合下 `btd` 与手柄配对 agent 冲突。脚本会处理重启并继续跟踪日志。

## 三种蓝牙配置

文档区分正常设置、关闭 Privacy 的兼容设置和 `DUCK_WEIRD_BLE` 特殊路径。只有确认板卡属于相应情况时才使用 workaround；恢复正常配置时删除标记并重启。

## 验证

```bash
robotctl health
sudo robotctl update apply --ref main daemon
```

还应核对服务 revision、开发公钥和 updater 配置。

## 重刷后的 SSH

host key 变化只在确认设备确实重刷后使用 provisioning 脚本的 `--forget-host-key` 处理。

## 把现有设备变成开发板

手工安装团队开发公钥、启用 `allow_dev_keys` 并重启 `updaterd`。私有仓库 token 可通过权限为 600 的 systemd drop-in 提供，不能写入公共配置或日志。

## 无网络安装

可从 USB/本地目录侧载 release：

```bash
sudo robotctl update apply daemon --from /media/usb/release
```

正常情况仍走更新器。只有恢复场景才直接运行 updater install，并应先停止运动服务。

