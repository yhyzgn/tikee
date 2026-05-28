package com.yhyzgn.tikee.examples.worker.processor;

import com.yhyzgn.tikee.processor.TikeeProcessor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;

/** Demo plugin-backed processor type. Worker advertises plugin-processor:sql capability. */
@Slf4j
@Component
public final class SqlPluginTaskProcessor {
    @TikeeProcessor("billing.sql-sync")
    public String run(String payload) {
        log.info("[billing.sql-sync] plugin SQL processor received payload='{}'", payload);
        return "sql-plugin-ok:" + payload;
    }
}
