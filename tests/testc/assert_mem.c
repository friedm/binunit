#include "../../csrc/binunit_runtime.h"

///[test]
void test_assert_mem_eq_pass(void) {
   int a[6] = {1,2,3,4,5,6};
   int b[6] = {1,2,3,4,5,6};

   assert_mem_eq(a, b, 0);
   assert_mem_eq(a, b, 6);
}

///[test]
void test_assert_mem_eq_fail(void) {
   int a[6] = {1,2,3,4,5,6};
   int b[6] = {6,5,4,3,2,1};

   assert_mem_eq(a, b, 6);
}

///[test]
void test_assert_mem_eq_pass_2(void) {
   int a[6] = {1,2,3,4,5,6};
   int b[6] = {6,5,4,3,2,1};

   assert_mem_eq(a, b, 0);
}
