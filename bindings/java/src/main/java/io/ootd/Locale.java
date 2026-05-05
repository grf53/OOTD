package io.ootd;

public enum Locale {
    EN("en"),
    KO("ko");

    private final String code;

    Locale(String code) {
        this.code = code;
    }

    public String code() {
        return code;
    }
}
