#include <stdio.h>
#include <stdlib.h>

#include "binunit.h"

void binunit_run_test(void (*fn)(), char *name) {
   if (fn) fn();
   else printf("%s: could not link\n", name);
}

void binunit_main(void) {
   binunit_run_tests();
   exit(0);
}
