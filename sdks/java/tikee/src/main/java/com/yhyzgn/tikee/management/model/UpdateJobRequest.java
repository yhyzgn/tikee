package com.yhyzgn.tikee.management.model;

import com.fasterxml.jackson.annotation.JsonInclude;

/** Request to update a job. Omitted fields are unchanged. */
@JsonInclude(JsonInclude.Include.NON_NULL)
public record UpdateJobRequest(
        String name,
        String scheduleType,
        String scheduleExpr,
        String processorType,
        String processorName,
        String scriptId,
        Boolean enabled) {
    public static UpdateJobRequest disable() {
        return new UpdateJobRequest(null, null, null, null, null, null, false);
    }

    public static UpdateJobRequest enable() {
        return new UpdateJobRequest(null, null, null, null, null, null, true);
    }

    public static UpdateJobRequest apiPlugin(String name, String processorType, String processorName) {
        return new UpdateJobRequest(name, JobScheduleType.API.value(), null, processorType, processorName, null, true);
    }

    public static UpdateJobRequest cronPlugin(
            String name,
            String scheduleExpr,
            String processorType,
            String processorName) {
        return new UpdateJobRequest(name, JobScheduleType.CRON.value(), scheduleExpr, processorType, processorName, null, true);
    }
}
