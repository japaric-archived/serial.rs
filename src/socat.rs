use std::old_io::{BufferedReader, Command, Process};

/// Wrapper around a child `socat` process
pub struct Socat {
    process: Process,
    ports: (Path, Path),
}

impl Socat {
    /// Spawns `socat`
    pub fn new() -> Socat {
        let mut process =
            Command::new("socat").
                args(&["-d", "-d", "pty", "pty"]).
                spawn().
                ok().
                expect("Couldn't find `socat`");

        let mut stderr = BufferedReader::new(process.stderr.take().unwrap());
        let mut devices = stderr.lines().map(|line| {
            Path::new(
                line.
                    ok().
                    expect("Couldn't read `socat` stderr").
                    as_slice().
                    trim().
                    split_str(" is ").
                    skip(1).
                    next().
                    expect("Couldn't parse serial device"))
        });

        // FIXME Reading stderr may block indefinitely if `socat` fails
        let (first, second) = match (devices.next(), devices.next()) {
            (Some(first), Some(second)) => (first, second),
            _ => unreachable!(),
        };

        Socat {
            process: process,
            ports: (first, second),
        }
    }

    /// Returns a pair of connected virtual serial ports
    pub fn ports(&self) -> (&Path, &Path) {
        (&self.ports.0, &self.ports.1)
    }
}

impl Drop for Socat {
    fn drop(&mut self) {
        // FIXME Destructor may panic
        self.process.signal_kill().unwrap()
    }
}
