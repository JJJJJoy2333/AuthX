#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <assert.h>
#include "authx.h"

// A simple macro-based test framework
#define TEST_START(name) printf("[TEST] %s... ", name);
#define TEST_PASS() printf("PASS\n")
#define TEST_FAIL(msg) { printf("FAIL: %s\n", msg); return 1; }

int test_get_version() {
    TEST_START("authx_get_version");
    
    char* version = NULL;
    AUTHX_RET ret = authx_get_version(&version);
    
    if (ret != AUTHX_OK) {
        TEST_FAIL("authx_get_version failed to return AUTHX_OK");
    }
    
    if (version == NULL) {
        TEST_FAIL("Version string is NULL");
    }
    
    printf("\n    [INFO] Version: %s\n", version);
    
    // Check for some expected markers
    if (strstr(version, " (") == NULL || strstr(version, ")") == NULL) {
        TEST_FAIL("Version string format incorrect (expected build markers)");
    }
    
    TEST_PASS();
    return 0;
}

int main() {
    printf("--- AuthX C-API Tests ---\n");
    
    int failed = 0;
    
    failed += test_get_version();
    
    if (failed == 0) {
        printf("\nALL TESTS PASSED\n");
    } else {
        printf("\nTESTS FAILED: %d\n", failed);
    }
    
    return failed;
}
