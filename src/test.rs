use std::io::{Read, Write};
use std::{iter, str};

use BaudRate::*;
use {BaudRate, BlockingMode, Direction, OpenOptions, SerialPort};

use socat::Socat;

const BAUD_RATES: &'static [BaudRate; 19] = &[
    B0,
    B50,
    B75,
    B110,
    B134,
    B150,
    B200,
    B300,
    B600,
    B1200,
    B1800,
    B2400,
    B4800,
    B9600,
    B19200,
    B38400,
    B57600,
    B115200,
    B230400,
];

const MESSAGE: &'static str = "Hello World!";

#[test]
fn bidirectional_baud_rate() {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port) {
        Err(e) => panic!("{:?}: Couldn't open ({:?})", port_, e),
        Ok(port) => port,
    };

    for &rate in BAUD_RATES {
        if let Err(e) = port.set_baud_rate(Direction::Both, rate) {
            panic!("{:?}: Couldn't set both baud rates to {:?} ({:?})", port_, rate, e)
        }
        let got = match port.baud_rate() {
            Err(e) => panic!("{:?}: Couldn't read baud rate ({:?})", port_, e),
            Ok(rate) => rate,
        };

        if (rate, rate) != got {
            panic!("{:?}: set {:?} - got {:?}", port_, rate, got);
        }
    }
}

#[quickcheck]
fn blocking_mode(bytes: u8, deciseconds: u8) -> bool {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port) {
        Err(e) => panic!("{:?}: Couldn't open ({:?})", port_, e),
        Ok(port) => port,
    };

    let mode = BlockingMode { bytes: bytes, deciseconds: deciseconds };
    if let Err(e) =  port.set_blocking_mode(mode) {
        panic!("{:?}: Couldn't set blocking mode to {:?} ({:?})", port_, (bytes, deciseconds), e)
    }

    match port.blocking_mode() {
        Err(e) => panic!("{:?}: Couldn't read blocking mode ({:?})", port_, e),
        Ok(got) => got == mode
    }
}

// XXX The PTY only seems to work with 8 data bits
#[test]
#[ignore]
fn data_bits() {
    use DataBits::*;

    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port) {
        Err(e) => panic!("{:?}: Couldn't open ({:?})", port_, e),
        Ok(port) => port,
    };

    for &bits in &[Five, Six, Seven, Eight] {
        if let Err(e) =  port.set_data_bits(bits) {
            panic!("{:?}: Couldn't set data bits to {:?} ({:?})", port_, bits, e)
        }
        let got = match port.data_bits() {
            Err(e) => panic!("{:?}: Couldn't read data bits ({:?})", port_, e),
            Ok(bits) => bits,
        };

        if bits != got {
            panic!("{:?}: set {:?} - got {:?}", port_, bits, got);
        }
    }
}

// XXX Should opening a port twice be forbidden?
// - AFAIK, opening a port twice is not possible in Windows, but it's possible on Linux
// - FWIW, QSerialPort and minicom forbid this operation via lockfiles
#[test]
#[ignore]
fn double_open() {
    let socat = Socat::new();
    let port = socat.ports().0;

    let mut opts = OpenOptions::new();
    opts.write(true);
    let first = opts.open(port);
    let second = opts.open(port);

    assert!(first.is_ok() && second.is_err());
}

#[test]
fn flow_control() {
    use FlowControl::*;

    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port) {
        Err(e) => panic!("{:?}: Couldn't open ({:?})", port_, e),
        Ok(port) => port,
    };

    for &flow in &[Hardware, None, Software] {
        if let Err(e) =  port.set_flow_control(flow) {
            panic!("{:?}: Couldn't set flow control to {:?} ({:?})", port_, flow, e)
        }
        let got = match port.flow_control() {
            Err(e) => panic!("{:?}: Couldn't read flow control ({:?})", port_, e),
            Ok(flow) => flow,
        };

        if flow != got {
            panic!("{:?}: set {:?} - got {:?}", port_, flow, got)
        }
    }
}

#[test]
fn input_baud_rate() {
    use Direction::Input;

    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port) {
        Err(e) => panic!("{:?}: Couldn't open ({:?})", port_, e),
        Ok(port) => port,
    };

    for &rate in BAUD_RATES {
        // XXX Can't set input baud rate to `B0`
        if rate == B0 {
            continue
        }

        if let Err(e) = port.set_baud_rate(Input, rate) {
            panic!("{:?}: Couldn't set input baud rate to {:?} ({:?})", port_, rate, e)
        }
        let got = match port.baud_rate() {
            Err(e) => panic!("{:?}: Couldn't read baud rate ({:?})", port_, e),
            Ok(rates) => rates.0,
        };

        if rate != got {
            panic!("{:?}: set {:?} - got {:?}", port_, rate, got)
        }
    }
}

#[test]
fn loopback() {
    let socat = Socat::new();
    let (tx, rx) = socat.ports();
    let (tx_, rx_) = (tx.display(), rx.display());
    let mut tx = match OpenOptions::new().write(true).open(tx) {
        Err(e) => panic!("{:?}: Couldn't open ({:?})", tx_, e),
        Ok(port) => port,
    };
    let mut rx = match SerialPort::open(rx) {
        Err(e) => panic!("{:?}: Couldn't open ({:?})", rx_, e),
        Ok(port) => port,
    };

    if let Err(e) = tx.write_all(MESSAGE.as_bytes()) {
        panic!("{:?}: Couldn't send message ({:?})", tx_, e)
    }

    let n = MESSAGE.len();
    let mut buf: Vec<u8> = iter::repeat(0).take(n).collect();
    match rx.read(&mut buf) {
        Err(e) => panic!("{:?}: Couldn't read ({:?})", rx_, e),
        Ok(k) => {
            if n == k {
                assert_eq!(str::from_utf8(&buf[..n]).ok(), Some(MESSAGE))
            } else {
                panic!("expected {} bytes, got {}", n, k);
            }
        },
    }
}

#[test]
fn open() {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();

    for &read in &[true, false] {
        for &write in &[true, false] {
            if let Err(e) = OpenOptions::new().read(read).write(write).open(port) {
                let access = match (read, write) {
                    (true, true) => "read/write",
                    (false, true) => "write",
                    (_, false) => "read",
                };

                panic!("{:?}: Couldn't open in {:?} mode ({:?})", port_, access, e)
            }
        }
    }
}

#[test]
fn output_baud_rate() {
    use Direction::Output;

    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port) {
        Err(e) => panic!("{:?}: Couldn't open ({:?})", port_, e),
        Ok(port) => port,
    };

    for &rate in BAUD_RATES {
        if let Err(e) = port.set_baud_rate(Output, rate) {
            panic!("{:?}: Couldn't set output baud rate to {:?} ({:?})", port_, rate, e)
        }
        let got = match port.baud_rate() {
            Err(e) => panic!("{:?}: Couldn't read baud rate ({:?})", port_, e),
            Ok(rates) => rates.1,
        };

        if rate != got {
            panic!("{:?}: set {:?} - got {:?}", port_, rate, got)
        }
    }
}

// XXX The PTY only seems to work with no parity
#[test]
#[ignore]
fn parity() {
    use Parity::*;

    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port) {
        Err(e) => panic!("{:?}: Couldn't open ({:?})", port_, e),
        Ok(port) => port,
    };

    for &parity in &[Even, None, Odd] {
        if let Err(e) = port.set_parity(parity) {
            panic!("{:?}: Couldn't set parity to {:?} ({:?})", port_, parity, e)
        }
        let got = match port.parity() {
            Err(e) => panic!("{:?}: Couldn't read parity ({:?})", port_, e),
            Ok(parity) => parity,
        };

        if parity != got {
            panic!("{:?}: set {:?} - got {:?}", port_, parity, got)
        }
    }
}

#[test]
fn read_in_write_only_mode() {
    let socat = Socat::new();
    let mut port = OpenOptions::new().write(true).open(socat.ports().0).unwrap();
    let mut buf = Vec::new();

    assert!(port.read_to_end(&mut buf).is_err())
}

#[test]
fn stop_bits() {
    use StopBits::*;

    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port) {
        Err(e) => panic!("{:?}: Couldn't open ({:?})", port_, e),
        Ok(port) => port,
    };

    for &bits in &[One, Two] {
        if let Err(e) = port.set_stop_bits(bits) {
            panic!("{:?}: Couldn't set stop bits to {:?} ({:?})", port_, bits, e)
        }
        let got = match port.stop_bits() {
            Err(e) => panic!("{:?}: Couldn't read parity ({:?})", port_, e),
            Ok(bits) => bits,
        };

        if bits != got {
            panic!("{:?}: set {:?} - got {:?}", port_, bits, got)
        }
    }
}

#[test]
fn write_in_read_only_mode() {
    let socat = Socat::new();
    let mut port = SerialPort::open(socat.ports().0).unwrap();

    assert!(port.write_all(MESSAGE.as_bytes()).is_err())
}
