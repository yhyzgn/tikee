# 下一步任务

执行 `.prompt/006-worker-sdk-rust-and-java-starter.md`：

1. 实现 Rust Worker SDK 最小 crate：主动连接 Worker Tunnel、注册、心跳。
2. 定义基础任务处理器接口，但不要要求业务应用暴露入站端口。
3. 规划并初始化 Java SDK 目录结构，优先 Spring Boot Starter 模式。
4. Java 侧建议模块：`scheduler-java-core`、`scheduler-spring-boot-autoconfigure`、`scheduler-spring-boot-starter`。
5. 保持 `{code,message,data}` HTTP 响应规范和现有 API trigger flow。
6. 保持 HTTP/OpenAPI、storage、Worker Tunnel 单元测试通过。
7. 更新设计路线图、`.memory` 和后续 `.prompt`。
8. 提交并推送。
