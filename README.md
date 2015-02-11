[![Build Status][status]](https://travis-ci.org/japaric/serial.rs)

# `serial.rs`

A serial port library on top of the termios API

# [Documentation][docs]

# Non-Rust dependencies

- `clang-3.5-dev` and `llvm-3.5-dev` packages to build [bindgen], which is used

  to generate bindings to glibc
- A libc that includes the termios API.
  - Tested against glibc-2.15 on Ubuntu 12.04. (See travis)
  - Tested against glibc-2.20 on Arch Linux
- `socat`, used to create virtual serial ports, only required to run the tests.

# License

serial.rs is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.

[bindgen]: https://github.com/crabtw/rust-bindgen
[docs]: http://japaric.github.io/serial.rs/serial/
[status]: https://travis-ci.org/japaric/serial.rs.svg?branch=master
