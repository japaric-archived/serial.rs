use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

/// Wrapper around a child `socat` process
pub struct Socat {
    child: Child,
    ports: (PathBuf, PathBuf),
}

impl Socat {
    /// Spawns `socat`
    pub fn new() -> Socat {
        let mut child =
            Command::new("socat").
                args(&["-d", "-d", "pty", "pty"]).
                stderr(Stdio::piped()).
                spawn().
                ok().expect("Couldn't find `socat`");

        let stderr = BufReader::new(child.stderr.take().unwrap());
        let mut devices = stderr.lines().map(|line| {
                line.
                    ok().expect("Couldn't read `socat` stderr").
                    trim().
                    split(" is ").
                    skip(1).
                    next().expect("Couldn't parse serial device").
                    to_string()
        });

        // FIXME Reading stderr may block indefinitely if `socat` fails
        let (first, second) = match (devices.next(), devices.next()) {
            (Some(first), Some(second)) => (first, second),
            _ => unreachable!(),
        };

        Socat {
            child: child,
            ports: (PathBuf::new(&first), PathBuf::new(&second)),
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
        self.child.kill().unwrap()
    }
}
