#include <iostream>

#include "rust_core.h"

int main() {
    const int a = 6;
    const int b = 4;

    std::cout << "rust_add(" << a << ", " << b << ") = " << rust_add(a, b) << std::endl;
    std::cout << "rust_sub(" << a << ", " << b << ") = " << rust_sub(a, b) << std::endl;
    std::cout << "rust_mul(" << a << ", " << b << ") = " << rust_mul(a, b) << std::endl;

    return 0;
}
