# `duckctl` 命令说明

> 对应英文原文：[`duckctl.md`](duckctl.md)

`duckctl` 从电脑通过蓝牙发现、配置和查询机器人，无需 SSH。它面向机器人而不是某一种底层传输。

## 安装与发现

```bash
cargo install --path duckctl
duckctl scan
duckctl --name <robot-name> info
```

若旧工具 `btctl` 仍存在，应卸载以免混淆。

## 网络地址和控制台

```bash
duckctl ip
duckctl open
ssh radxa@$(duckctl ip)
```

`open` 获取机器人地址并打开板载 WebRTC 控制台。

## 固定目标机器人

多台设备环境中使用 `--name` 或 `DUCK_ROBOT`，不要默认连接扫描到的第一台。稳定身份使用机器人 ID/名称，不使用随机化蓝牙地址。

## 身份和电源

```bash
duckctl --name <robot> info
duckctl --name <robot> name <new-name>
duckctl --name <robot> reboot
```

## Wi-Fi

```bash
duckctl --name <robot> wifi status
duckctl --name <robot> wifi scan
duckctl --name <robot> wifi connect <ssid> --psk <passphrase>
duckctl --name <robot> wifi forget <ssid>
```

## 健康和版本

```bash
duckctl --name <robot> health
duckctl --name <robot> status
duckctl --name <robot> version
```

## 更新

```bash
duckctl --name <robot> update check
duckctl --name <robot> update status
duckctl --name <robot> update versions
duckctl --name <robot> update log --limit 20
duckctl --name <robot> update apply
duckctl --name <robot> update rollback
duckctl --name <robot> update select <version>
duckctl --name <robot> update watch
```

开发板还可使用 `--ref` 或 `--staging`。更新期间 BLE 断线后重新连接并查询状态。

## 通用调用

```bash
duckctl --name <robot> call <method> '<json-params>'
```

它只调用 BLE 路由允许的方法，不能绕过权限或执行维护级电机写入。

## 故障处理

使用 `--verbose` 查看发现和连接信息。若找不到设备，检查 Bluetooth 服务、机器人广播、名称选择、配对状态和是否有其他进程占用 adapter。

