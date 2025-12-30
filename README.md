# libdecor Headers for Rust

This library contains minimalist Rust FFI bindings for libdecor in a way that's roughly equivalent to the official [libdecor.h](https://gitlab.freedesktop.org/libdecor/libdecor/-/blob/0.2.2/src/libdecor.h?ref_type=tags) for C/C++. It makes no attempt at providing safe or idiomatic Rust wrappers and doesn't rename any C identifiers to match Rust's style guidelines.

The following Rust code:

```rust
use libdecor_headers::libdecor::*;
```

is roughly equivalent to the following C code:

```c
#include <libdecor.h>
```

This library is based specifically on libdecor 0.2.2 since that's the version provided by [Steam Runtime 3 'sniper'](https://gitlab.steamos.cloud/steamrt/steamrt/-/blob/steamrt/sniper/README.md).

Using this library does not automatically link against `libdecor-0.so`.
