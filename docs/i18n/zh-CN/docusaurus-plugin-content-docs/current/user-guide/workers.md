# Workers 运维手册

## 概览

Workers 页面用于查看执行资源是否在线（Worker Tunnel 出站连接状态）、是否具备承接任务的能力，以及最近的生命周期事件。Worker 通过出站 Tunnel 主动连接 Server；Server 不要求业务 Worker 暴露入站端口。判断任务能否派发时，先检查这里。

运维依据：页面由 `web/src/pages/WorkersPage.tsx` 提供；主要接口包括 `/api/v1/workers`、`/api/v1/workers/history`、`/api/v1/workers/stream` 和 `/api/v1/dispatch-queue`。执行通道对应 `WorkerTunnelService`、`OpenTunnel`、`RegisterWorker`、`Heartbeat`、`DispatchTask`、`TaskLog`、`TaskResult`、`TaskCheckpoint`。

## 前置条件

- 具备 `workers:read` 权限。
- 至少一个 Worker 已配置 Server 地址并能从 Worker 所在网络出站访问 Server。
- Worker 注册时填写正确的 namespace、app、cluster、region、worker pool 和 capabilities。
- Job 依赖的 SDK processor、script runner、plugin processor 或 labels 已由 Worker 明确声明。

```bash
curl -fsS http://127.0.0.1:9090/api/v1/workers \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data.items[] | {workerId,status,namespace,app,structuredCapabilities}'
```

## 打开页面

1. 登录控制台。
2. 在左侧菜单选择 **Worker 集群**，或打开 `/workers`。
3. 查看集群总览、Worker 表和生命周期历史。
4. 需要排查派发队列时，点击 **查看调度队列**，路径为 `/workers/dispatch-queue`。

## 常见操作

### 确认 Worker 在线

查看 Worker 表中的 status、last heartbeat、generation 和 logical instance。Worker 消失时，先看 lifecycle history，判断是正常重连、被替换、租约过期还是异常断开。

### 核对能力声明

逐项核对 structured capabilities：

- `sdkProcessors` 是否包含 Job 绑定的 processor。
- `scriptRunners` 是否包含脚本语言和 sandbox backend。
- `pluginProcessors` 是否包含插件处理器类型和名称。
- labels、tags、region、cluster 是否满足广播选择条件。

Worker 不应声明自己无法实际执行的 runner 或 processor。

### 排查 pending 实例

1. 在 Jobs 页面查看 scheduling advice。
2. 在 Workers 页面确认至少一个 Worker 匹配 scope 和能力。
3. 在 dispatch queue 中查看队列状态、attempt、worker selector 和 runAfter。
4. 修复 Worker 注册或 Job selector 后，再观察新实例。

## 验收

- Worker 通过出站 Tunnel 上线后，`/api/v1/workers` 显示在线数量增加。
- Worker 断开后，`/api/v1/workers/history` 保留 session 或 event 记录。
- 页面显示的 structured capabilities 与 Worker 实际可执行能力一致。
- 需要某个 processor、script runner 或 label 的 Job，可以在 Worker 表中找到匹配声明。
- 调度队列能帮助解释 pending、running、done 或 failed 状态。

## 故障排查

| 现象 | 处理 |
| --- | --- |
| Worker 不在线 | 检查 Worker 到 Server 的出站网络、Server 地址、认证和时间同步。 |
| Worker 反复重连 | 查看 generation、replacedByWorkerId、statusReason 和 lifecycle events。 |
| Job 无法派发 | 核对 namespace、app、worker pool、processor、labels 和 runner。 |
| 页面有历史但 live list 为空 | 判断是否为重启后的持久化快照；以 live list 判断当前容量。 |
| 广播漏掉 Worker | 检查 broadcast selector 与 Worker labels、tags、region、cluster 是否完全匹配。 |

## 生产检查清单

- [ ] Worker 只需要出站连接，不开放入站执行端口。
- [ ] 每个生产 Worker 有稳定的 namespace、app、pool、cluster 和 region。
- [ ] capabilities 只声明已安装、已配置、可失败关闭的执行能力。
- [ ] Worker 断连告警能关联到 lifecycle history。
- [ ] pending 队列处理前先核对 Worker 能力，不盲目重试实例。
