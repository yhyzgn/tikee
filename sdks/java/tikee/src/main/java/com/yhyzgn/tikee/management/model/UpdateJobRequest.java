package com.yhyzgn.tikee.management.model;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;

/** Request to update a job. Omitted fields are unchanged. */
@JsonInclude(JsonInclude.Include.NON_NULL)
public record UpdateJobRequest(
        String name,
        @JsonProperty("schedule_type") String scheduleType,
        @JsonProperty("schedule_expr") String scheduleExpr,
        @JsonProperty("processor_name") String processorName,
        Boolean enabled) {
    public static UpdateJobRequest disable() {
        return new UpdateJobRequest(null, null, null, null, false);
    }

    public static UpdateJobRequest enable() {
        return new UpdateJobRequest(null, null, null, null, true);
    }
}
