use libc::{c_int, c_uchar, c_uint};

#[allow(non_camel_case_types)]
pub type speed_t = c_uint;
#[allow(non_camel_case_types)]
type cc_t = c_uchar;
#[allow(non_camel_case_types)]
type tcflag_t = c_uint;

pub static CRTSCTS: tcflag_t = 0x80000000;
pub static CSIZE: tcflag_t = 0x30;
pub static CSTOPB: tcflag_t = 0x40;
pub static FAILURE: c_int = -1;
pub static IXANY: tcflag_t = 0x0800;
pub static IXOFF: tcflag_t = 0x1000;
pub static IXON: tcflag_t = 0x0400;
pub static NCCS: c_int = 32;
pub static PARENB: tcflag_t = 0x0100;
pub static PARODD: tcflag_t = 0x0200;
pub static SUCCESS: c_int = 0;
pub static TCSANOW: c_int = 0;
pub static VMIN: cc_t = 6;
pub static VTIME: cc_t = 5;

#[repr(C)]
pub struct Termios {
    pub c_iflag: tcflag_t,
    c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    c_lflag: tcflag_t,
    c_line: cc_t,
    pub c_cc: [cc_t, ..NCCS as uint],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

impl Termios {
    pub fn new() -> Termios {
        Termios {
            c_cc: [0, ..NCCS as uint],
            c_cflag: 0,
            c_iflag: 0,
            c_ispeed: 0,
            c_lflag: 0,
            c_line: 0,
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
