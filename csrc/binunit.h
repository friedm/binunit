char *current_test_failure_reason;
char current_test_failed;

void binunit_run_tests(void);
void binunit_run_test(void (*fn)(), char *name);
