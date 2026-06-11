# Dashboard 运维手册

## 概览

Dashboard 是登录控制台后的运行总览页，用来快速判断任务、实例和 Worker 是否健康。它只展示平台当前状态，不负责创建、修改、重试或取消资源。

运维依据：页面由 `web/src/pages/Dashboard.tsx` 提供；主要读取 `/api/v1/jobs`、`/api/v1/workers`、`/api/v1/jobs/{job}/instances`，并监听 `/api/v1/instances/stream` 与 `/api/v1/workers/stream`。排障时也可以对照 `/api/v1/metrics/summary` 和 `/api/v1/cluster`。

## 前置条件

- 已登录控制台，并具备查看 Jobs、Instances、Workers 的权限。
- Management API 可访问，控制台和后端使用同一环境。
- 至少有一个 Job 或 Worker，空环境只能验证页面可打开，不能判断调度健康。
- 如需用命令核对接口，准备好本地令牌变量，不要把令牌写进文档或工单正文。

```bash
curl -fsS http://127.0.0.1:9090/api/v1/workers \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data.online'
```

## 打开页面

1. 登录 Tikeo 控制台。
2. 在左侧菜单选择 **总览**，或直接打开 `/dashboard`。
3. 等待任务、实例和 Worker 卡片完成刷新。
4. 如果数字长时间不变，刷新页面后再确认浏览器网络面板里的 stream 和 API 响应。

## 常见操作

### 判断 Worker 是否在线

先看在线 Worker 数。为 `0` 时，进入 Workers 页面检查出站 Tunnel 会话、生命周期事件和 Worker 注册信息。

### 判断任务是否积压

查看 pending 实例数。如果 pending 持续增长：

1. 打开 Jobs 页面确认 Job 已启用。
2. 检查 Job 的 processor、namespace、app、worker pool、标签或广播选择条件。
3. 打开 Workers 页面确认至少一个 Worker 声明了匹配能力。
4. 打开 Instances 页面查看实例是否已经进入 failed 或 partial_failed。

### 判断广播执行是否异常

广播实例数升高时，进入 Instances 页面查看每个执行节点的状态和日志。部分 Worker 成功、部分失败时，不要只看总览数字，应以实例详情和日志为准。

## 验收

- `/dashboard` 可以打开，且卡片能显示任务总数、启用任务、pending 实例、在线 Worker 和广播实例。
- Worker 上线或下线后，Dashboard 能通过 `/api/v1/workers/stream` 或刷新后的 `/api/v1/workers` 反映变化。
- 新实例创建或状态变化后，Dashboard 能通过 `/api/v1/instances/stream` 或刷新后的实例接口反映变化。
- Dashboard 不出现创建、编辑、删除、取消等会改变资源状态的入口。

## 故障排查

| 现象 | 处理 |
| --- | --- |
| 在线 Worker 为 0 | 打开 Workers 页面，检查 Worker 是否以出站 Tunnel 连接到 Server。 |
| pending 实例持续增加 | 打开 Jobs 的 scheduling advice，再核对 Worker structured capabilities。 |
| 卡片数字停滞 | 检查 `/api/v1/instances/stream`、`/api/v1/workers/stream` 和普通 API 是否返回。 |
| Dashboard 与列表页数字不一致 | 以 Jobs、Instances、Workers 列表页和 API 响应为准，并刷新 Dashboard。 |
| 控制台无权限 | 联系管理员补齐 read 权限；不要用共享账号绕过。 |

## 生产检查清单

- [ ] Dashboard 只作为只读总览使用，不在这里执行变更。
- [ ] Worker 在线数与 Workers 页面一致。
- [ ] pending 实例有明确归因：无合格 Worker、调度延迟、Job 配置错误或后端异常。
- [ ] stream 断开时，普通 API 仍可用于核对状态。
- [ ] 排障记录只保存资源 ID、trace ID、状态和时间，不保存令牌或明文凭据。
