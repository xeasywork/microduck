# 从本机构建并安装到板卡

> 对应英文原文：[`dev-push.md`](dev-push.md)

## 首次准备

安装项目要求的交叉构建工具，例如：

```bash
cargo install cargo-zigbuild --locked
```

板卡必须已完成开发配置、SSH 可达并信任团队开发密钥。

## 日常循环

使用仓库的 `scripts/dev-push.sh` 构建 arm64 制品、上传、安装并运行健康检查。脚本负责使用正确 release 布局，不要手工覆盖 `/opt/robot/daemon/current/bin`。

推送后检查：

```bash
robotctl version
ssh radxa@<ip> robotctl health
ssh radxa@<ip> 'journalctl -f -u robotd -u configd -u btd -u padd'
```

需要回退时：

```bash
sudo robotctl update rollback daemon
```

## 调高日志

通过 systemd drop-in 设置 `RUST_LOG`，执行 `daemon-reload` 并重启对应服务。调试结束后删除 drop-in，避免长期高日志影响存储和性能。

## 只验证、不安装

脚本支持构建/打包验证模式；也可在容器中使用固定 sysroot。`cargo board --bins` 可只为板卡编译二进制。

## 侧载已有制品

```bash
sudo robotctl update apply daemon --from ~/duck-sideload
```

侧载仍应走更新器和健康门，不直接复制文件。

## 常见失败

- 重刷后 SSH host key 变化；
- 板卡地址变化；
- token 或开发公钥未安装；
- `updaterd` 仍运行旧 revision；
- 交叉构建误用主机库；
- 推送成功但服务未真正重启。

使用 `--forget-host-key` 只针对确认重刷的设备，不能无差别清理 SSH 信任。

## 非目标

开发推送不是正式发布，不替代 CI 签名、候选通道或用户更新流程。

