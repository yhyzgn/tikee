# Phase4 P2：任务依赖自动发现与拓扑基础

## 背景
Phase3/4 P2 剩余项中，“任务依赖自动发现、拓扑可视化、工作流回放、智能调度”会直接影响团队理解任务编排关系和排障效率。迁移工具已降为最低优先级 backlog，因此本阶段优先落地不依赖外部系统的拓扑基础。

## 本阶段目标
- 从现有 Job 与 Workflow definition 中自动推导任务依赖关系。
- 暴露只读拓扑 API，返回任务节点、工作流节点、依赖边与无法解析的引用。
- 在 Jobs 页面提供轻量拓扑入口，先用表格/抽屉展示依赖关系，为后续图形画布、回放、智能调度做数据基础。

## 范围
- 后端：新增 `GET /api/v1/jobs/topology`，权限沿用 `jobs:read`。
- 数据来源：`JobRepository::list_jobs()` + `WorkflowRepository::list_workflows()`。
- 推导规则：
  - 每个 Job 形成 `job` node。
  - 每个 Workflow 形成 `workflow` node。
  - Workflow DAG 中 `job_id -> job_id` 的 edge 形成 `workflow_job_dependency` 边。
  - Workflow 对 Job 的引用形成 `workflow_job_ref` 边，方便知道任务被哪些工作流使用。
  - Workflow 节点引用不存在的 Job 时进入 `unresolved` 列表。
- 前端：Jobs 页面抽屉展示依赖边和 unresolved 列表。

## 验收
- 后端测试覆盖：从工作流 DAG 中发现 `A -> B` 依赖，并返回缺失 Job 引用。
- 前端 API client 测试覆盖 `/api/v1/jobs/topology`。
- JobsPage 源码测试覆盖拓扑入口存在。
- 更新总设计文档 Phase4 P2 状态：标记“任务依赖自动发现与拓扑可视化基础”完成，工作流回放/智能调度仍保留后续增强。
