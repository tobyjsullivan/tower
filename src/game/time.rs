use crate::game::Command;

pub const TICK_DURATION_MILLIS: u32 = 10;

/// Represents the current in-game time.
/// This should advance relative to real-time in accordance with game speed setting.
pub struct TimeState {
    /// Game speed as a multiple of real time.
    /// Min: 1, Max: 3600 (1 in-game hr/sec)
    pub speed: u32,

    pub day: u32,
    pub second: u32,
    millis: u32,
}

impl TimeState {
    const MILLIS_PER_SEC: u32 = 1000;
    const SECS_PER_DAY: u32 = 86400;

    pub fn new() -> Self {
        Self {
            speed: 1,

            day: 0,
            second: 0,
            millis: 0,
        }
    }

    pub fn step(mut self, _cmd: Option<Command>) -> Self {
        self.advance(TICK_DURATION_MILLIS * self.speed);
        self
    }

    fn advance(&mut self, millis: u32) {
        self.millis += millis;

        while self.millis >= Self::MILLIS_PER_SEC {
            self.second += 1;
            self.millis -= Self::MILLIS_PER_SEC;
        }

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
    fn test_advance_second() {
        let mut time = TimeState {
            speed: 1,

            day: 2,
            second: 14,
            millis: 650,
        };

        time.advance(500);

        assert_eq!(time.day, 2);
        assert_eq!(time.second, 15);
        assert_eq!(time.millis, 150);
    }

    #[test]
    fn test_advance_day() {
        let mut time = TimeState {
            speed: 1,

            day: 3,
            second: 86399,
            millis: 995,
        };

        time.advance(10);

        assert_eq!(time.day, 4);
        assert_eq!(time.second, 0);
        assert_eq!(time.millis, 5);
    }

    #[test]
    fn test_speed() {
        let mut time = TimeState {
            speed: 30,

            day: 2,
            second: 14,
            millis: 650,
        };

        time = time.step(None);

        assert_eq!(time.day, 2);
        assert_eq!(time.second, 14);
        assert_eq!(time.millis, 950);
    }
}
