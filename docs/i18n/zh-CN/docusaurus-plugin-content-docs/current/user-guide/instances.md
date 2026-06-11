# Instances 运维手册

## 概览

Instances 页面用于查看 Job 和 Workflow 产生的执行记录。这里是确认执行结果、attempt、日志、广播节点和取消状态的主要入口。遇到任务失败、长时间 pending 或广播部分失败时，应先在这里收集证据。

运维依据：页面由 `web/src/pages/InstancesPage.tsx` 提供；主要接口包括 `/api/v1/jobs`、`/api/v1/jobs/{job}/instances`、`/api/v1/instances/{instance}`、`/api/v1/instances/{instance}/attempts`、`/api/v1/instances/{instance}/logs`、`/api/v1/instances/{instance}/cancel`、`/api/v1/instances/stream` 和 `/api/v1/instances/{instance}/logs/stream`。

## 前置条件

- 具备查看实例的权限；取消实例需要执行控制权限。
- 已知道 Job、Workflow 或 instance ID。
- Worker 通过出站 Tunnel 上报日志和结果。
- 浏览器允许控制台接收 SSE；若 stream 不可用，仍可用普通接口刷新。

```bash
curl -fsS http://127.0.0.1:9090/api/v1/instances/INSTANCE_ID \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data | {id,status,result}'
```

## 打开页面

1. 登录控制台。
2. 在左侧菜单选择 **实例**，或打开 `/instances`。
3. 按 Job、状态或时间定位目标实例。
4. 点击实例行打开详情，查看 attempts、logs 和执行结果。

## 常见操作

### 查看实例状态

- `pending`：调度器尚未完成派发，通常要核对 Worker 匹配条件。
- `running`：Worker 已接收任务，等待日志或结果。
- `succeeded`：执行成功，是终态。
- `failed`：执行失败，是终态，需要查看日志和 attempt。
- `partial_failed`：广播执行中至少一个节点失败，需要逐节点检查。

### 查看日志

1. 打开实例详情。
2. 查看持久化日志列表。
3. 如日志正在写入，保持详情抽屉打开观察日志流。
4. 复制 Worker ID、attempt 编号、trace ID 和错误消息，用于跨页面排查。

### 查看广播执行节点

广播实例会按 Worker 或执行节点展示结果。逐个检查节点状态，不要只看实例总状态。出现 partial_failed 时，记录成功节点和失败节点的差异，例如 region、cluster、labels 或 runner 能力。

### 取消实例

1. 确认实例仍处于 active 状态。
2. 确认当前账号有取消权限。
3. 点击取消后刷新详情。
4. Worker 可能继续上报 cleanup 日志；以最终状态和日志为准。

## 验收

- 可以打开一个 succeeded 实例并看到 result、attempt 和日志。
- 可以打开一个 failed 实例并看到失败原因或日志证据。
- 广播或 partial_failed 实例能展示多个执行节点。
- 取消按钮只在有权限且实例可取消时出现。
- `/api/v1/instances/stream` 或刷新后的详情能反映状态变化。

## 故障排查

| 现象 | 处理 |
| --- | --- |
| 实例一直 pending | 检查 Job selector、Worker structured capabilities 和 dispatch queue。 |
| 没有日志 | 检查 Worker 是否在线、是否通过 Tunnel 上报 `TaskLog`。 |
| 状态与日志不一致 | 以详情接口和最新 attempt 为准，必要时刷新页面。 |
| 取消无效 | 确认实例是否已进入终态，或 Worker 是否仍在执行 cleanup。 |
| 广播部分失败 | 对比成功和失败 Worker 的 labels、region、cluster、runner 和版本。 |

## 生产检查清单

- [ ] 每次生产失败都记录 instance ID、Job ID、Worker ID、attempt 和时间。
- [ ] failed 或 partial_failed 必须附带日志或明确的无日志原因。
- [ ] 取消生产实例前确认业务影响。
- [ ] 广播执行按节点验收，不用单一总状态替代节点结果。
- [ ] 工单中不粘贴令牌、密钥值或未脱敏环境变量。
