use std::{
    io::Write,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread::{sleep, spawn},
    time::Duration,
};

pub struct Spinner {
    out: Arc<Mutex<dyn Write + Send>>,
    frame_duration: Duration,
}

pub struct StartedSpinner {
    done_tx: Sender<()>,
}

impl Spinner {
    pub fn new(out: Arc<Mutex<dyn Write + Send>>, frame_duration: Duration) -> Spinner {
        Spinner {
            out,
            frame_duration,
        }
    }

    pub fn start(self) -> StartedSpinner {
        let (done_tx, done_rx) = mpsc::channel();
        let spinner = StartedSpinner { done_tx };
        spawn(move || {
            let mut i = 0;
            let mut out = self.out.lock().expect("Failed to lock the writer");
            while done_rx.try_recv().is_err() {
                let frame = match i % 4 {
                    0 => "|",
                    1 => "/",
                    2 => "-",
                    3 => "\\",
                    _ => unreachable!(),
                };
                let _ = write!(out, "{}\u{8}", frame);
                let _ = out.flush();
                i += 1;
                sleep(self.frame_duration);
            }
            // we're done, clear the spinner
            let _ = write!(out, "\u{8}");
            let _ = out.flush();
        });
        spinner
    }
}

impl StartedSpinner {
    pub fn stop(self) {
        self.done_tx.send(()).expect("Failed to stop spinner");
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread::sleep,
        time::Duration,
    };

    use crate::spinner::Spinner;

    #[test]
    pub fn test_spinner() {
        let out = Arc::new(Mutex::new(Vec::new()));
        let spinner = Spinner {
            out: out.clone(),
            frame_duration: Duration::from_millis(1),
        };
        let started_spinner = spinner.start();
        sleep(Duration::from_millis(50));
        started_spinner.stop();
        let out = out.lock().unwrap();
        assert_eq!(*out.first().unwrap(), '|'.to_ascii_lowercase() as u8);
        assert_eq!(*out.get(1).unwrap(), '\u{8}'.to_ascii_lowercase() as u8);
        assert_eq!(*out.get(2).unwrap(), '/'.to_ascii_lowercase() as u8);
        assert_eq!(*out.get(3).unwrap(), '\u{8}'.to_ascii_lowercase() as u8);
        assert_eq!(*out.get(4).unwrap(), '-'.to_ascii_lowercase() as u8);
        assert_eq!(*out.get(5).unwrap(), '\u{8}'.to_ascii_lowercase() as u8);
        assert_eq!(*out.get(6).unwrap(), '\\'.to_ascii_lowercase() as u8);
        assert_eq!(*out.last().unwrap(), '\u{8}'.to_ascii_lowercase() as u8);
    }
}
