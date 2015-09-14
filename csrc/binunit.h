char current_test_failure_reason[1000];
char current_test_failed;

void binunit_run_tests(void);
void binunit_run_test(void (*fn)(), char *name);
