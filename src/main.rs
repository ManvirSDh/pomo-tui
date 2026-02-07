use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let time = Instant::now();
    let mut counter = 0;
    loop {
        println!("Time is {}", time.elapsed().as_secs());
        if time.elapsed() > Duration::new(counter, 0) {
            counter += 1;
        }
        if counter > 10 {
            break;
        }
        sleep(Duration::from_millis(1000));
    }
}
