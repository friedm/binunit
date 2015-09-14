#include "../../csrc/binunit_runtime.h"

#define TRUE 1
#define FALSE 0

///[test]
void test_assert_pass(void) {
   assert(TRUE);
}

///[test]
void test_assert_fail(void) {
   assert(FALSE);
}
