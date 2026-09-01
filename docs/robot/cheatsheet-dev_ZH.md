# 开发板命令速查

> 对应英文原文：[`cheatsheet-dev.md`](cheatsheet-dev.md)

## 开发通道

```bash
sudo robotctl update apply --ref <branch> daemon
sudo robotctl update apply --ref main daemon
```

仅安装了团队开发公钥并允许开发签名的板卡可以使用 `--ref`。

## 候选发布

```bash
sudo robotctl update apply --staging daemon
sudo robotctl update apply --staging --version <version> daemon
```

候选版本仍需签名和健康门，不等于跳过验证。

## 更新后的常见问题

先运行：

```bash
robotctl health
```

若配置或更新服务仍是旧进程，可按诊断结果重启相应服务，而不是盲目重启全部：

```bash
sudo systemctl restart configd
sudo systemctl restart updaterd
```

## 从电脑构建并安装

详细流程见 [`dev-push_ZH.md`](dev-push_ZH.md)。推送后核对版本、健康和实际运行 exe。

## 在电脑使用手柄驱动

先停止板载 `padd`，通过 SSH 转发 `robotd` socket，然后本机运行 `padd`：

```bash
sudo systemctl stop padd
ssh -L /tmp/robotd.sock:/run/robotd.sock radxa@<ip>
cargo run -p padd -- --socket /tmp/robotd.sock
```

结束后恢复板载服务。不要让两个 `padd` 同时争夺控制。

