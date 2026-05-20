# 下一步

## 当前建议阶段

执行 `.prompt/016-dynamic-script-sandbox.md`。

## 目标

进入多语言动态脚本与安全沙箱最小切片。脚本定义由 Server 管理，实际执行必须在 Worker 侧受控沙箱中完成，并为多语言 runtime 预留扩展接口。

## 开始前检查

- 先确认 014-worker-capability-routing 已提交并推送。
- 确认设计文档中提及的安全边界与权限分离约束。
- 接口需要严格遵循 `{code,message,data}` 规范。
- 阅读 `design/auth-session-design.md`，保持 SessionStore 抽象不被后续安全模块破坏。
- Server 不得执行用户脚本；Worker 侧执行必须受 SandboxPolicy 约束。
- 完成后更新 `.memory/*`、`design/scheduler-architecture-design.md`、新增 `.prompt/017-*.md`。
