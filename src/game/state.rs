use std::time::Duration;

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
        self.time.advance(TICK_DURATION_MILLIS);
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

#[derive(Clone, Copy)]
/// Represents the current in-game time.
/// This should advance relative to real-time in accordance with game speed setting.
pub struct TimeState {
    pub day: u32,
    pub second: u32,
    next_second: u32,
}

impl TimeState {
    const MILLIS_PER_SEC: u32 = 1000;
    const SECS_PER_DAY: u32 = 86400;

    fn new() -> Self {
        Self {
            day: 0,
            second: 0,
            next_second: Self::MILLIS_PER_SEC,
        }
    }

    fn advance(&mut self, mut millis: u32) {
        while self.next_second <= millis {
            self.second += 1;
            millis -= self.next_second;
            self.next_second = Self::MILLIS_PER_SEC;
        }

        self.next_second -= millis;

        while self.second >= Self::SECS_PER_DAY {
            self.day += 1;
            self.second -= Self::SECS_PER_DAY;
        }
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
    fn test_time_advance_second() {
        let mut state = GameState::new();
        state.time.advance(995);

        state = state.step(None);

        assert_eq!(state.time.day, 0);
        assert_eq!(state.time.second, 1);
    }

    #[test]
    fn test_time_advance_day() {
        let mut state = GameState::new();
        state.time = TimeState {
            day: 3,
            second: 86399,
            next_second: 5,
        };

        state = state.step(None);

        assert_eq!(state.time.day, 4);
        assert_eq!(state.time.second, 0);
    }
}
