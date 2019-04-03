use std::time::Duration;

use super::time::TimeState;
use crate::game::Command;

const TICK_DURATION_MILLIS: u32 = 10;
pub const TICK_DURATION: Duration = Duration::from_millis(TICK_DURATION_MILLIS as u64);

#[derive(Clone, Copy)]
pub struct GameState {
    pub points: u32,
    /// Game speed as a multiple of real time.
    /// Min: 1, Max: 3600 (1 in-game hr/sec)
    pub speed: u32,
    pub tick: u128,

    pub time: TimeState,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            points: 0,
            speed: 1,
            tick: 0,
            time: TimeState::new(),
        }
    }

    pub fn step(mut self, cmd: Option<Command>) -> Self {
        self.tick += 1;
        self.time.advance(TICK_DURATION_MILLIS * self.speed);
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
        state.time.advance(995);

        state = state.step(None);

        assert_eq!(state.time.day, 0);
        assert_eq!(state.time.second, 1);
    }

    #[test]
    fn test_step_advances_time_at_speed() {
        let mut state = GameState::new();
        state.speed = 6;
        state.time.advance(950);

        state = state.step(None);

        assert_eq!(state.time.day, 0);
        assert_eq!(state.time.second, 1);
    }
}
