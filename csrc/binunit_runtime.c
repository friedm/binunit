#include "binunit_runtime.h"
#include "binunit.h"

void assert_fail(void) {
   current_test_failed = 1;
   current_test_failure_reason = "assert_fail called";
}

void assert_eq(int expected, int actual) {
   if (expected != actual) {
      current_test_failed = 1;
      current_test_failure_reason = "assert_eq failed";
   }
}
