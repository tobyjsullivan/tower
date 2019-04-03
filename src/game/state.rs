use std::time::Duration;

use super::time::{TimeState, TICK_DURATION_MILLIS};
use crate::game::Command;

pub const TICK_DURATION: Duration = Duration::from_millis(TICK_DURATION_MILLIS as u64);

pub struct GameState {
    pub points: u32,
    pub tick: u128,

    pub time: TimeState,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            points: 0,
            tick: 0,
            time: TimeState::new(),
        }
    }

    pub fn step(mut self, cmd: Option<Command>) -> Self {
        self.tick += 1;
        self.time = self.time.step(cmd);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_point() {
        let mut state = GameState::new();
        state.points = 5;

        state = state.step(Some(Command::AddPoint));

        assert_eq!(state.points, 6);
    }

    #[test]
    fn test_step_advances_time() {
        let mut state = GameState::new();

        for _ in 0..200 {
            state = state.step(None);
        }

        assert_eq!(state.time.day, 0);
        assert_eq!(state.time.second, 2);
    }
}
