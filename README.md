bscore-lib
==========

bowling score library for C (written in Rust)


build
-----

```
cargo build --release
```

output example (dll on Windows)
- target/release/bscore_lib.dll
- target/release/bscore_lib.dll.lib


sample
------

```C
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

/*
only 300
  x   x   x   x   x   x   x   x   x xxx
 30  60  90 120 150 180 210 240 270 300
300
ext 300 false
  x   x   x   x   x   x   x   x   x xxx
 30  60  90 120 150 180 210 240 270 300
300
ext 300 true
  x   x   x   x   x   x   x   x   x xxx
 30  60  90 120 150 180 210 240 270 300
300
  x   x   x   x   x   x   x   x   x xxx
 30  60  90 120 150 180 210 240 270 300
300
  x   x   x   x   x   x   x   x   x xxx
 30  60  90 120 150 180 210 240 270 300
300
more false
 9/   x   x   x   x   x   x   x   x xxx
 20  50  80 110 140 170 200 230 260 290
290
more true
 9/   x   x   x   x   x   x   x   x xxx
 20  50  80 110 140 170 200 230 260 290
290
  x   x   x   x   x   x   x   x   x xxx
 30  60  90 120 150 180 210 240 270 300
300
  x   x   x   x   x   x   x   x   x xxx
 30  60  90 120 150 180 210 240 270 300
300
  x   x   x   x   x   x   x   x   x xx9
 30  60  90 120 150 180 210 240 270 299
299
  x   x   x   x   x   x   x   x   x x9/
 30  60  90 120 150 180 210 240 269 289
289
  x   x   x   x   x   x   x   x   x 9/x
 30  60  90 120 150 180 210 239 259 279
279
  x   x   x   x   x   x   x   x  9/ xxx
 30  60  90 120 150 180 209 229 249 279
279
*/
```


links
-----

library

- crate [https://crates.io/crates/bscore](https://crates.io/crates/bscore)
- repository [https://github.com/nomissbowling/bscore-rs](https://github.com/nomissbowling/bscore-rs)
- docs [https://docs.rs/bscore/latest/bscore/](https://docs.rs/bscore/latest/bscore/)

cdylib

- crate [https://crates.io/crates/bscore-lib](https://crates.io/crates/bscore-lib)
- repository [https://github.com/nomissbowling/bscore-lib](https://github.com/nomissbowling/bscore-lib)
- docs [https://docs.rs/bscore-lib/latest/bscore_lib/](https://docs.rs/bscore-lib/latest/bscore_lib/)

app sample

- [https://crates.io/crates/bowling_score](https://crates.io/crates/bowling_score)
- [https://github.com/nomissbowling/bowling_score](https://github.com/nomissbowling/bowling_score)

for python

- [https://github.com/nomissbowling/bscore](https://github.com/nomissbowling/bscore)


License
-------

MIT
