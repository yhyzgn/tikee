# Settings 与治理运维手册

## 概览

Settings 相关能力分布在多个治理页面，而不是一个单独页面。运维人员通常在这里管理用户、角色、tenant scopes、API-Key、service account、调度日历、GitOps/IaC，以及已启用的身份和观测治理入口。

运维依据：路由和菜单元数据由 `web/src/routes.tsx` 提供。相关路径包括 `/users`、`/roles`、`/scopes`、`/api-keys`、`/calendars`、`/gitops`、`/notifications`、`/alerts` 和 `/audit`。菜单显示与操作按钮受 RBAC resource/action 控制。

## 前置条件

- 已登录控制台。
- 具备目标页面的 read 权限；变更用户、角色、scope、API-Key 或 calendar 需要对应 manage/write 权限。
- 已明确变更所属 namespace、app、service account 和授权范围。
- 创建 API-Key 后只在安全位置保存一次，后续不要要求系统再次显示明文。

```bash
curl -fsS http://127.0.0.1:9090/api/v1/jobs \
  -H "x-tikeo-api-key: $TIKEO_MANAGEMENT_API_KEY" | jq '.code'
```

## 打开页面

1. 登录控制台。
2. 在左侧菜单找到 **治理配置** 分组。
3. 按目标选择 **用户管理**、**角色管理**、**租户范围**、**调度日历**、**API-Key** 或 **GitOps/IaC**。
4. 如果菜单项不可见，先检查账号角色和 route metadata 要求的 resource/action。

## 常见操作

### 管理用户和角色

1. 在 `/users` 查看用户和角色绑定。
2. 在 `/roles` 查看 permission matrix、菜单权限和界面操作元素权限。
3. 修改角色前，确认 read、write、execute、manage 的影响范围。
4. 保存后用普通用户、operator 和 admin 三种视角验收。

### 管理 tenant scopes

1. 在 `/scopes` 管理 namespace、app、worker pool 和受控引用。
2. Job ownership、service accounts、Worker pools、密钥引用和 canary targets 都依赖 scope。
3. 跨 scope 移动 Job 前，确认源 scope 与目标 scope 均授权。

### 管理 API-Key 和 service account

1. 在 `/api-keys` 创建 service account。
2. 设置 namespace、app 和允许的 scopes。
3. 创建 API-Key 后立即复制并保存到安全系统。
4. 发生泄露、离职或服务下线时，执行 rotate 或 revoke。
5. 后续页面只应显示 prefix，不应显示完整值。

### 管理调度日历和 GitOps/IaC

在 `/calendars` 维护调度窗口和假期策略；在 `/gitops` 管理配置交付入口。生产变更前，确认影响的 namespace/app 和关联 Job。

## 验收

- 普通用户只能看到被授权菜单和按钮。
- operator 能执行日常运维操作，但不能越权管理高风险资源。
- admin 能看到治理配置，并能管理 RBAC、scope 和 API-Key。
- API-Key 创建后只展示一次完整值，后续只展示 prefix。
- revoke 或 rotate 后，旧凭据不能继续调用 Management API。

## 故障排查

| 现象 | 处理 |
| --- | --- |
| 菜单不可见 | 检查 `web/src/routes.tsx` 中该 route 的 permission，再核对用户角色。 |
| 按钮不可见 | 检查界面操作元素权限和后端 permission catalog。 |
| API-Key 调用失败 | 检查 key 是否 revoked、service account scope 是否正确、请求头是否为 `x-tikeo-api-key`。 |
| 跨 scope 操作失败 | 确认源 scope 和目标 scope 都已授权。 |
| 普通用户看到高风险入口 | 立即回滚角色变更，并检查菜单权限和后端权限是否一致。 |

## 生产检查清单

- [ ] 所有治理变更有变更单或审批记录。
- [ ] RBAC 同时覆盖菜单、按钮和后端接口。
- [ ] API-Key 只保存到授权的安全系统，不写入文档、日志或工单正文。
- [ ] service account scope 遵循最小权限。
- [ ] rotate、revoke、角色变更后已用实际账号验收。
