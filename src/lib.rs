#![allow(unused_features)]
#![cfg_attr(test, plugin(quickcheck_macros))]
#![deny(missing_docs, warnings)]
#![feature(convert)]
#![feature(custom_attribute)]
#![feature(fs)]
#![feature(io)]
#![feature(io_ext)]
#![feature(path)]
#![feature(plugin)]
#![feature(std_misc)]

//! A library for serial port communication

extern crate termios;
#[cfg(test)]
extern crate quickcheck;

use std::fmt;
use std::fs::{File, self};
use std::io::{Read, Write, self};
use std::os::unix::io::AsRawFd;
use std::path::Path;

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

/// Options and flags which can be used to configure how a serial port is opened.
pub struct OpenOptions(fs::OpenOptions);

impl OpenOptions {
    /// Creates a blank net set of options ready for configuration.
    ///
    /// All options are initially set to false.
    pub fn new() -> OpenOptions {
        OpenOptions(fs::OpenOptions::new())
    }

    /// Set the option for read access.
    ///
    /// This option, when true, will indicate that the serial port should be read-able when opened.
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.0.read(read);
        self
    }

    /// Set the option for write access.
    ///
    /// This option, when true, will indicate that the serial port should be write-able when
    /// opened.
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.0.write(write);
        self
    }

    /// Opens a serial port in "raw" mode with the specified read/write permissions.
    ///
    /// If no permission was specified, the port will be opened in read only mode.
    pub fn open<P: ?Sized>(&self, port: &P) -> io::Result<SerialPort> where
        P: AsRef<Path>,
    {
        self.open_(port.as_ref())
    }

    fn open_(&self, path: &Path) -> io::Result<SerialPort> {
        let file = try!(self.0.open(path));

        let mut termios = try!(Termios::fetch(file.as_raw_fd()));
        termios.make_raw();

        let sp = SerialPort(file);

        try!(sp.update(termios));

        Ok(sp)
    }
}

/// A serial device
pub struct SerialPort(File);

impl SerialPort {
    /// Opens a serial port in "raw" mode with read-only permission
    pub fn open(port: &Path) -> io::Result<SerialPort> {
        OpenOptions::new().open(port)
    }

    /// Returns the input and output baud rates
    pub fn baud_rate(&self) -> io::Result<(BaudRate, BaudRate)> {
        self.fetch().map(|termios| {
            (termios.ispeed(), termios.ospeed())
        })
    }

    /// Returns the blocking mode used by the device
    pub fn blocking_mode(&self) -> io::Result<BlockingMode> {
        self.fetch().map(|termios| {
            BlockingMode {
                bytes: termios.cc[control::Char::VMIN],
                deciseconds: termios.cc[control::Char::VTIME],
            }
        })
    }

    /// Returns the number of data bits used per character
    pub fn data_bits(&self) -> io::Result<DataBits> {
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
    pub fn flow_control(&self) -> io::Result<FlowControl> {
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
    pub fn parity(&self) -> io::Result<Parity> {
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
    pub fn set_baud_rate(&mut self, direction: Direction, rate: BaudRate) -> io::Result<()> {
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
    pub fn set_blocking_mode(&mut self, mode: BlockingMode) -> io::Result<()> {
        self.fetch().and_then(|mut termios| {
            termios.cc[control::Char::VMIN] = mode.bytes;
            termios.cc[control::Char::VTIME] = mode.deciseconds;

            self.update(termios)
        })
    }

    /// Changes the number of data bits per character
    pub fn set_data_bits(&mut self, bits: DataBits) -> io::Result<()> {
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
    pub fn set_flow_control(&mut self, flow: FlowControl) -> io::Result<()> {
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
    pub fn set_parity(&mut self, parity: Parity) -> io::Result<()> {
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
    pub fn set_stop_bits(&mut self, bits: StopBits) -> io::Result<()> {
        self.fetch().and_then(|mut termios| {
            match bits {
                StopBits::One => termios.clear(control::Flag::CSTOPB),
                StopBits::Two => termios.set(control::Flag::CSTOPB),
            }

            self.update(termios)
        })
    }

    /// Returns the number of stop bits per character
    pub fn stop_bits(&self) -> io::Result<StopBits> {
        self.fetch().map(|termios| {
            if termios.contains(control::Flag::CSTOPB) {
                StopBits::Two
            } else {
                StopBits::One
            }
        })
    }

    /// Fetches the current state of the termios structure
    fn fetch(&self) -> io::Result<Termios> {
        Termios::fetch(self.0.as_raw_fd())
    }

    /// Updates the underlying termios structure
    fn update(&self, termios: Termios) -> io::Result<()> {
        termios.update(self.0.as_raw_fd(), When::Now)
    }
}

impl Read for SerialPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.0.read_to_end(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.0.read_to_string(buf)
    }
}

impl Write for SerialPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
        self.0.write_fmt(fmt)
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
