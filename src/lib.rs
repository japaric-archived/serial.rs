#![allow(unused_features)]
#![cfg_attr(test, plugin(quickcheck_macros))]
#![deny(missing_docs, warnings)]
#![feature(collections)]
#![feature(core)]
#![feature(io)]
#![feature(libc)]
#![feature(old_io)]
#![feature(old_path)]
#![feature(path)]
#![feature(plugin)]
#![feature(std_misc)]

//! A library for serial port communication

extern crate termios;
#[cfg(test)]
extern crate quickcheck;

use std::old_io::{File, FileAccess, FileMode, IoResult};
use std::os::unix::AsRawFd;

pub use termios::BaudRate;

use termios::prelude::*;

#[cfg(test)]
mod socat;
#[cfg(test)]
mod test;

/// For how long to block `read()` calls
#[derive(Copy, PartialEq)]
pub struct BlockingMode {
    /// The device will block until *at least* `bytes` are received
    pub bytes: u8,
    /// The device will block for at least `deciseconds` after each `read()` call
    pub deciseconds: u8,
}

/// A serial device
pub struct SerialPort(File);

impl SerialPort {
    /// Opens a serial `device` in "raw" mode
    pub fn open(device: &Path, access: FileAccess) -> IoResult<SerialPort> {
        let file = try!(File::open_mode(device, FileMode::Open, access));

        let mut termios = try!(Termios::fetch(file.as_raw_fd()));
        termios.make_raw();

        let sp = SerialPort(file);

        try!(sp.update(termios));

        Ok(sp)
    }

    /// Returns the input and output baud rates
    pub fn baud_rate(&self) -> IoResult<(BaudRate, BaudRate)> {
        self.fetch().map(|termios| {
            (termios.ispeed(), termios.ospeed())
        })
    }

    /// Returns the blocking mode used by the device
    pub fn blocking_mode(&self) -> IoResult<BlockingMode> {
        self.fetch().map(|termios| {
            BlockingMode {
                bytes: termios.cc[control::Char::VMIN],
                deciseconds: termios.cc[control::Char::VTIME],
            }
        })
    }

    /// Returns the number of data bits used per character
    pub fn data_bits(&self) -> IoResult<DataBits> {
        self.fetch().map(|termios| {
            match termios.get::<control::CSIZE>() {
                control::CSIZE::CS5 => DataBits::Five,
                control::CSIZE::CS6 => DataBits::Six,
                control::CSIZE::CS7 => DataBits::Seven,
                control::CSIZE::CS8 => DataBits::Eight,
            }
        })
    }

    /// Returns the flow control used by the device
    pub fn flow_control(&self) -> IoResult<FlowControl> {
        self.fetch().map(|termios| {
            if termios.contains(control::Flag::CRTSCTS) {
                FlowControl::Hardware
            } else if termios.contains(input::Flag::IXANY) &&
                termios.contains(input::Flag::IXOFF) &&
                termios.contains(input::Flag::IXON)
            {
                FlowControl::Software
            } else {
                FlowControl::None
            }
        })
    }

    /// Returns the bit parity used by the device
    pub fn parity(&self) -> IoResult<Parity> {
        self.fetch().map(|termios| {
            match (
                termios.contains(control::Flag::PARENB),
                termios.contains(control::Flag::PARODD),
            ) {
                (true, true) => Parity::Odd,
                (true, false) => Parity::Even,
                (false, _) => Parity::None,
            }
        })
    }

    /// Changes the baud rate of the input/output or both directions
    pub fn set_baud_rate(&mut self, direction: Direction, rate: BaudRate) -> IoResult<()> {
        self.fetch().and_then(|mut termios| {
            match direction {
                Direction::Both => termios.set_speed(rate),
                Direction::Input => termios.set_ispeed(rate),
                Direction::Output => termios.set_ospeed(rate),
            }

            self.update(termios)
        })
    }

    /// Changes the blocking mode used by the device
    pub fn set_blocking_mode(&mut self, mode: BlockingMode) -> IoResult<()> {
        self.fetch().and_then(|mut termios| {
            termios.cc[control::Char::VMIN] = mode.bytes;
            termios.cc[control::Char::VTIME] = mode.deciseconds;

            self.update(termios)
        })
    }

    /// Changes the number of data bits per character
    pub fn set_data_bits(&mut self, bits: DataBits) -> IoResult<()> {
        self.fetch().and_then(|mut termios| {
            termios.set(match bits {
                DataBits::Five => control::CSIZE::CS5,
                DataBits::Six => control::CSIZE::CS6,
                DataBits::Seven => control::CSIZE::CS7,
                DataBits::Eight => control::CSIZE::CS8,
            });

            self.update(termios)
        })
    }

    /// Changes the flow control used by the device
    pub fn set_flow_control(&mut self, flow: FlowControl) -> IoResult<()> {
        self.fetch().and_then(|mut termios| {
            match flow {
                FlowControl::Hardware => {
                    termios.clear(input::Flag::IXANY);
                    termios.clear(input::Flag::IXOFF);
                    termios.clear(input::Flag::IXON);
                    termios.set(control::Flag::CRTSCTS);
                },
                FlowControl::None => {
                    termios.clear(control::Flag::CRTSCTS);
                    termios.clear(input::Flag::IXANY);
                    termios.clear(input::Flag::IXOFF);
                    termios.clear(input::Flag::IXON);
                },
                FlowControl::Software => {
                    termios.clear(control::Flag::CRTSCTS);
                    termios.set(input::Flag::IXANY);
                    termios.set(input::Flag::IXOFF);
                    termios.set(input::Flag::IXON);
                },
            }

            self.update(termios)
        })
    }

    /// Changes the bit parity used by the device
    pub fn set_parity(&mut self, parity: Parity) -> IoResult<()> {
        self.fetch().and_then(|mut termios| {
            match parity {
                Parity::Even => {
                    termios.clear(control::Flag::PARODD);
                    termios.set(control::Flag::PARENB);
                },
                Parity::None => termios.clear(control::Flag::PARENB),
                Parity::Odd => {
                    termios.set(control::Flag::PARENB);
                    termios.set(control::Flag::PARODD);
                },
            }

            self.update(termios)
        })
    }

    /// Changes the number of stop bits per character
    pub fn set_stop_bits(&mut self, bits: StopBits) -> IoResult<()> {
        self.fetch().and_then(|mut termios| {
            match bits {
                StopBits::One => termios.clear(control::Flag::CSTOPB),
                StopBits::Two => termios.set(control::Flag::CSTOPB),
            }

            self.update(termios)
        })
    }

    /// Returns the number of stop bits per character
    pub fn stop_bits(&self) -> IoResult<StopBits> {
        self.fetch().map(|termios| {
            if termios.contains(control::Flag::CSTOPB) {
                StopBits::Two
            } else {
                StopBits::One
            }
        })
    }

    /// Fetches the current state of the termios structure
    fn fetch(&self) -> IoResult<Termios> {
        Termios::fetch(self.0.as_raw_fd())
    }

    /// Updates the underlying termios structure
    fn update(&self, termios: Termios) -> IoResult<()> {
        termios.update(self.0.as_raw_fd(), When::Now)
    }
}

impl Reader for SerialPort {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.0.read(buf)
    }
}

impl Writer for SerialPort {
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        self.0.write_all(buf)
    }
}

#[allow(missing_docs)]
/// Number of data bits
#[derive(Copy, Debug, PartialEq)]
pub enum DataBits {
    Five,
    Six,
    Seven,
    Eight,
}

#[allow(missing_docs)]
#[derive(Copy)]
pub enum Direction {
    Both,
    Input,
    Output,
}

#[allow(missing_docs)]
/// Flow control
#[derive(Copy, Debug, PartialEq)]
pub enum FlowControl {
    Hardware,
    None,
    Software,
}

#[allow(missing_docs)]
/// Parity checking
#[derive(Copy, Debug, PartialEq)]
pub enum Parity {
    Even,
    None,
    Odd,
}

#[allow(missing_docs)]
/// Number of stop bits
#[derive(Copy, Debug, PartialEq)]
pub enum StopBits {
    One,
    Two,
}
