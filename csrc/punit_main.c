#include <stdio.h>
#include <stdlib.h>

#include "punit.h"

void punit_run_test(void (*fn)(), char *name) {
   if (fn) fn();
   else printf("%s: could not link\n", name);
}

void punit_main(void) {
   punit_run_tests();
   exit(0);
}
