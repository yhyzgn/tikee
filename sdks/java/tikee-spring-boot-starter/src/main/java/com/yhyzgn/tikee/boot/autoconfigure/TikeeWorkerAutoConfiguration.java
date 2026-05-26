package com.yhyzgn.tikee.boot.autoconfigure;

import com.yhyzgn.tikee.boot.lifecycle.TikeeWorkerLifecycle;
import com.yhyzgn.tikee.management.client.HttpTikeeJobClient;
import com.yhyzgn.tikee.management.client.TikeeJobClient;
import com.yhyzgn.tikee.worker.identity.ClientInstanceIds;
import com.yhyzgn.tikee.worker.client.GrpcTikeeWorkerClient;
import com.yhyzgn.tikee.worker.client.NoopTikeeWorkerClient;
import com.yhyzgn.tikee.worker.client.TikeeWorkerClient;
import com.yhyzgn.tikee.worker.WorkerRegistration;
import com.yhyzgn.tikee.spring.processor.TikeeProcessorRegistry;
import com.yhyzgn.tikee.spring.worker.SpringTikeeTaskProcessor;
import java.time.Duration;
import java.util.ArrayList;
import java.util.LinkedHashSet;
import java.util.List;
import org.springframework.boot.autoconfigure.AutoConfiguration;
import org.springframework.boot.autoconfigure.condition.ConditionalOnMissingBean;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.boot.context.properties.EnableConfigurationProperties;
import org.springframework.context.annotation.Bean;

/**
 * Auto-configuration for the tikee Spring Boot Starter.
 */
@AutoConfiguration
@EnableConfigurationProperties({TikeeWorkerProperties.class, TikeeManagementProperties.class})
public class TikeeWorkerAutoConfiguration {
    @Bean
    @ConditionalOnMissingBean
    @ConditionalOnProperty(prefix = "tikee.worker", name = "enabled", havingValue = "true", matchIfMissing = true)
    TikeeWorkerClient tikeeWorkerClient(
            TikeeWorkerProperties properties, TikeeProcessorRegistry processorRegistry) {
        String clientInstanceId = properties.getStateDir() == null || properties.getStateDir().isBlank()
                ? ClientInstanceIds.resolve(
                        properties.getClientInstanceId(),
                        properties.getNamespace(),
                        properties.getApp(),
                        properties.getCluster(),
                        properties.getRegion())
                : ClientInstanceIds.resolve(
                        properties.getClientInstanceId(),
                        properties.getNamespace(),
                        properties.getApp(),
                        properties.getCluster(),
                        properties.getRegion(),
                        java.nio.file.Path.of(properties.getStateDir()));
        var registration = new WorkerRegistration(
                clientInstanceId,
                properties.getNamespace(),
                properties.getApp(),
                properties.getCluster(),
                properties.getRegion(),
                workerCapabilities(properties, processorRegistry),
                properties.getLabels());
        if (properties.isDryRun()) {
            return new NoopTikeeWorkerClient(registration);
        }
        return new GrpcTikeeWorkerClient(
                properties.getEndpoint(),
                registration,
                new SpringTikeeTaskProcessor(processorRegistry),
                Duration.ofMillis(properties.getHeartbeatIntervalMillis()));
    }

    @Bean
    @ConditionalOnMissingBean
    @ConditionalOnProperty(prefix = "tikee.worker", name = "enabled", havingValue = "true", matchIfMissing = true)
    TikeeWorkerLifecycle tikeeWorkerLifecycle(TikeeWorkerClient client, TikeeWorkerProperties properties) {
        return new TikeeWorkerLifecycle(client, properties);
    }

    private static List<String> workerCapabilities(
            TikeeWorkerProperties properties, TikeeProcessorRegistry processorRegistry) {
        var capabilities = new LinkedHashSet<String>();
        capabilities.addAll(properties.getCapabilities());
        capabilities.addAll(processorRegistry.processorCapabilities());
        return new ArrayList<>(capabilities);
    }

    @Bean
    @ConditionalOnMissingBean
    @ConditionalOnProperty(prefix = "tikee.management", name = "enabled", havingValue = "true")
    TikeeJobClient tikeeJobClient(TikeeManagementProperties properties) {
        return new HttpTikeeJobClient(
                properties.getEndpoint(),
                properties.getToken(),
                properties.getNamespace(),
                properties.getApp());
    }

    @Bean
    @ConditionalOnMissingBean
    static TikeeProcessorRegistry tikeeProcessorRegistry() {
        return new TikeeProcessorRegistry();
    }
}
