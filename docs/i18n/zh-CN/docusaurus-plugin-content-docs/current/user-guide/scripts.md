# Scripts 运维手册

## 概览

Scripts 页面用于管理脚本草稿、执行策略、发布审批、版本、回滚和 diff 预览。Job 只能绑定已发布并通过审批的脚本版本。Server 负责保存和派发脚本元数据与不可变内容；实际执行发生在 Worker 控制的 sandbox 中。

运维依据：页面由 `web/src/pages/ScriptsPage.tsx` 提供；主要接口包括 `/api/v1/scripts`、`/api/v1/scripts/{id}`、`/api/v1/scripts/{id}/publish`、`/api/v1/scripts/{id}/rollback`、`/api/v1/scripts/{id}/versions` 和 `/api/v1/scripts/{id}/diff`。

## 前置条件

- 具备 `scripts:read` 查看权限；创建、编辑、发布、回滚或删除需要 `scripts:manage`。
- 已确认脚本语言和 sandbox backend 有真实 Worker runner 支持。
- 已准备审批单号或发布说明。
- 脚本策略不得包含明文凭据；需要外部值时使用受控引用或允许运行时环境变量名。

```bash
curl -fsS http://127.0.0.1:9090/api/v1/scripts \
  -H "authorization: Bearer $TIKEO_TOKEN" | jq '.data.items[] | {id,name,language,status,version}'
```

## 打开页面

1. 登录控制台。
2. 在左侧菜单选择 **脚本管理**，或打开 `/scripts`。
3. 点击脚本进入编辑页，路径形如 `/scripts/{id}/edit`。
4. 使用列表筛选语言、状态或关键字。

## 常见操作

### 新建脚本

1. 点击 **新建脚本**。
2. 填写名称、语言、内容和基础资源限制。
3. 配置执行策略：timeout、内存、output limit、环境变量白名单、文件系统、网络、密钥引用和 sandbox backend。
4. 保存草稿。
5. 检查 Worker 是否声明匹配的 `scriptRunners`。

### 预览 diff

发布或保存重大修改前，点击预览变更。重点检查内容 diff 和策略 diff，尤其是网络、文件系统、输出限制、环境变量白名单和 sandbox backend。

### 发布脚本

1. 确认草稿已通过人工检查。
2. 填写审批信息或发布说明。
3. 点击 publish。
4. 发布后生成不可变版本，Job 才能绑定该 approved script。

### 回滚版本

1. 打开 versions。
2. 选择目标版本。
3. 执行 rollback，并填写必要审批信息。
4. 回滚会影响后续调度，应按生产变更处理。

## 验收

- 可以创建草稿并保存策略字段。
- diff 能展示内容变化和 policy_diff。
- publish 能创建 approved 版本。
- versions 能列出版本号、创建人、时间和内容摘要。
- rollback 后当前版本指向目标 approved version。
- Job 绑定脚本前，Workers 页面能看到匹配 script runner。

## 故障排查

| 现象 | 处理 |
| --- | --- |
| 无法保存 | 检查必填字段、语言、资源限制和 `scripts:manage` 权限。 |
| 无法发布 | 检查审批信息、策略限制和后端返回的错误。 |
| Job 绑定后一直 pending | 到 Workers 页面确认 `scriptRunners` 包含语言和 sandbox backend。 |
| 执行失败 | 到 Instances 查看日志；不要在 Server 上寻找脚本执行进程。 |
| 回滚后仍执行旧行为 | 确认 Job 绑定的版本指针和新实例创建时间。 |

## 生产检查清单

- [ ] 默认拒绝网络和文件系统，除非审批明确放开。
- [ ] timeout、内存限制和 output limit 已设置。
- [ ] 环境变量只列白名单名称，不写入明文值。
- [ ] Worker runner 与脚本语言、sandbox backend 匹配。
- [ ] publish 和 rollback 均有审批记录和版本号。
