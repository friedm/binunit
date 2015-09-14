#include "../../csrc/binunit_runtime.h"

///[test]
void test_pass(void) {
}

///[test]
void test_assert_fail_fail(void) {
   assert_fail();
}

///[test]
void test_assert_eq_int_pass(void) {
   int x = 5;
   int y = 10;
   assert_eq(x, y/2);
}

///[test]
void test_assert_eq_int_fail(void) {
   int x = 3;
   int y = 6;
   assert_eq(x, y/3);
}
