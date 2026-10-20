#include <stdio.h>
#include <wchar.h>

extern int rs_wcwidth(wchar_t wc);

int main(void) {
    // 1. A typical C0 control character (Backspace)
    wchar_t backspace = L'\b';

    // 2. A null character
    wchar_t null_char = L'\0';

    // 3. A standard printable ASCII character
    wchar_t standard_char = L'A';

    printf("wcwidth('\\b') = %d (Expected: -1)\n", rs_wcwidth(backspace));
    printf("wcwidth('\\0') = %d (Expected:  0)\n", rs_wcwidth(null_char));
    printf("wcwidth('A')  = %d (Expected:  1)\n", rs_wcwidth(standard_char));

    return 0;
}
