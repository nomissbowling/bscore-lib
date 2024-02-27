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
  char dst[sizeof(src) * 8];
  bscore_s(src, strlen(src), 0, dst, sizeof(dst));
  printf("%s", dst);
  return 0;
}
