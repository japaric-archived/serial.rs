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

pub static FAILURE: c_int = -1;
pub static IXANY: tcflag_t = 0x0800;
pub static PARENB: tcflag_t = 0x1000;
pub static SUCCESS: c_int = 0;
pub static TCSANOW: c_int = 0;

#[cfg(target_os = "linux")]
mod os {
    use libc::c_uint;
    use super::cc_t;

    #[allow(non_camel_case_types)]
    pub type speed_t = c_uint;
    #[allow(non_camel_case_types)]
    pub type tcflag_t = c_uint;

    pub static B0: speed_t = 0x00;
    pub static B1000000: speed_t = 0x1008;
    pub static B110: speed_t = 0x03;
    pub static B1152000: speed_t = 0x1009;
    pub static B115200: speed_t = 0x1002;
    pub static B1200: speed_t = 0x09;
    pub static B134: speed_t = 0x04;
    pub static B1500000: speed_t = 0x100A;
    pub static B150: speed_t = 0x05;
    pub static B1800: speed_t = 0x0A;
    pub static B19200: speed_t = 0x0E;
    pub static B2000000: speed_t = 0x100B;
    pub static B200: speed_t = 0x06;
    pub static B230400: speed_t = 0x1003;
    pub static B2400: speed_t = 0x0B;
    pub static B2500000: speed_t = 0x100C;
    pub static B3000000: speed_t = 0x100D;
    pub static B300: speed_t = 0x07;
    pub static B3500000: speed_t = 0x100E;
    pub static B38400: speed_t = 0x0F;
    pub static B4000000: speed_t = 0x100F;
    pub static B460800: speed_t = 0x1004;
    pub static B4800: speed_t = 0x0C;
    pub static B500000: speed_t = 0x1005;
    pub static B50: speed_t = 0x01;
    pub static B576000: speed_t = 0x1006;
    pub static B57600: speed_t = 0x1001;
    pub static B600: speed_t = 0x08;
    pub static B75: speed_t = 0x02;
    pub static B921600: speed_t = 0x1007;
    pub static B9600: speed_t = 0x0D;
    pub static CRTSCTS: tcflag_t = 0x80000000;
    pub static CS5: tcflag_t = 0x00;
    pub static CS6: tcflag_t = 0x10;
    pub static CS7: tcflag_t = 0x20;
    pub static CS8: tcflag_t = 0x30;
    pub static CSIZE: tcflag_t = 0x30;
    pub static CSTOPB: tcflag_t = 0x40;
    pub static IXOFF: tcflag_t = 0x1000;
    pub static IXON: tcflag_t = 0x0400;
    pub static NCCS: uint = 32;
    pub static PARODD: tcflag_t = 0x0200;
    pub static VMIN: cc_t = 6;
    pub static VTIME: cc_t = 5;
}

#[cfg(target_os = "macos")]
mod os {
    use libc::c_ulong;
    use super::cc_t;

    #[allow(non_camel_case_types)]
    pub type speed_t = c_ulong;
    #[allow(non_camel_case_types)]
    pub type tcflag_t = c_ulong;

    pub static B0: speed_t = 0;
    pub static B110: speed_t = 110;
    pub static B115200: speed_t = 115200;
    pub static B1200: speed_t = 1200;
    pub static B134: speed_t = 134;
    pub static B14400: speed_t = 14400;
    pub static B150: speed_t = 150;
    pub static B1800: speed_t = 1800;
    pub static B19200: speed_t = 19200;
    pub static B200: speed_t = 200;
    pub static B230400: speed_t = 230400;
    pub static B2400: speed_t = 2400;
    pub static B28800: speed_t = 28800;
    pub static B300: speed_t = 300;
    pub static B38400: speed_t = 38400;
    pub static B4800: speed_t = 4800;
    pub static B50: speed_t = 50;
    pub static B57600: speed_t = 57600;
    pub static B600: speed_t = 600;
    pub static B7200: speed_t = 7200;
    pub static B75: speed_t = 75;
    pub static B76800: speed_t = 76800;
    pub static B9600: speed_t = 9600;
    pub static CRTSCTS: tcflag_t = 0x020000 | 0x040000;
    pub static CS5: tcflag_t = 0x0000;
    pub static CS6: tcflag_t = 0x0100;
    pub static CS7: tcflag_t = 0x0200;
    pub static CS8: tcflag_t = 0x0300;
    pub static CSIZE: tcflag_t = 0x0300;
    pub static CSTOPB: tcflag_t = 0x0400;
    pub static IXOFF: tcflag_t = 0x0400;
    pub static IXON: tcflag_t = 0x0200;
    pub static NCCS: uint = 20;
    pub static PARODD: tcflag_t = 0x2000;
    pub static VMIN: cc_t = 16;
    pub static VTIME: cc_t = 17;
}

#[repr(C)]
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
