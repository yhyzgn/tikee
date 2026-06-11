# Audit 运维手册

## 概览

Audit 页面用于查看平台写操作、认证事件、脚本治理动作、派发相关事件和失败原因。它是事故复盘、权限核查、发布审阅和合规导出的主要入口。

运维依据：页面由 `web/src/pages/AuditLogsPage.tsx` 提供；主要接口包括 `/api/v1/audit-logs` 和 `/api/v1/audit-logs:export`。导出格式为 JSON，并沿用当前筛选条件。

## 前置条件

- 具备 `audit:read` 权限。
- 已知道调查时间窗口、actor、resource type、resource id、action 或 trace ID 中至少一项。
- 导出文件应按敏感运维记录保存，不放入公开聊天、公开仓库或未授权工单。
- 浏览器允许下载 JSON 文件。

```bash
curl -fsS 'http://127.0.0.1:9090/api/v1/audit-logs?pageSize=20' \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data.items[] | {createdAt,actor,action,result}'
```

## 打开页面

1. 登录控制台。
2. 在左侧菜单选择 **审计日志**，或打开 `/audit`。
3. 设置 actor、action、resource type、resource id、failure reason 或 page size。
4. 复制 URL 可保留当前筛选视图。

## 常见操作

### 过滤成功写操作

选择 resource type 和 action，确认 result 为 success。打开详情后记录 resource id、actor、trace ID、before/after snapshot 和 request identifiers。

### 过滤失败操作

设置 failure reason 或 result 过滤，查看失败行中的 reason。用 trace ID 去关联 API 错误、服务端日志和用户操作时间。

### 核查敏感变更

重点关注 Job scope move、script publication、API-Key rotation、RBAC edits、service-account 变更和 Worker 相关派发事件。使用 before/after snapshot 确认实际变更内容。

### 导出证据

1. 设置好当前筛选条件。
2. 点击导出。
3. 下载 JSON 文件。
4. 核对导出内的 filter metadata 和 `exported` 数量。
5. 按内部安全要求存放导出文件。

## 验收

- 可以过滤至少一个成功写操作。
- 可以过滤至少一个失败操作，并看到 failure reason。
- 带 trace ID 的行可用于关联服务端日志。
- 导出的 JSON 保留当前筛选条件。
- 导出内容包含数量、筛选元数据、before/after 字段和必要请求信息。

## 故障排查

| 现象 | 处理 |
| --- | --- |
| 页面无数据 | 放宽时间或筛选条件，确认当前账号有 `audit:read`。 |
| 找不到失败原因 | 检查服务端是否在失败路径写入 failure reason。 |
| trace ID 无法关联 | 对照 API 网关、应用日志和请求时间，确认日志采集链路。 |
| 导出为空 | 确认导出前的筛选结果非空，并检查 page size 或权限。 |
| before/after 缺失 | 回到对应写操作路径，确认审计写入是否覆盖该资源类型。 |

## 生产检查清单

- [ ] 生产变更至少能在 Audit 中查到 actor、action、resource 和时间。
- [ ] 高风险失败操作必须记录 failure reason。
- [ ] 导出文件只发给授权人员，并按敏感记录保存。
- [ ] 审计调查记录 trace ID、resource ID 和时间窗口。
- [ ] 不用截图替代 JSON 导出；截图只能作为辅助说明。
