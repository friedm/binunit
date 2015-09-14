#include "../../csrc/binunit_runtime.h"

///[test]
void test_segf() {
   int y = 0;
   int x = 4/y;
}

///[test]
void test_2_segf() {
   int x = *(int *)0x1234;
}
