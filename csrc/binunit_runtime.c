#include "binunit_runtime.h"
#include "binunit.h"

#include <stdio.h>
#include <string.h>

void assert_fail(void) {
   current_test_failed = 1;
   sprintf(current_test_failure_reason, "assert_fail called\n");
}

void assert_eq(int expected, int actual) {
   if (expected != actual) {
      current_test_failed = 1;
      sprintf(current_test_failure_reason, "assert_eq failed -- expected \"%d\", got \"%d\"\n", expected, actual);
   }
}

void assert_str_eq(const char *expected, const char *actual) {
   if (0 != strcmp(expected, actual)) {
      current_test_failed = 1;
      sprintf(current_test_failure_reason, "assert_str_eq failed -- expected \"%s\", got \"%s\"\n", expected, actual);
   }
}

void assert_mem_eq(const void *expected, const void *actual, int len) {
   if (0 != memcmp(expected, actual, len)) {
      current_test_failed = 1;
      sprintf(current_test_failure_reason, "assert_mem_eq failed -- %d bytes at \"%p\", \"%p\" not equal\n", len, expected, actual);
   }
}

void assert(int bool_value) {
   if (!bool_value) {
      current_test_failed = 1;
      sprintf(current_test_failure_reason, "assert failed -- expected nonzero value\n");
   }
}
