# 手工逐步安装

> 对应英文原文：[`install-by-hand.md`](install-by-hand.md)

此文档把自动 provisioning 拆开，适合调试某一步，不是普通安装的首选。

## 上传文件

把 `setup-board.sh`、`migrate-network.sh`、`install.sh` 及所需 release/token 上传到板卡，不要从未知来源混搭脚本版本。

## 重启前

建立 `robot` 系统组并把开发用户加入组，然后依次运行：

```bash
sudo sh ~/setup-board.sh
sudo sh ~/migrate-network.sh
sudo reboot
```

网络迁移会跨重启继续，SSH 地址可能变化。

## 重启后

重新运行幂等 setup/migration 确认完成，再带所需环境运行：

```bash
sudo -E sh ~/install.sh
```

也可使用 `DUCK_NO_START=1` 先安装不启动，检查后再启用：

```bash
sudo systemctl enable --now updaterd robotd configd btd padd
```

## 媒体板卡

需要摄像头/WebRTC 的板卡运行 `setup-gstreamer.sh` 或板载恢复命令，安装正确插件、权限和 3A 环境。

## 检查

```bash
robotctl health
robotctl version
sudo robotctl pad pair
```

同时检查 journald、systemd 单元、`current` 链接和实际进程 exe。

