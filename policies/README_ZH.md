# 运动策略

> 对应英文原文：[`README.md`](README.md)

这里存放 `robotd` 当前运行的 ONNX。所有运动策略必须满足：

```text
obs[1,61] -> actions[1,14]
```

`robotd` 在加载时检查形状，避免错误模型在运动中才暴露。

## 临时存放位置

长期目标是把策略作为独立 `model` 更新组件放到 Hugging Face Hub，使动作更新不必重新发布整个守护进程。目前模型随仓库发布，保证 release 自包含。

## 文件和角色

| 文件 | 角色 |
|---|---|
| `alpha_walking.onnx` | 全向行走 / velstand |
| `alpha_stand.onnx` | 站立和身体姿态 |
| `alpha_sitstand.onnx` | 坐下与站起 |
| `alpha_ground_pick.onnx` | 捡拾循环 |
| `ball_kick_left/right.onnx` | 左右脚踢球 |
| `roller.onnx` | 滑轮移动 |
| `roller_crouch.onnx` | 滑轮下蹲 |
| `roulade.onnx` | 前滚翻 |

文件名表示运行角色，不代表某一次训练 run。

当前行走角色使用 `567fdcd` 中的 `BEST_alpha_walking_flat.onnx`。MuJoCo 实测它能响应
前进、后退、左右平移和左右转向；后来的崎岖地面权重虽然改善了头部水平和轻微斜坡
鲁棒性，但后退和平移能力出现了明显回归，因此暂不作为默认权重。

## 61 维协议

旧原型存在多种观测宽度，当前守护进程只构建统一的 61 维观测，其中末尾 13 维为速度、头部和身体命令。旧模型会在加载时被明确拒绝。

## 使用自定义策略

在 `robotd.toml` 的 `[policy]` 中填写绝对路径并重启 `robotd`。加载失败会通过 `robot.health` 报告，控制循环继续保持姿态。
