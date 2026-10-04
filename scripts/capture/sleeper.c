/* h-capture self-test target: a tiny 32-bit program that idles so the
 * capture tool can read it. Our own code, written for this test. */
#include <windows.h>

static volatile unsigned long long counter = 0;

int main(void) {
    /* Idle ~90 s. Touch a global each second so .data is genuinely live. */
    for (int i = 0; i < 90; i++) {
        counter += (unsigned long long)(i + 1);
        Sleep(1000);
    }
    return (int)(counter & 1);
}
