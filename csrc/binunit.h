char current_test_failure_reason[1000];
char current_test_failed;

void binunit_run_test_with_label(char *label);
void binunit_run_test(void (*fn)(), char *name);
