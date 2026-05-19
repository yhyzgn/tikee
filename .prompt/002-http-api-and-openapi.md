# 002-http-api-and-openapi：HTTP 管理接口与 OpenAPI

> 本阶段提示词需在 001-bootstrap 完成后由执行智能体根据实际代码结构更新。

## 预期目标

- 建立 `/api/v1` HTTP API 基础结构。
- 实现 Auth placeholder、Job CRUD skeleton、Instance query skeleton。
- 暴露 `/openapi.json` 与 `/docs`。
- 建立统一错误响应、分页结构、trace_id。
- 为 Web UI 和 CLI 提供稳定管理面接口基础。

## 依赖

- 001-bootstrap 已完成并通过 fmt / clippy / test / build / healthz 冒烟。

## 完成后更新

- `.memory/*`
- `.prompt/003-worker-tunnel.md`
