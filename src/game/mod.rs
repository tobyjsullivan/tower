use std::ops::{Add, Sub};
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod state;
mod time;

use state::GameState;

const TICK_DURATION_MILLIS: u32 = 10;

pub struct Game {
    cmd_queue: Option<Sender<Command>>,
    mx_render_state: Arc<Mutex<Option<RenderState>>>,
}

impl Game {
    const TICK_DURATION: Duration = Duration::from_millis(TICK_DURATION_MILLIS as u64);

    pub fn new() -> Self {
        Self {
            cmd_queue: None,
            mx_render_state: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&mut self) {
        let (sender, receiver) = channel();
        self.cmd_queue = Some(sender);

        let mx_render_state = Arc::clone(&self.mx_render_state);

        thread::spawn(move || {
            let mut state = GameState::new();
            let mut lag = Duration::from_secs(0);
            let mut last = Instant::now();
            loop {
                let current = Instant::now();
                let elapsed = current.duration_since(last);
                last = current;
                lag = lag.add(elapsed);

                while lag >= Self::TICK_DURATION {
                    // Read a command if present
                    let cmd = match receiver.try_recv() {
                        Ok(cmd) => Some(cmd),
                        Err(TryRecvError::Empty) => None,
                        Err(TryRecvError::Disconnected) => {
                            // Time to close.
                            break;
                        }
                    };

                    // Run update
                    state = state.step(cmd);
                    lag = lag.sub(Self::TICK_DURATION);
                }

                // Render output
                let rs: RenderState = (&state).into();
                let mut ptr_render_state = mx_render_state.lock().unwrap();
                *ptr_render_state = Some(rs);
            }
        });
    }

    pub fn apply(&self, cmd: Command) {
        match &self.cmd_queue {
            Some(queue) => queue.send(cmd).expect("Failed to send command over queue."),
            None => panic!("Attempted to apply command to uninitialised game."),
        }
    }

    pub fn get_state(&self) -> Option<RenderState> {
        let rs = self.mx_render_state.lock().expect("Attempted to get game state but none available.");
        *rs
    }
}

#[derive(Clone, Copy)]
pub enum Command {
    AddPoint,
    SetSpeed{ speed: u32 },
}

/// The view of the world exposed by the game API. The RenderState should only include information
/// which is known by the player. Hidden state should be kept within GameState.
#[derive(Clone, Copy)]
pub struct RenderState {
    pub tick: u128,
    pub points: u32,
    pub speed: u32,
    pub day: u32,
    pub second: u32,
}

impl From<&GameState> for RenderState {
    fn from(state: &GameState) -> Self {
        RenderState {
            tick: state.tick,
            points: state.points,
            speed: state.time.speed,
            day: state.time.day,
            second: state.time.second,
        }
    }
}
