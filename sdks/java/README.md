# scheduler Java SDKs

Java SDK packages live under `sdks/java/<sdk-name>/`. This language directory is a Gradle multi-project aggregator; each included SDK module can also be built/tested independently by Gradle task path.

Current packages:

- `scheduler-java-core/`
- `scheduler-spring-boot-autoconfigure/`
- `scheduler-spring-boot-starter/`

Java SDK uses Gradle and requires JDK 21+. Maven `pom.xml` is intentionally not used.

Registration model: Java workers must treat scheduler-assigned `worker_id` as authoritative. Starter configuration exposes only `scheduler.worker.client-instance-id` as an optional stable hint; future heartbeat/log/result calls must use the id returned by the server registration ack.

Validation from repository root:

```bash
./sdks/java/gradlew -p sdks/java test
./sdks/java/gradlew -p sdks/java :scheduler-java-core:test
./sdks/java/gradlew -p sdks/java :scheduler-spring-boot-autoconfigure:test
./sdks/java/gradlew -p sdks/java :scheduler-spring-boot-starter:test
```
