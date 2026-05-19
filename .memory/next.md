# 下一步

## 当前建议阶段

执行 `.prompt/015-dynamic-script-sandbox.md`。

## 目标

在 Phase 014 已完成的 Worker namespace/app 基础路由上，进一步推进 Worker 的执行能力，实现多语言动态脚本（例如 Shell, Python 等）的受控执行与基础沙箱策略。

## 开始前检查

- 先确认 014-worker-capability-routing 已提交并推送。
- 遵循 design 文档关于安全沙箱与动态脚本的设计。
- Worker 需要声明支持的 script 能力并在 Server 侧可见。
- HTTP 接口和 UI 要配合展示对应的执行记录或脚本提交。
- 完成后更新 `.memory/*`、`design/scheduler-architecture-design.md`、新增 `.prompt/016-*.md`。
