#include <stdio.h>
#include <stdlib.h>

#include "punit.h"

void run(void (*fn)(), char *name) {
   if (fn) fn();
   else printf("Could not link %s\n", name);
}

void punit_main(void) {
   run_tests();
   exit(0);
}
