---
title: Java Spring Boot Starter
description: Java SDK, Spring adapter, and Spring Boot starter compatibility docs.
---

# Java Spring Boot Starter

The Java SDK is a Gradle multi-module SDK with separate Spring Framework adapters and Spring Boot starter compatibility lines.

## Verify the SDK

```bash
cd sdks/java
./gradlew test --no-daemon
./gradlew jar sourcesJar --no-daemon
```

## Verify demos

```bash
cd examples/java/spring-boot2-worker-demo && ./gradlew test --no-daemon
cd examples/java/spring-boot3-worker-demo && ./gradlew test --no-daemon
cd examples/java/spring-boot4-worker-demo && ./gradlew test --no-daemon
```

## Compatibility rule

Java modules must keep explicit source/resource/test boundaries. Do not replace compatibility modules with empty source-set indirection.

## Layering model

The Java SDK is intentionally split into native Java, Spring Framework adapters, and Spring Boot starter compatibility lines. This preserves a clear source/resource/test boundary for Spring Boot 2, 3, and 4 without hiding compatibility behind Gradle source-set tricks.

## Processor dispatch

Spring workers use explicit processor binding metadata. The starter adapts annotated processor methods into worker handlers, maps exceptions to failed task outcomes, and lets the Worker Tunnel lifecycle start and stop with the Spring application lifecycle.

## Evaluation checklist

- Run SDK module tests and jar generation.
- Run Boot 2, Boot 3, and Boot 4 demo tests independently.
- In live mode, confirm the worker appears in the Web console with Java/Spring capabilities.
- Trigger a job routed to a Java processor.
- Validate logs/results and graceful shutdown behavior.

## Production notes

Use constructor injection for Spring beans and avoid field injection. Keep `clientInstanceId` as a client hint; the authoritative worker id is assigned by the Server during registration.
