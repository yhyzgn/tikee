# 027-workflow-node-canvas-polish

## 背景
026 阶段已把工作流可视化编辑器从静态预览推进为轻量节点画布：支持节点拖拽、边缘端口 hover 显示、从输出端口按住拖出临时箭头并释放到输入端口连线，并补齐常用节点类型（start/end/job/script/http/condition/parallel/join/delay/approval/notification/map/map_reduce/sub_workflow）。

## 当前约束
- Web：`web/`，React + Ant Design + Bun。
- 后端：Rust workspace；主入口在根 `src/main.rs`，不要放进 `crates/`。
- API 返回必须保持 `{ code, message, data }`。
- 禁止 Swagger；不要新增外键。
- 工作项推进后必须更新 `.memory/`，编译、测试、运行通过后提交并推送。

## 下一阶段建议
1. 继续打磨节点画布 UX：缩放/平移、小地图、框选、多选、对齐线、撤销/重做。
2. 增加节点属性编辑面板：按节点类型编辑 job_id、script/http/condition/delay/approval/notification 等配置。
3. 补齐 workflow update API，让已有工作流可保存更新，而不是只能创建新定义。
4. 明确非执行型节点的运行语义：start/end/condition/parallel/join/delay/approval/notification 当前主要用于定义和可视化，后续需要 runtime materialize/advance 语义。
5. 评估是否引入 React Flow / X6；若新增依赖，必须先说明包体积、维护风险和与现有 Ant Design 风格整合方式。

## 验证基线
继续保持：
- `bun run --cwd web lint`
- `bun run --cwd web typecheck`
- `bun test --cwd web`
- `bun run --cwd web build`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo build --workspace --all-features`
- `./scripts/dev.sh` smoke（可用 `timeout 10 ./scripts/dev.sh` 验证健康检查通过）
