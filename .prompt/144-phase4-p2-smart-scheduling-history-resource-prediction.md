# Phase4 P2：智能调度完整历史耗时与资源预测

## 背景
调度建议接口已能判断当前 Job 是否具备可执行 Worker 能力，但仍缺少完整历史耗时统计和基于 Worker 资源标签的预测能力。用户要求直接补齐“完整历史耗时/资源预测”。

## 目标
- `GET /api/v1/jobs/{job}/scheduling-advice` 返回历史耗时统计：inspected/completed/failed、avg、p50、p95、max。
- 返回资源预测：estimated duration、recommended concurrency、eligible worker count、advertised CPU/memory、预测原因。
- Jobs 页面调度建议抽屉展示历史耗时与资源预测。
- 保持原 readiness 字段兼容：ready/severity/reason/requiredCapability/eligibleWorkers/recentInstances/recentFailures。

## 数据来源
- 历史耗时基于 `job_instances.created_at/updated_at` 的 terminal rows（succeeded/failed）计算。
- Worker 资源能力来自在线 eligible worker labels：`cpu` 与 `memory_mb`。
- 无历史时使用保守兜底估算，避免返回空结构。

## 验收
- 后端测试覆盖两条成功实例的 avg/p95/预测字段。
- 前端 client 测试覆盖 history/prediction 字段解析。
- JobsPage 测试覆盖“历史耗时 / 资源预测”展示文案。
