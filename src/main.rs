use std::thread;
use std::time::Duration;

mod game;

use game::{Command, Game};

fn main() {
    println!("Started.");
    let mut eng = Game::new();

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
