# 下一步

## 当前建议阶段

执行 `.prompt/013-broadcast-execution.md`。

## 目标

在保持 Worker 只主动建立 OpenTunnel、不引入 Worker 入站端口的前提下，实现广播执行基础能力：一次触发向多个在线 Worker 下发任务，并能查询每个 Worker 的子执行结果。

## 开始前检查

- 先确认 012-auth-rbac-foundation 已提交并推送。
- 保持 HTTP API envelope：`{code,message,data}`。
- 完成后更新 `.memory/*`、`design/scheduler-architecture-design.md`、新增 `.prompt/014-*.md`。
