use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::spinner::Spinner;

mod spinner;

fn main() {
    let spinner = Spinner::new(
        Arc::new(Mutex::new(std::io::stdout())),
        Duration::from_millis(100),
    );
    println!("start spinning");
    let spinner = spinner.start();
    sleep(Duration::from_secs(3));
    spinner.stop();
    println!("done spinning");
}
