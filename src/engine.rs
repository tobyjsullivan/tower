use std::ops::{Add, Sub};
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const TICK_DURATION: Duration = Duration::from_millis(10);

pub struct Engine {
    cmd_queue: Option<Sender<Command>>,
    mx_render_state: Arc<Mutex<Option<RenderState>>>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
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

                while lag >= TICK_DURATION {
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
                    lag = lag.sub(TICK_DURATION);
                }

                // Render output
                let rs: RenderState = state.into();
                let mut ptr_render_state = mx_render_state.lock().unwrap();
                *ptr_render_state = Some(rs);
            }
        });
    }

    pub fn apply(&self, cmd: Command) {
        match &self.cmd_queue {
            Some(queue) => queue.send(cmd).unwrap(),
            None => {}
        }
    }

    pub fn get_state(&self) -> Option<RenderState> {
        let rs = self.mx_render_state.lock().unwrap();
        *rs
    }
}

#[derive(Clone, Copy)]
pub enum Command {
    AddPoint,
}

#[derive(Clone, Copy)]
pub struct RenderState {
    pub tick: u128,
    pub points: u32,
}

impl From<GameState> for RenderState {
    fn from(state: GameState) -> Self {
        RenderState {
            tick: state.tick,
            points: state.points,
        }
    }
}

#[derive(Clone, Copy)]
struct GameState {
    tick: u128,
    points: u32,
}

impl GameState {
    fn new() -> Self {
        GameState { tick: 0, points: 0 }
    }

    fn step(mut self, cmd: Option<Command>) -> Self {
        self.tick += 1;
        if let Some(cmd) = cmd {
            match cmd {
                Command::AddPoint => {
                    self.points += 1;
                }
            }
        }
        self
    }
}
