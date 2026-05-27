# Phase4 P2：拓扑图形画布、跨工作流影响分析与回放

## 背景
任务依赖自动发现已提供拓扑数据和轻量表格抽屉，但用户验收目标已升级：需要直接具备图形画布、跨工作流影响分析，以及可用于事故复盘的回放能力。

## 目标
- 拓扑 API 返回前端可直接渲染的布局坐标和分层信息。
- Jobs 页面从表格拓扑升级为 SVG graph canvas，保留依赖/引用/缺失引用列表作为辅助信息。
- 新增跨工作流影响分析 API：给定 Job，返回引用它的工作流、上游/下游 Job 和风险提示。
- 新增工作流实例 replay bundle API：一次性返回 workflow instance、definition、events timeline 和 graph，供前端/SDK 做事故复盘。

## API
- `GET /api/v1/jobs/topology`：扩展 node `position/layer` metadata。
- `GET /api/v1/jobs/{job}/impact`：返回 `targetJob`、`referencingWorkflows`、`upstreamJobs`、`downstreamJobs`、`riskSummary`。
- `GET /api/v1/workflow-instances/{id}/replay`：返回 `instance`、`workflow`、`events`、`graph`。

## 前端
- Jobs 页面“任务拓扑”抽屉加入 SVG graph canvas：节点按 job/workflow 类型区分，依赖边/引用边不同颜色，支持选中 Job 后加载影响分析。
- 增加影响分析区：展示引用工作流、上游/下游、风险摘要。
- 回放 API 先由 client 暴露，后续可接入 Workflow 实例详情页。

## 验收
- 后端测试覆盖 impact 跨工作流关系。
- 后端测试覆盖 workflow instance replay bundle。
- 前端 client 测试覆盖 impact/replay endpoints。
- JobsPage 源码测试覆盖 SVG graph canvas 和影响分析 UI。
