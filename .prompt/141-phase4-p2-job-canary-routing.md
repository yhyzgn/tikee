# Phase4 P2：任务灰度发布基础

## 背景
任务版本和回滚已落地，但线上变更仍缺少低风险流量切分。完整灰度需要 worker label、版本策略、自动回滚和指标闭环；本阶段先提供可运行的 Job-level canary 路由基础。

## 本阶段目标
- Job 增加可选 canary 配置：目标 Job ID 与百分比。
- 普通 UI/API trigger 主 Job 时，Server 按确定性采样决定是否路由到 canary Job。
- Response 保持返回实际创建的 instance/job，并附带 canaryRouting 元数据，便于 UI/SDK 知道是否命中灰度。
- 不影响定时调度和工作流 materialization；灰度只作用于显式 trigger 路径。

## 范围
- Storage：jobs 增加 canary_job_id / canary_percent。
- HTTP：Create/Update/JobSummary/Trigger response 扩展。
- Web：Jobs 创建/编辑支持 canary 配置，触发成功提示是否命中灰度。

## 验收
- 后端测试：主 Job canaryPercent=100 时 API trigger 实际创建 canary Job instance，并返回 canaryRouting.enabled=true/routed=true/originalJobId/main->routedJobId/canary。
- 兼容性：canaryPercent 缺省/0 时仍触发原 Job。
- 前端 client 与 JobsPage 源码测试覆盖字段与提示。
