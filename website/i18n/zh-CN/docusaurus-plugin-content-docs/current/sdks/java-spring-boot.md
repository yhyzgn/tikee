---
title: Java Spring Boot Starter
description: Java SDK、Spring adapter 与 Spring Boot starter 兼容线说明。
---

# Java Spring Boot Starter

Java SDK 是 Gradle 多模块工程，包含原生 Java SDK、Spring Framework adapter，以及 Spring Boot 2 / 3 / 4 starter 兼容线。它适合已有 Spring 体系的业务服务把处理器接入 Tikeo Worker Tunnel。

## 运行时要求

Java SDK 运行时要求 Java 17+；仓库 CI 使用 Temurin 21 进行构建验证。Spring Boot 2、3、4 demo 分别独立存在，不应合并成空模块或伪 sourceSet 兼容。

## 验证 SDK

```bash
cd sdks/java
./gradlew test --no-daemon
./gradlew jar sourcesJar --no-daemon
```

## 验证 demo

```bash
cd examples/java/spring-boot2-worker-demo && ./gradlew test --no-daemon
cd examples/java/spring-boot3-worker-demo && ./gradlew test --no-daemon
cd examples/java/spring-boot4-worker-demo && ./gradlew test --no-daemon
```

## 分层模型

Java SDK 明确拆分为 native Java、Spring Framework adapter 和 Spring Boot starter。这样可以为 Boot 2、Boot 3、Boot 4 分别保留真实源码、资源、测试与依赖边界。

## Processor 派发

Spring Worker 使用显式 processor binding metadata。starter 将带注解的 processor 方法适配为 worker handler，把异常映射为失败结果，并让 Worker Tunnel 生命周期跟随 Spring 应用生命周期启动和停止。

## 评估清单

运行 SDK 测试和 jar 生成；分别运行 Boot 2、Boot 3、Boot 4 demo 测试；live mode 下确认 Worker 在 Web 控制台展示 Java/Spring capability；触发 Java processor 任务；验证日志、结果与优雅停机。

## 适合场景

Java / Spring Boot Worker 适合已有 Spring 服务希望复用 bean、配置体系、监控和企业依赖治理的团队。评估重点不是“能启动一个 Spring 应用”，而是 starter 是否能把处理器绑定、生命周期、异常结果、日志和优雅停机纳入 Tikeo 的统一执行证据。

对企业项目来说，还应验证依赖版本、日志格式、健康检查和应用关闭钩子是否符合现有规范。
