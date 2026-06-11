# Jobs 运维手册

## 概览

Jobs 页面用于管理任务定义、调度方式、执行绑定、版本、回滚、手动触发、广播触发和容量建议。运维人员应把 Job 看作调度契约：它描述要运行什么、在哪个 namespace/app 下运行、由哪些 Worker 能力承接。

运维依据：页面由 `web/src/pages/JobsPage.tsx` 提供；主要接口包括 `/api/v1/jobs`、`/api/v1/jobs/{job}`、`/api/v1/jobs/{job}:trigger`、`/api/v1/jobs/{job}/versions`、`/api/v1/jobs/{job}/rollback`、`/api/v1/jobs/{job}/scheduling-advice`、`/api/v1/jobs/topology` 和 `/api/v1/jobs/{job}/impact`。

## 前置条件

- 具备 `jobs:read` 查看权限；创建、编辑、删除或回滚需要 `jobs:write`。
- 手动触发实例需要 `instances:execute`。
- 已准备 namespace、app、worker pool、calendar、processor、script 或 plugin processor。
- Worker 必须已经声明匹配的 structured capabilities；Job 名称不能替代能力声明。

```bash
curl -fsS http://127.0.0.1:9090/api/v1/jobs \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data.items[] | {id,name,namespace,app,enabled}'
```

## 打开页面

1. 登录控制台。
2. 在左侧菜单选择 **任务**，或打开 `/jobs`。
3. 使用搜索框按任务名、namespace 或 app 过滤。
4. 如需查看依赖关系，点击 **任务拓扑**，对应路径为 `/jobs/topology`。

## 常见操作

### 创建 Job

1. 点击 **新建任务**。
2. 先选择 namespace 和 app。
3. 选择调度类型：API 手动触发、Cron、固定频率、固定延迟、一次性未来任务或 Daily Time Interval。
4. 选择执行方式：SDK processor、插件处理器或已发布脚本。
5. 配置 retry policy、calendar、worker pool、canary target 和 misfire 策略。
6. 保存后打开 scheduling advice，确认 eligible workers 不为空。

### 编辑 Job

1. 在任务列表中选择编辑。
2. 修改 scope 时，同时确认源 scope 和目标 scope 都已授权。
3. 修改 processor、script 或 plugin processor 后，重新检查调度建议。
4. 保存后记录新版本号，必要时在版本列表中确认变更。

### 手动触发单实例

1. 确认 Job 已启用。
2. 打开任务操作中的触发入口。
3. 默认触发契约为 `triggerType=api` 与 `executionMode=single`。
4. 触发后进入 Instances 页面查看 `/api/v1/instances/{instance}` 和 `/api/v1/instances/{instance}/logs`。

### 广播触发

1. 打开广播抽屉。
2. 明确填写 `broadcastSelector`，例如 tags、region、cluster 或 labels。
3. 只选择 Worker 已真实声明的条件。
4. 触发后在 Instances 页面逐个检查执行节点，不能只看总状态。

### 回滚版本

1. 打开版本历史。
2. 对照版本号、创建人和创建时间。
3. 执行 rollback 后重新查看当前 Job 定义和调度建议。
4. 如 Job 已被定时触发，按生产变更流程记录回滚时间。

## 验收

- 可以创建一个 API 手动触发 Job，并通过 `/api/v1/jobs/{job}:trigger` 生成实例。
- 可以编辑 Job，并在版本列表中看到新版本。
- 可以对一个已知版本执行 rollback，当前定义回到目标版本。
- scheduling advice 能显示 eligible workers、近期实例和失败窗口。
- 广播 Job 必须携带 `broadcastSelector`，并在实例详情中看到广播执行结果。

## 故障排查

| 现象 | 处理 |
| --- | --- |
| 保存失败 | 检查必填字段、namespace/app 授权、calendar 和 canary target。 |
| eligible workers 为空 | 到 Workers 页面核对 structured capabilities、worker pool、namespace、app、labels。 |
| 手动触发失败 | 确认用户具备 `instances:execute`，并检查 Job 是否启用。 |
| 实例一直 pending | 对照 scheduling advice 和 Worker 在线状态，确认没有过窄的 selector。 |
| 回滚后行为仍异常 | 打开版本历史和 Instances 日志，确认当前运行实例是否仍来自回滚前版本。 |

## 生产检查清单

- [ ] Job scope、processor binding、worker pool 和 calendar 已复核。
- [ ] 所有广播触发都显式使用 `broadcastSelector`。
- [ ] 触发前已确认至少一个 eligible worker。
- [ ] 回滚、删除、禁用等变更有审批或工单记录。
- [ ] 排障命令只使用环境变量传入令牌，不记录明文凭据。
