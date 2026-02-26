use notify_rust::Notification;
use rodio::Decoder;
use std::env::args;
use std::fs::File;
use std::thread::sleep;
use std::time::{Duration, Instant};

//
// CLI Call:
// timer {first_timer_length} {first_timer_name(opt)} {second_timer_length (opt)} {second_timer_name(opt)}
fn main() {
    // Input Handling
    let mut times: Vec<u64> = vec![];
    let mut titles: Vec<String> = vec![];
    {
        let args: Vec<String> = args().collect();
        println!("Args are: {:?}", args);
        println!("Length is: {}", args.len());
        if args.len() > 1 {
            let mut added_timer_prev: bool = false;
            for arg in args[1..].iter() {
                match arg.parse::<f64>() {
                    Ok(timer_val) => {
                        if added_timer_prev {
                            titles.push("Unnamed".to_string());
                        }
                        times.push((timer_val * 60.0) as u64);
                        added_timer_prev = true;
                    }
                    Err(_) => {
                        if added_timer_prev {
                            added_timer_prev = false;
                            titles.push(arg.clone());
                        } else {
                            panic!(
                                "Expected a numeric input, received: {}. \n
                            Titles are only valid after timer duration.",
                                arg
                            );
                        }
                    }
                }
            }
            if added_timer_prev {
                titles.push("Unnamed".to_string());
            }
        } else {
            times.push(60);
            titles.push("Default".to_string());
        }
        println!("Timer lengths: {:?}", times);
        println!("Timer names: {:?}", titles);
    }

    // Timer loop
    //
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    loop {
        let mut timer: Instant;

        for (index, length) in times.iter().enumerate() {
            timer = Instant::now();

            println!("Time and name: {} {}", length, titles[index]);
            while timer.elapsed() < Duration::new(*length, 0) {
                sleep(Duration::from_millis(1000));
                println!("Time elapsed: {}", timer.elapsed().as_secs());
            }

            //End of Timer
            println!("Timer done!");

            let file = File::open("assets/song208.mp3").unwrap();
            let source = Decoder::try_from(file).unwrap();
            stream_handle.mixer().add(source);

            Notification::new()
                .appname("CLI Pomodoro!")
                .summary(&format!("{} timer ended!", titles[index]))
                .body("The timer has ended - Time to switch!")
                .show()
                .expect("Failed to show notification");
        }
    }
}
