use crate::actions::Action;

pub struct ShipV1 {
    position: (i32, i32),
    facing: i32
}

impl ShipV1 {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            facing: 0
        }
    }
    pub fn pos(&self) -> (i32, i32) {
        self.position
    }
    pub fn handle_action(&mut self, a: Action) {
        match a {
            Action::North(v) => {
                self.position.1 += v;
            }
            Action::East(v) => {
                self.position.0 += v;
            }
            Action::South(v) => {
                self.position.1 -= v;
            }
            Action::West(v) => {
                self.position.0 -= v;
            }
            Action::TurnLeft(d) => {
                self.facing -= d;
            }
            Action::TurnRight(d) => {
                self.facing += d;
            }
            Action::Forward(v) => {
                match self.facing % 360 {
                    0 => {
                        // East
                        self.position.0 += v;
                    },
                    -270 | 90 => {
                        // South
                        self.position.1 -= v;
                    }
                    -180 | 180 => {
                        // West
                        self.position.0 -= v;
                    }
                    -90 | 270 => {
                        // North
                        self.position.1 += v;
                    }
                    _ => panic!("unexpected direction: {}", self.facing)
                };
            }
        };
    }
}