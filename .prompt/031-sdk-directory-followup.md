# 031 — SDK directory follow-up (superseded by 034)

## Updated rule
The previous rule “all SDK packages live directly under `./sdks`” has been refined:

```text
sdks/
├── rust/
├── java/
├── go/
├── python/
└── nodejs/

examples/
├── rust/
├── java/
├── go/
├── python/
└── nodejs/
```

## Current known mismatch
- Rust SDK currently still lives at `sdks/scheduler-worker-sdk`; migrate it to `sdks/rust`.
- Java SDK currently still uses Maven `pom.xml`; migrate it to Gradle multi-project with JDK 21+ support.
- `examples/` demo directories do not yet exist; create them when executing 034.

## Forward path
Use `.prompt/034-sdk-layout-gradle-examples.md` as the active execution plan for SDK layout normalization.
