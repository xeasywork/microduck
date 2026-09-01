# 部署与系统配置

> 对应英文原文：[`README.md`](README.md)

本文说明如何把空白 RK3566 板卡准备成 Microduck、区分开发板和普通设备，以及安装流程写入哪些系统内容。

## 快速路径

私有仓库下，从电脑运行 `scripts/provision-board.sh`，提供板卡地址、读取 token 和团队开发公钥。也可以在板上直接运行 `provision.sh`。未来公开制品不再需要仓库 token，普通设备只信任正式发布密钥。

## 安装过程

`provision.sh` 依次协调：

1. `setup-board.sh` 安装系统包、用户、权限和硬件支持；
2. `migrate-network.sh` 迁移到 NetworkManager；
3. `install.sh` 安装版本、systemd 单元、配置和策略；
4. 重启并进行健康检查；
5. 按参数决定是否启用开发通道。

## 开发板、token 与信任链

开发板额外信任团队开发密钥，可安装 `--ref <branch>`。普通设备只能接受正式签名。私有仓库 token 使用最小读取权限，不应写入镜像或日志。

```text
私钥签名 manifest
→ 板载公钥验证
→ 校验制品摘要
→ 原子安装
→ 健康门
→ 失败回滚
```

## 文件位置

- `/opt/robot/releases/<version>`：不可变发布；
- `/opt/robot/daemon/current`：当前发布链接；
- `/etc/robot`：配置和信任材料；
- `/var/lib/robot`：身份、更新状态和持久记录；
- `/run/<service>`：socket 和短期状态。

## 日志和版本

服务日志进入 journald。排障时要区分安装目录版本、当前进程 revision、systemd 的 ExecStart 和更新器 active/golden 记录；不一致通常表示重启或链接切换未完成。

