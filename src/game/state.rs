use crate::game::Command;

#[derive(Clone, Copy)]
pub struct GameState {
    pub tick: u128,
    pub points: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState { tick: 0, points: 0 }
    }

    pub fn step(mut self, cmd: Option<Command>) -> Self {
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
