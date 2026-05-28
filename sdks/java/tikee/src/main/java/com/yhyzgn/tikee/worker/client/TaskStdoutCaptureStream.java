package com.yhyzgn.tikee.worker.client;

import java.io.IOException;
import java.io.OutputStream;
import java.util.Objects;

/** Mirrors processor stdout to the original stream while buffering it for task-log emission. */
final class TaskStdoutCaptureStream extends OutputStream {
    private final OutputStream original;
    private final OutputStream captured;

    TaskStdoutCaptureStream(OutputStream original, OutputStream captured) {
        this.original = Objects.requireNonNull(original, "original");
        this.captured = Objects.requireNonNull(captured, "captured");
    }

    @Override
    public void write(int value) throws IOException {
        original.write(value);
        captured.write(value);
    }

    @Override
    public void write(byte[] buffer, int offset, int length) throws IOException {
        original.write(buffer, offset, length);
        captured.write(buffer, offset, length);
    }

    @Override
    public void flush() throws IOException {
        original.flush();
        captured.flush();
    }
}
