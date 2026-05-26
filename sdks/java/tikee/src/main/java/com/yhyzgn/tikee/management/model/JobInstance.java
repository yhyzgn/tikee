package com.yhyzgn.tikee.management.model;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;

/** Job instance returned after a trigger call. */
@JsonIgnoreProperties(ignoreUnknown = true)
public record JobInstance(
        String id,
        @JsonProperty("job_id") String jobId,
        String status,
        @JsonProperty("trigger_type") String triggerType,
        @JsonProperty("execution_mode") String executionMode,
        @JsonProperty("created_at") String createdAt,
        @JsonProperty("updated_at") String updatedAt) {}
