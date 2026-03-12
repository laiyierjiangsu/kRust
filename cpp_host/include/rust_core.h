#pragma once

#ifdef __cplusplus
extern "C" {
#endif

__declspec(dllimport) int rust_add(int a, int b);
__declspec(dllimport) int rust_sub(int a, int b);
__declspec(dllimport) int rust_mul(int a, int b);

#ifdef __cplusplus
}
#endif
