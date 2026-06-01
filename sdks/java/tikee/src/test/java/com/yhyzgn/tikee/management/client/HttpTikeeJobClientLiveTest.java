package com.yhyzgn.tikee.management.client;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

import com.yhyzgn.tikee.management.model.CreateJobRequest;
import com.yhyzgn.tikee.management.model.TriggerJobRequest;
import java.time.Instant;
import org.junit.jupiter.api.Test;

class HttpTikeeJobClientLiveTest {

    @Test
    void usesLiveSdkApiKeyAndRejectsOutOfScopeApp() {
        String endpoint = env("TIKEE_LIVE_MANAGEMENT_ENDPOINT");
        String apiKey = env("TIKEE_LIVE_MANAGEMENT_API_KEY");
        assumeTrue(!endpoint.isBlank(), "TIKEE_LIVE_MANAGEMENT_ENDPOINT is required for live smoke");
        assumeTrue(!apiKey.isBlank(), "TIKEE_LIVE_MANAGEMENT_API_KEY is required for live smoke");

        String namespace = envOrDefault("TIKEE_LIVE_MANAGEMENT_NAMESPACE", "default");
        String app = envOrDefault("TIKEE_LIVE_MANAGEMENT_APP", "default");
        String otherApp = envOrDefault("TIKEE_LIVE_MANAGEMENT_OTHER_APP", "other");
        String jobName = "java-live-" + Instant.now().toEpochMilli();

        TikeeJobClient client = new HttpTikeeJobClient(endpoint, apiKey, namespace, app);
        var created = client.createJob(CreateJobRequest.api(jobName, "demo.echo"));
        try {
            assertEquals(namespace, created.namespace());
            assertEquals(app, created.app());
            assertEquals(jobName, created.name());

            assertTrue(client.listJobs().stream().anyMatch(job -> created.id().equals(job.id())));

            var triggered = client.triggerJob(created.id(), TriggerJobRequest.api());
            assertEquals(created.id(), triggered.jobId());
            assertEquals("api", triggered.triggerType());

            TikeeJobClient outOfScope = new HttpTikeeJobClient(endpoint, apiKey, namespace, otherApp);
            assertThrows(
                    TikeeManagementException.class,
                    () -> outOfScope.createJob(CreateJobRequest.api(jobName + "-blocked", "demo.echo")));
        } finally {
            client.deleteJob(created.id());
        }
    }

    private static String env(String name) {
        return System.getenv(name) == null ? "" : System.getenv(name).trim();
    }

    private static String envOrDefault(String name, String fallback) {
        String value = env(name);
        return value.isBlank() ? fallback : value;
    }
}
