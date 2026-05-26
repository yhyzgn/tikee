package com.yhyzgn.tikee.management.model;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;

/** Request to create a job in the current namespace/app scope. */
@JsonInclude(JsonInclude.Include.NON_NULL)
public record CreateJobRequest(
        String name,
        @JsonProperty("schedule_type") String scheduleType,
        @JsonProperty("schedule_expr") String scheduleExpr,
        @JsonProperty("processor_name") String processorName,
        Boolean enabled) {
    public static CreateJobRequest api(String name, String processorName) {
        return new CreateJobRequest(name, JobScheduleType.API.value(), null, processorName, true);
    }
}
