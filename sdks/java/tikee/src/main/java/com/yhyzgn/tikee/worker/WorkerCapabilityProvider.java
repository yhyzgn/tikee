package com.yhyzgn.tikee.worker;

/** Provides structured worker capabilities for registration. */
@FunctionalInterface
public interface WorkerCapabilityProvider {
    WorkerCapabilitySet workerCapabilities();
}
