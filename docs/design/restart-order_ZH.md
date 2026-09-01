# 服务在何时重启

> 对应英文原文：[`restart-order.md`](restart-order.md)

## 1. 服务集合

发布可能包含七个守护进程。需要启动或重启的 unit 从新发布实际内容推导，不写死易漂移的名单。

## 2. 应用更新顺序

1. 下载并验证 manifest、签名和摘要；
2. 解包到 staging；
3. 预检和孤儿 unit 检查；
4. 安装为不可变 release；
5. 运行 `hooks/postinstall`；
6. 原子切换 `current`；
7. 启动/重启新 unit；
8. 自测和健康门；
9. 成功则提交，失败则回滚；
10. 延后重启 `updaterd`，避免中断自己的事务。

## 3～4. 其他转换和开机

切换已有版本、回滚、首次安装和开发推送都应遵守同一规则。开机后更新器核对磁盘记录、链接目标、进程 revision 和服务身份。

## 5. 启动核对

每个服务发布服务名、版本、revision、可执行路径和 PID。更新器据此判断重启是否真的生效，而不只相信 `systemctl restart` 返回值。

## 6～7. 首装和诊断

首次安装仍要建立完整目录、配置和身份记录。版本错位时检查 `current`、active/golden、ExecStart、进程 exe、postinstall 和更新 transcript。

