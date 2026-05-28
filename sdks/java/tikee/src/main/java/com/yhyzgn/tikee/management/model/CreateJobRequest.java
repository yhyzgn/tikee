package com.yhyzgn.tikee.management.model;

import com.fasterxml.jackson.annotation.JsonInclude;

/** Request to create a job in the current namespace/app scope. */
@JsonInclude(JsonInclude.Include.NON_NULL)
public record CreateJobRequest(
        String name,
        String scheduleType,
        String scheduleExpr,
        String processorType,
        String processorName,
        String scriptId,
        Boolean enabled) {
    public static CreateJobRequest api(String name, String processorName) {
        return new CreateJobRequest(name, JobScheduleType.API.value(), null, null, processorName, null, true);
    }

    public static CreateJobRequest apiScript(String name, String scriptId) {
        return new CreateJobRequest(name, JobScheduleType.API.value(), null, null, null, scriptId, true);
    }

    public static CreateJobRequest apiPlugin(String name, String processorType, String processorName) {
        return new CreateJobRequest(name, JobScheduleType.API.value(), null, processorType, processorName, null, true);
    }

    public static CreateJobRequest cronPlugin(
            String name,
            String scheduleExpr,
            String processorType,
            String processorName) {
        return new CreateJobRequest(name, JobScheduleType.CRON.value(), scheduleExpr, processorType, processorName, null, true);
    }
}
