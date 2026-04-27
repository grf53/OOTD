package io.ootd;

public enum OotdLocale {
    EN("en"),
    KO("ko");

    private final String code;

    OotdLocale(String code) {
        this.code = code;
    }

    public String code() {
        return code;
    }
}
