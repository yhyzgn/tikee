# 决策记录

## 2026-05-19 — 开发交接协议

Decision:
- 使用 `prompt.md` 作为跨 AI 智能体开发总提示词。
- 使用 `.memory/` 保存长期项目记忆。
- 使用 `.prompt/` 保存有序阶段提示词。

Rationale:
- 项目周期长，可能由 Codex、Claude、Gemini、OpenCode 等不同智能体接手。
- 需要保证上下文、验证证据、下一步任务和提交状态持续可追溯。

Constraint:
- 每次推进后必须更新记忆库和后续阶段 prompt。
