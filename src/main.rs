use std::thread;
use std::time::Duration;

mod game;

use game::{Command, Game};

fn main() {
    println!("Started.");
    let mut eng = Game::new();

    eng.start();

    for i in 0..3 {
        thread::sleep(Duration::from_millis(1900));
        eng.apply(Command::SetSpeed{speed: 1 + (i * 3600)});

        thread::sleep(Duration::from_millis(100));
        match eng.get_state() {
            Some(state) => {
                println!("************************");
                println!("Tick: {}", state.tick);
                println!("Points: {}", state.points);
                println!("Speed: {}", state.speed);
                println!("Day: {}", state.day);
                println!("Second: {}", state.second);
            }
            None => println!("No state."),
        }
    }

    println!("Done.");
}
