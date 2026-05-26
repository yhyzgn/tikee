package com.yhyzgn.tikee.management.model;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;

/** Job definition returned by tikee management APIs. */
@JsonIgnoreProperties(ignoreUnknown = true)
public record JobDefinition(
        String id,
        String namespace,
        String app,
        String name,
        @JsonProperty("schedule_type") String scheduleType,
        @JsonProperty("schedule_expr") String scheduleExpr,
        @JsonProperty("processor_name") String processorName,
        boolean enabled) {}
