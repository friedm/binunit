#include "binunit_runtime.h"

void assert_fail(void) {
   current_test_failed = 1;
}
