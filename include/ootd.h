#pragma once

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OotdDurationInput {
  int64_t seconds;
  bool is_future;
  const char *locale;
} OotdDurationInput;

char *ootd_between_rfc3339(const char *start, const char *end, const char *locale);
char *ootd_between_rfc3339_with_options(const char *start, const char *end, const char *locale, bool use_native_ko_number);
char *ootd_from_duration(OotdDurationInput input);
char *ootd_from_duration_parts(int64_t seconds, bool is_future, const char *locale);
char *ootd_from_duration_parts_with_options(int64_t seconds, bool is_future, const char *locale, bool use_native_ko_number);
void ootd_free_string(char *raw);

#ifdef __cplusplus
}
#endif
