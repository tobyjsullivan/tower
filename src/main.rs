use std::thread;
use std::time::Duration;

mod engine;

use engine::{Command, Engine};

fn main() {
    println!("Hello, world!");
    let mut eng = Engine::new();

    eng.start();

    for _ in 0..3 {
        thread::sleep(Duration::from_secs(2));
        eng.apply(Command::AddPoint);

        match eng.get_state() {
            Some(state) => {
                println!("Tick: {}", state.tick);
                println!("Points: {}", state.points);
            }
            None => println!("No state."),
        }
    }

    println!("Done.");
}
