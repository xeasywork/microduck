# WebRTC 会话、信令与控制通道

> 对应英文原文：[`remote-webrtc.md`](remote-webrtc.md)

## 0. 状态

摄像头推流和 WebRTC 控制已在硬件运行。文档记录了丢帧、旋转、自动曝光、CPU 和 GStreamer 插件问题。质量被统一为一个用户档位，再映射到分辨率、帧率和码率。

## 1～2. 边界和会话

WebRTC 不是新业务 API，只是远程传输。完整会话规划包含视频、音频、可靠 `control` JSON-RPC 和不可靠 `teleop`。当前重点是视频与 control；未来 teleop 必须丢弃过期输入。

## 3. 信令

`mediad` 运行本地信令服务器，并拥有完整 PeerConnection。媒体和 datachannel 不能拆给不同进程。

## 4. 鉴权

当前局域网入口没有完整会话鉴权，这是明确限制。公网桥接必须负责认证和授权；实验室可用不等于适合家庭公网暴露。

## 5. 控制通道

control 把现有 JSON-RPC 路由到各服务。WebRTC 比 BLE 开放更多普通控制，但不开放维护级底层写权限。

## 6. teleop

可靠通道适合低频命令，不适合摇杆。teleop 需要不可靠传输、序号、超时和控制权规则。

## 7. 公网

跨 NAT 需要信令、STUN、TURN、认证和带宽预算；局域网直连无需 TURN。

## 8～11. 更新、权限、构建和延后项

远程更新和控制权需要额外同意规则。`mediad` 依赖固定 GStreamer/MPP/sysroot。延后项包括 Agent WebSocket + 按需 JPEG、teleop、多 peer、隐私指示和 TURN。

