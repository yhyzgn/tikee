# 015-dynamic-script-sandbox

## 背景

014-worker-capability-routing 已完成：
- 在 Registry 增加了 worker 的 namespace/app 基础路由匹配。
- Dispatcher 和 HTTP API 的 broadcast 触发都已接入路由。
- Web UI 完成了对齐与视觉体验的初步整改。

## 目标

推进 Worker 的执行能力，实现多语言动态脚本（例如 Shell, Python 等）的受控执行与基础沙箱策略，验证系统不直接在 Server 端运行代码。

## 约束

- 脚本和命令必须在 Worker 侧执行。
- 保证安全性，如果可能加入基础资源限制或者策略。
- HTTP 接口和 UI 能够支持查看/提交脚本类型任务。
- 完成后更新 `.memory/*`、`design/scheduler-architecture-design.md`、并新增 `.prompt/016-*.md`。

## 验证要求

- 可以通过 API 或 UI 提交一个 Shell / Python 的 Script 类型的 job。
- 对应的 Worker 能够拉取到脚本代码并成功通过安全环境执行，返回输出日志。
- 确保整个工作流的通过，包括 linting 和 testing。
