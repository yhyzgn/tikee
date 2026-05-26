package com.yhyzgn.tikee.examples.worker.web;

import com.yhyzgn.tikee.management.client.HttpTikeeJobClient;
import com.yhyzgn.tikee.management.client.TikeeJobClient;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

/** Optional demo control-plane client configuration. */
@Configuration(proxyBeanMethods = false)
public class DemoManagementConfig {
    @Bean
    @ConfigurationProperties(prefix = "tikee.management")
    DemoManagementProperties demoManagementProperties() {
        return new DemoManagementProperties();
    }

    @Bean
    @ConditionalOnProperty(prefix = "tikee.management", name = "enabled", havingValue = "true")
    TikeeJobClient tikeeJobClient(DemoManagementProperties properties) {
        return new HttpTikeeJobClient(
                properties.getEndpoint(),
                properties.getToken(),
                properties.getNamespace(),
                properties.getApp());
    }

    public static class DemoManagementProperties {
        private boolean enabled = false;
        private String endpoint = "http://127.0.0.1:9999";
        private String token = "";
        private String namespace = "default";
        private String app = "default";

        public boolean isEnabled() {
            return enabled;
        }

        public void setEnabled(boolean enabled) {
            this.enabled = enabled;
        }

        public String getEndpoint() {
            return endpoint;
        }

        public void setEndpoint(String endpoint) {
            this.endpoint = endpoint;
        }

        public String getToken() {
            return token;
        }

        public void setToken(String token) {
            this.token = token;
        }

        public String getNamespace() {
            return namespace;
        }

        public void setNamespace(String namespace) {
            this.namespace = namespace;
        }

        public String getApp() {
            return app;
        }

        public void setApp(String app) {
            this.app = app;
        }
    }
}
