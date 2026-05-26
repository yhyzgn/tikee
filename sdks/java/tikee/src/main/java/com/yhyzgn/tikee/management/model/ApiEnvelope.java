package com.yhyzgn.tikee.management.model;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

/** tikee management API response envelope. */
@JsonIgnoreProperties(ignoreUnknown = true)
public record ApiEnvelope<T>(int code, String message, T data) {}
