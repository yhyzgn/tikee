package com.yhyzgn.tikee.management.client;

/** Runtime exception raised by the tikee management client. */
public class TikeeManagementException extends RuntimeException {
    public TikeeManagementException(String message) {
        super(message);
    }

    public TikeeManagementException(String message, Throwable cause) {
        super(message, cause);
    }
}
