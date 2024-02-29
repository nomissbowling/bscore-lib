/*
  bowling_score.cpp

  cpp sample uses dll on Windows
  cl -I../include bowling_score.cpp -link -libpath:../target/release bscore_lib.dll.lib
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <bscore_lib.h>

int disp_score(char *src, size_t len, bool mode)
{
  size_t sz = 0;
  bscore_s(src, len, mode, NULL, &sz);
  char *dst = (char *)malloc(sz);
  if(!dst) return -1;
  bscore_s(src, len, mode, dst, &sz);
  printf("%s", dst);
  free(dst);
  return 0;
}

int main(int ac, char **av)
{
  char src_300[] = "xxxxxxxxxxxx"; // 12x
  printf("only 300\n");
  disp_score(src_300, strlen(src_300), false);
  char src_ext[] = "xxxxxxxxxxxxxx"; // 12x + extra 2x
  printf("ext 300 false\n");
  disp_score(src_ext, strlen(src_ext), false);
  printf("ext 300 true\n");
  disp_score(src_ext, strlen(src_ext), true);
  char src_more[] = "9/xxxxxxxxxxxxx9/xxx"; // more
  printf("more false\n");
  disp_score(src_more, strlen(src_more), false);
  printf("more true\n");
  disp_score(src_more, strlen(src_more), true);
  return 0;
}
