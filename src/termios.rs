use libc::{c_int, c_uchar};

pub use self::os::{
    B0, B50, B75, B110, B134, B150, B200, B300, B600, B1200, B1800, B2400, B4800, B9600, B19200,
    B38400, B57600, B115200, B230400, CRTSCTS, CS5, CS6, CS7, CS8, CSIZE, CSTOPB, IXOFF, IXON,
    NCCS, PARODD, VMIN, VTIME, speed_t,
};

#[cfg(target_os = "linux")]
pub use self::os::{
    B460800, B500000, B576000, B921600, B1000000, B1152000, B1500000, B2000000, B2500000, B3000000,
    B3500000, B4000000
};

#[cfg(target_os = "macos")]
pub use self::os::{B7200, B14400, B28800, B76800};

use self::os::tcflag_t;

#[allow(non_camel_case_types)]
pub type cc_t = c_uchar;

pub const FAILURE: c_int = -1;
pub const IXANY: tcflag_t = 0x0800;
pub const PARENB: tcflag_t = 0x1000;
pub const SUCCESS: c_int = 0;
pub const TCSANOW: c_int = 0;

#[cfg(target_os = "linux")]
mod os {
    use libc::c_uint;
    use super::cc_t;

    #[allow(non_camel_case_types)]
    pub type speed_t = c_uint;
    #[allow(non_camel_case_types)]
    pub type tcflag_t = c_uint;

    pub const B0: speed_t = 0x00;
    pub const B1000000: speed_t = 0x1008;
    pub const B110: speed_t = 0x03;
    pub const B1152000: speed_t = 0x1009;
    pub const B115200: speed_t = 0x1002;
    pub const B1200: speed_t = 0x09;
    pub const B134: speed_t = 0x04;
    pub const B1500000: speed_t = 0x100A;
    pub const B150: speed_t = 0x05;
    pub const B1800: speed_t = 0x0A;
    pub const B19200: speed_t = 0x0E;
    pub const B2000000: speed_t = 0x100B;
    pub const B200: speed_t = 0x06;
    pub const B230400: speed_t = 0x1003;
    pub const B2400: speed_t = 0x0B;
    pub const B2500000: speed_t = 0x100C;
    pub const B3000000: speed_t = 0x100D;
    pub const B300: speed_t = 0x07;
    pub const B3500000: speed_t = 0x100E;
    pub const B38400: speed_t = 0x0F;
    pub const B4000000: speed_t = 0x100F;
    pub const B460800: speed_t = 0x1004;
    pub const B4800: speed_t = 0x0C;
    pub const B500000: speed_t = 0x1005;
    pub const B50: speed_t = 0x01;
    pub const B576000: speed_t = 0x1006;
    pub const B57600: speed_t = 0x1001;
    pub const B600: speed_t = 0x08;
    pub const B75: speed_t = 0x02;
    pub const B921600: speed_t = 0x1007;
    pub const B9600: speed_t = 0x0D;
    pub const CRTSCTS: tcflag_t = 0x80000000;
    pub const CS5: tcflag_t = 0x00;
    pub const CS6: tcflag_t = 0x10;
    pub const CS7: tcflag_t = 0x20;
    pub const CS8: tcflag_t = 0x30;
    pub const CSIZE: tcflag_t = 0x30;
    pub const CSTOPB: tcflag_t = 0x40;
    pub const IXOFF: tcflag_t = 0x1000;
    pub const IXON: tcflag_t = 0x0400;
    pub const NCCS: uint = 32;
    pub const PARODD: tcflag_t = 0x0200;
    pub const VMIN: cc_t = 6;
    pub const VTIME: cc_t = 5;
}

#[cfg(target_os = "macos")]
mod os {
    use libc::c_ulong;
    use super::cc_t;

    #[allow(non_camel_case_types)]
    pub type speed_t = c_ulong;
    #[allow(non_camel_case_types)]
    pub type tcflag_t = c_ulong;

    pub const B0: speed_t = 0;
    pub const B110: speed_t = 110;
    pub const B115200: speed_t = 115200;
    pub const B1200: speed_t = 1200;
    pub const B134: speed_t = 134;
    pub const B14400: speed_t = 14400;
    pub const B150: speed_t = 150;
    pub const B1800: speed_t = 1800;
    pub const B19200: speed_t = 19200;
    pub const B200: speed_t = 200;
    pub const B230400: speed_t = 230400;
    pub const B2400: speed_t = 2400;
    pub const B28800: speed_t = 28800;
    pub const B300: speed_t = 300;
    pub const B38400: speed_t = 38400;
    pub const B4800: speed_t = 4800;
    pub const B50: speed_t = 50;
    pub const B57600: speed_t = 57600;
    pub const B600: speed_t = 600;
    pub const B7200: speed_t = 7200;
    pub const B75: speed_t = 75;
    pub const B76800: speed_t = 76800;
    pub const B9600: speed_t = 9600;
    pub const CRTSCTS: tcflag_t = 0x020000 | 0x040000;
    pub const CS5: tcflag_t = 0x0000;
    pub const CS6: tcflag_t = 0x0100;
    pub const CS7: tcflag_t = 0x0200;
    pub const CS8: tcflag_t = 0x0300;
    pub const CSIZE: tcflag_t = 0x0300;
    pub const CSTOPB: tcflag_t = 0x0400;
    pub const IXOFF: tcflag_t = 0x0400;
    pub const IXON: tcflag_t = 0x0200;
    pub const NCCS: uint = 20;
    pub const PARODD: tcflag_t = 0x2000;
    pub const VMIN: cc_t = 16;
    pub const VTIME: cc_t = 17;
}

#[repr(C)]
#[deriving(Copy)]
pub struct Termios {
    pub c_iflag: tcflag_t,
    c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    c_lflag: tcflag_t,
    #[cfg(target_os = "linux")] c_line: cc_t,
    pub c_cc: [cc_t, ..NCCS],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

// TODO (rust-lang/rust#7622) Remove the `new()` method, make `Termios` derive the `Default` trait
impl Termios {
    #[cfg(target_os = "linux")]
    pub fn new() -> Termios {
        Termios {
            c_cc: [0, ..NCCS],
            c_cflag: 0,
            c_iflag: 0,
            c_ispeed: 0,
            c_lflag: 0,
            c_line: 0,
            c_oflag: 0,
            c_ospeed: 0,
        }
    }

    #[cfg(target_os = "macos")]
    pub fn new() -> Termios {
        Termios {
            c_cc: [0, ..NCCS],
            c_cflag: 0,
            c_iflag: 0,
            c_ispeed: 0,
            c_lflag: 0,
            c_oflag: 0,
            c_ospeed: 0,
        }
    }
}

#[link(name = "c")]
extern {
    pub fn cfmakeraw(termios: *mut Termios);
    pub fn cfsetispeed(termios: *mut Termios, speed: speed_t) -> c_int;
    pub fn cfsetospeed(termios: *mut Termios, speed: speed_t) -> c_int;
    pub fn cfsetspeed(termios: *mut Termios, speed: speed_t) -> c_int;
    pub fn tcgetattr(fd: c_int, termios: *mut Termios) -> c_int;
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const Termios) -> c_int;
}
