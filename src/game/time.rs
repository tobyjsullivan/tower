#[derive(Clone, Copy)]
/// Represents the current in-game time.
/// This should advance relative to real-time in accordance with game speed setting.
pub struct TimeState {
    pub day: u32,
    pub second: u32,
    pub millis: u32,
}

impl TimeState {
    const MILLIS_PER_SEC: u32 = 1000;
    const SECS_PER_DAY: u32 = 86400;

    pub fn new() -> Self {
        Self {
            day: 0,
            second: 0,
            millis: 0,
        }
    }

    pub fn advance(&mut self, millis: u32) {
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
    fn test_time_advance_day() {
        let mut time = TimeState {
            day: 3,
            second: 86399,
            millis: 995,
        };

        time.advance(10);

        assert_eq!(time.day, 4);
        assert_eq!(time.second, 0);
        assert_eq!(time.millis, 5);
    }
}
