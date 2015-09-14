#include "../../csrc/binunit_runtime.h"

///[test]
void test_assert_eq_string_pass() {
   assert_str_eq("a string!", "a string!");
}

///[test]
void test_assert_eq_string_fail() {
   assert_str_eq("a string!", "another string!");
}
