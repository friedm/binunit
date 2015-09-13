#include <stdio.h>
#include <stdlib.h>

#include "binunit_runtime.h"
#include "binunit.h"

void binunit_run_test(void (*fn)(), char *name) {
   if (fn) { 
      fn();
      if (!current_test_failed) {
         printf("%s: ok\n", name);
      } else {
         printf("%s: failed\n", name);
      }
   } else printf("%s: could not link\n", name);
}

void binunit_main(void) {
   binunit_run_tests();
   exit(0);
}
