/*
  bowling_score.cpp

  cpp sample uses dll on Windows
  cl -I../include bowling_score.cpp -link -libpath:../target/release bscore_lib.dll.lib
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <bscore_lib.h>

int main(int ac, char **av)
{
  char src[] = "xxxxxxxxxxxx";
  size_t sz = 0;
  bscore_s(src, strlen(src), 0, NULL, &sz);
  char *dst = (char *)malloc(sz);
  if(!dst) return -1;
  bscore_s(src, strlen(src), 0, dst, &sz);
  printf("%s", dst);
  free(dst);
  return 0;
}
