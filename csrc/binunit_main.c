#include <stdio.h>
#include <stdlib.h>

#include "binunit_runtime.h"
#include "binunit.h"

void binunit_run_test(void (*fn)(), char *name) {
   if (fn) { 
      current_test_failed = 0;
      sprintf(current_test_failure_reason, "test passed");

      fn();

      if (!current_test_failed) {
         printf("%s: ok\n", name);
      } else {
         printf("%s: failed\n\t%s\n", name, current_test_failure_reason);
      }
   } else printf("%s: could not link\n", name);
}

void main(int argc, char **argv) {
   binunit_run_test_with_label(argv[1]);
   exit(0);
}
