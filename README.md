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

int main(int ac, char **av)
{
  char src[] = "xxxxxxxxxxxx";
  char dst[sizeof(src) * 8];
  bscore_s(src, strlen(src), 0, dst, sizeof(dst));
  printf("%s", dst);
  return 0;
}

/*
  x   x   x   x   x   x   x   x   x xxx
 30  60  90 120 150 180 210 240 270 300
300
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
