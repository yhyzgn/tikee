# 下一步任务

执行 `.prompt/009-worker-dispatch.md`：

1. 扩展 Worker Tunnel 协议，支持 Server 向 Worker 下发任务、Worker 回传执行结果。
2. 在 server 侧实现 pending job_instance 到在线 worker 的最小分发循环。
3. 在 `scheduler-worker-sdk` 中接收任务并调用 `TaskProcessor`。
4. 更新实例状态：pending -> running -> succeeded/failed，并记录基础错误信息。
5. 增加 HTTP/存储测试与 SDK 集成测试，确保统一 `{code,message,data}` 接口契约不回退。
6. 保持 Worker 主动出站连接模型；不得要求 Worker 暴露入站端口。
7. 按 cargo、maven、bun、docker/compose 既有命令全量验证。
8. 更新设计路线图、`.memory` 和后续 `.prompt`，提交并推送。
