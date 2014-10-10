use std::io::{Read, ReadWrite, Write};
use std::str;

use {
    BlockingMode, SerialPort,
    //Direction,
        BothDirections, Input, Output,
    BaudRate,
        B0, B50, B75, B110, B134, B150, B200, B300, B600, B1K2, B2K4, B4K8, B9K6, B19K2, B38K4,
        B57K6, B115K2, B230K4,
    //DataBits,
        Data5, Data6, Data7, Data8,
    //FlowControl,
        HardwareControl, NoFlowControl, SoftwareControl,
    //Parity,
        EvenParity, NoParity, OddParity,
    //StopBits,
        Stop1, Stop2,
};

#[cfg(target_os = "linux")]
use {B460K8, B500K, B576K, B921K6, B1M, B1M152, B1M5, B2M, B2M5, B3M, B3M5, B4M};

#[cfg(target_os = "macos")]
use {B7K2, B14K4, B28K8, B76K8};

use socat::Socat;

#[cfg(target_os = "linux")]
const BAUD_RATES: &'static [BaudRate] = &[
    B0, B50, B75, B110, B134, B150, B200, B300, B600, B1K2, B2K4, B4K8, B9K6, B19K2, B38K4, B57K6,
    B115K2, B230K4, B460K8, B500K, B576K, B921K6, B1M, B1M152, B1M5, B2M, B2M5, B3M, B3M5, B4M];

#[cfg(target_os = "macos")]
const BAUD_RATES: &'static [BaudRate] = &[
    B0, B50, B75, B110, B134, B150, B200, B300, B600, B1K2, B2K4, B4K8, B7K2, B9K6, B14K4, B19K2,
    B28K8, B38K4, B57K6, B115K2, B230K4];

const MESSAGE: &'static str = "Hello World!";

#[test]
fn bidirectional_baud_rate() {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port, Read) {
        Err(e) => fail!("{}: Couldn't open ({})", port_, e),
        Ok(port) => port,
    };

    for &rate in BAUD_RATES.iter() {
        match port.set_baud_rate(BothDirections, rate) {
            Err(e) => fail!("{}: Couldn't set both baud rates to {} ({})", port_, rate, e),
            Ok(_) => {},
        }
        let got = match port.baud_rate() {
            Err(e) => fail!("{}: Couldn't read baud rate ({})", port_, e),
            Ok(rate) => rate,
        };

        if (rate, rate) != got {
            fail!("{}: set {} - got {}", port_, rate, got);
        }
    }
}

#[quickcheck]
fn blocking_mode(bytes: u8, deciseconds: u8) -> bool {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port, Read) {
        Err(e) => fail!("{}: Couldn't open ({})", port_, e),
        Ok(port) => port,
    };

    let mode = BlockingMode { bytes: bytes, deciseconds: deciseconds };
    match port.set_blocking_mode(mode) {
        Err(e) => {
            fail!("{}: Couldn't set blocking mode to {} ({})", port_, (bytes, deciseconds), e)
        },
        Ok(_) => {},
    }

    match port.blocking_mode() {
        Err(e) => fail!("{}: Couldn't read blocking mode ({})", port_, e),
        Ok(got) => got == mode
    }
}

// XXX The PTY only seems to work with 8 data bits
#[test]
#[ignore]
fn data_bits() {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port, Read) {
        Err(e) => fail!("{}: Couldn't open ({})", port_, e),
        Ok(port) => port,
    };

    for &bits in [Data5, Data6, Data7, Data8].iter() {
        match port.set_data_bits(bits) {
            Err(e) => fail!("{}: Couldn't set data bits to {} ({})", port_, bits, e),
            Ok(_) => {},
        }
        let got = match port.data_bits() {
            Err(e) => fail!("{}: Couldn't read data bits ({})", port_, e),
            Ok(bits) => bits,
        };

        if bits != got {
            fail!("{}: set {} - got {}", port_, bits, got);
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

    let first = SerialPort::open(port, Write);
    let second = SerialPort::open(port, Write);

    assert!(first.is_ok() && second.is_err());
}

#[test]
fn flow_control() {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port, Read) {
        Err(e) => fail!("{}: Couldn't open ({})", port_, e),
        Ok(port) => port,
    };

    for &flow in [HardwareControl, NoFlowControl, SoftwareControl].iter() {
        match port.set_flow_control(flow) {
            Err(e) => fail!("{}: Couldn't set flow control to {} ({})", port_, flow, e),
            Ok(_) => {},
        }
        let got = match port.flow_control() {
            Err(e) => fail!("{}: Couldn't read flow control ({})", port_, e),
            Ok(flow) => flow,
        };

        if flow != got {
            fail!("{}: set {} - got {}", port_, flow, got)
        }
    }
}

#[test]
fn input_baud_rate() {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port, Read) {
        Err(e) => fail!("{}: Couldn't open ({})", port_, e),
        Ok(port) => port,
    };

    for &rate in BAUD_RATES.iter() {
        // XXX Can't set input baud rate to `B0`
        if rate == B0 {
            continue
        }

        match port.set_baud_rate(Input, rate) {
            Err(e) => fail!("{}: Couldn't set input baud rate to {} ({})", port_, rate, e),
            Ok(_) => {},
        }
        let got = match port.baud_rate() {
            Err(e) => fail!("{}: Couldn't read baud rate ({})", port_, e),
            Ok(rates) => rates.0,
        };

        if rate != got {
            fail!("{}: set {} - got {}", port_, rate, got)
        }
    }
}

#[test]
fn loopback() {
    let socat = Socat::new();
    let (tx, rx) = socat.ports();
    let (tx_, rx_) = (tx.display(), rx.display());
    let mut tx = match SerialPort::open(tx, Write) {
        Err(e) => fail!("{}: Couldn't open ({})", tx_, e),
        Ok(port) => port,
    };
    let mut rx = match SerialPort::open(rx, Read) {
        Err(e) => fail!("{}: Couldn't open ({})", rx_, e),
        Ok(port) => port,
    };

    match tx.write_str(MESSAGE) {
        Err(e) => fail!("{}: Couldn't send message ({})", tx_, e),
        _ => {},
    }

    match rx.read_exact(MESSAGE.len()) {
        Err(e) => fail!("{}: Couldn't read ({})", rx_, e),
        Ok(buf) => assert_eq!(str::from_utf8(buf[]), Some(MESSAGE)),
    }
}

#[test]
fn open() {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();

    for &access in [Read, ReadWrite, Write].iter() {
        match SerialPort::open(port, Read) {
            Err(e) => {
                let access = match access {
                    Read => "read",
                    ReadWrite => "read/write",
                    Write => "write",
                };

                fail!("{}: Couldn't open in {} mode ({})", port_, access, e)
            },
            Ok(_) => {},
        }
    }
}

#[test]
fn output_baud_rate() {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port, Read) {
        Err(e) => fail!("{}: Couldn't open ({})", port_, e),
        Ok(port) => port,
    };

    for &rate in BAUD_RATES.iter() {
        match port.set_baud_rate(Output, rate) {
            Err(e) => fail!("{}: Couldn't set output baud rate to {} ({})", port_, rate, e),
            Ok(_) => {},
        }
        let got = match port.baud_rate() {
            Err(e) => fail!("{}: Couldn't read baud rate ({})", port_, e),
            Ok(rates) => rates.1,
        };

        if rate != got {
            fail!("{}: set {} - got {}", port_, rate, got)
        }
    }
}

// XXX The PTY only seems to work with no parity
#[test]
#[ignore]
fn parity() {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port, Read) {
        Err(e) => fail!("{}: Couldn't open ({})", port_, e),
        Ok(port) => port,
    };

    for &parity in [EvenParity, NoParity, OddParity].iter() {
        match port.set_parity(parity) {
            Err(e) => fail!("{}: Couldn't set parity to {} ({})", port_, parity, e),
            Ok(_) => {},
        }
        let got = match port.parity() {
            Err(e) => fail!("{}: Couldn't read parity ({})", port_, e),
            Ok(parity) => parity,
        };

        if parity != got {
            fail!("{}: set {} - got {}", port_, parity, got)
        }
    }
}

#[test]
fn read_in_write_only_mode() {
    let socat = Socat::new();
    let mut port = SerialPort::open(socat.ports().0, Write);

    assert!(port.read_to_string().is_err())
}

#[test]
fn stop_bits() {
    let socat = Socat::new();
    let port = socat.ports().0;
    let port_ = port.display();
    let mut port = match SerialPort::open(port, Read) {
        Err(e) => fail!("{}: Couldn't open ({})", port_, e),
        Ok(port) => port,
    };

    for &bits in [Stop1, Stop2].iter() {
        match port.set_stop_bits(bits) {
            Err(e) => fail!("{}: Couldn't set stop bits to {} ({})", port_, bits, e),
            Ok(_) => {},
        }
        let got = match port.stop_bits() {
            Err(e) => fail!("{}: Couldn't read parity ({})", port_, e),
            Ok(bits) => bits,
        };

        if bits != got {
            fail!("{}: set {} - got {}", port_, bits, got)
        }
    }
}

#[test]
fn write_in_read_only_mode() {
    let socat = Socat::new();
    let mut port = SerialPort::open(socat.ports().0, Read);

    assert!(port.write_str(MESSAGE).is_err())
}
