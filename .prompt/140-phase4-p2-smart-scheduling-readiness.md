# Phase4 P2：智能调度建议基础

## 背景
“智能调度”完整形态需要历史耗时、Worker 负载、失败率、资源预测和策略引擎。为了先提升服务可用性，本阶段实现只读的调度就绪/风险建议：在用户触发任务前，告诉他是否有在线 Worker 能执行、需要什么能力、最近执行是否稳定。

## 本阶段目标
- 新增 `GET /api/v1/jobs/{job}/scheduling-advice`。
- 基于 Job 绑定、在线 Worker 能力、最近实例状态生成建议。
- 不改变现有调度路径，仅提供 UI/API 可观测建议。

## 建议规则
- SDK Processor 任务需要 `processor:<name>` 能力。
- Script 任务在基础阶段需要 `script` 能力（后续可结合脚本语言细分）。
- 在线 Worker 数为 0 或没有匹配能力时 `ready=false`，severity 为 error。
- 最近实例存在失败时生成 warning。

## 验收
- 后端测试覆盖：无匹配能力时返回 not ready；注册匹配 Worker 后返回 ready。
- 前端 API client 测试覆盖 endpoint。
- Jobs 页面动作区增加“调度建议”入口，抽屉展示 ready/severity/reason/requiredCapability/eligibleWorkers。
- 设计文档 Phase4 P2 标记“智能调度建议基础”完成，完整资源预测仍保留后续增强。
