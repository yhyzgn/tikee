package com.yhyzgn.tikee.management.model;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;

/** Request to trigger a job. */
@JsonInclude(JsonInclude.Include.NON_NULL)
public record TriggerJobRequest(
        @JsonProperty("trigger_type") String triggerType,
        @JsonProperty("execution_mode") String executionMode) {
    public static TriggerJobRequest api() {
        return new TriggerJobRequest(JobTriggerType.API.value(), ExecutionMode.SINGLE.value());
    }
}
