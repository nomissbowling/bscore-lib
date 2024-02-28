#ifndef _BSCORE_LIB_H_
#define _BSCORE_LIB_H_

#ifdef __cplusplus
extern "C" {
#endif

int bscore_s(const char *src, size_t len, bool mode, char *dst, size_t *size);

#ifdef __cplusplus
}
#endif

#endif // _BSCORE_LIB_H_
