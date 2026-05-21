# 034 — SDK layout normalization, Java Gradle, and examples

## Goal
Normalize SDK and demo layout according to the latest project rule.

## Required target layout
```text
sdks/
├── rust/      # Rust Worker SDK crate (currently sdks/scheduler-worker-sdk; migrate here)
├── java/      # Java 21+ Gradle multi-project Spring Boot Starter SDK
├── go/        # planned Go SDK
├── python/    # planned Python SDK
└── nodejs/    # planned Node.js/TypeScript SDK

examples/
├── rust/      # Rust SDK demo worker
├── java/      # Java Spring Boot demo app, Gradle, JDK21+
├── go/        # Go demo worker
├── python/    # Python demo worker
└── nodejs/    # Node.js demo worker
```

## Must do
1. Move Rust SDK from `sdks/scheduler-worker-sdk` to `sdks/rust`.
   - Update root Cargo workspace, Dockerfile cache stage, README, memory/prompt references.
2. Convert Java SDK from Maven to Gradle.
   - Remove/replace `pom.xml` main build files.
   - Add `settings.gradle.kts` / `build.gradle.kts` multi-project setup.
   - Use Java toolchain 21+.
   - Keep Spring Boot Starter, autoconfigure, annotation scanning modules.
   - Replace validation command with Gradle command.
3. Create `examples/{rust,java,go,python,nodejs}` directories.
   - Add minimal README or runnable skeleton where the corresponding SDK exists.
   - For future SDKs without implementation, add placeholder README explaining planned demo.
4. From now on, when SDK/Worker/workflow integration needs end-to-end debugging, autonomously create or update the relevant `examples/<language>` demo.

## Hard constraints
- `examples/` is for demos only; runtime config stays in `config/`.
- No database foreign keys.
- HTTP envelope remains `{ code, message, data }`.
- Swagger UI is forbidden.
- After changes run full validation: fmt, clippy, cargo test/build, Java Gradle tests, web checks if docs/API references changed as needed, dev script smoke if runtime changed, update design/.memory/.prompt, commit and push.
