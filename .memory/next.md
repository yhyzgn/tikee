# 下一步任务

执行 `.prompt/003-worker-tunnel.md`：

1. 新增 proto/gRPC crate，例如 `crates/scheduler-proto`。
2. 使用当前最新稳定的 tonic/prost 工具链。
3. 定义最小 Worker Tunnel protobuf。
4. 实现 Worker 主动注册与心跳的 server 侧最小服务。
5. 增加内存连接 registry 雏形。
6. 补充注册/心跳测试。
7. 保持 HTTP/OpenAPI 现有验证全部通过。
8. 更新 `.memory` 和 `.prompt/004-storage-and-scheduler.md`。
9. 提交并推送。
