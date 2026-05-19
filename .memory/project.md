# 项目记忆：scheduler

`scheduler` 是 Rust 原生分布式任务调度平台，设计目标是企业公共调度基础设施。

核心不可破坏约束：

- Worker 主动连接 Server，所有反向调用复用 gRPC/HTTP2 tunnel。
- Server 不直连 Worker，不要求 Worker 暴露入站端口。
- Server 不执行用户脚本或用户代码。
- K8s / Docker / 跨集群 / 跨 VPC 部署必须是一等能力。
- Web UI 与 HTTP/OpenAPI 管理接口是一等平台能力。
- 每次开发推进必须编译、测试、运行/冒烟，通过后提交并推送。
- 每次推进后必须更新 `.memory` 和后续 `.prompt`。

设计源：`design/scheduler-architecture-design.md`。
