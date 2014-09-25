[![Build Status][status]](https://travis-ci.org/japaric/serial.rs)

# `serial.rs`

A serial port library on top of the termios API

# [Documentation][docs]

# Non-Rust dependencies

- A libc that includes the termios API.
  - Tested against glibc-2.15 on Ubuntu 12.04. (See travis)
  - Tested against glibc-2.20 on some [obscure][exherbo] Linux distro.
  - Tested against OSX 10.9
- `socat`, used to create virtual serial ports, only required to run the tests.

# License

serial.rs is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.

[docs]: http://rust-ci.org/japaric/serial.rs/doc/serial/
[exherbo]: http://exherbo.org/
[status]: https://travis-ci.org/japaric/serial.rs.svg?branch=master
