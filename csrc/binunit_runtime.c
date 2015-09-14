#include "binunit_runtime.h"
#include "binunit.h"

#include <stdio.h>

void assert_fail(void) {
   current_test_failed = 1;
   sprintf(current_test_failure_reason, "assert_fail called");
}

void assert_eq(int expected, int actual) {
   if (expected != actual) {
      current_test_failed = 1;
      sprintf(current_test_failure_reason, "assert_eq failed -- expected %d, got %d\n", expected, actual);
   }
}

void assert(int bool_value) {
   if (!bool_value) {
      current_test_failed = 1;
      sprintf(current_test_failure_reason, "assert failed -- expected nonzero value");
   }
}
