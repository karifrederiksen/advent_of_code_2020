use crate::actions::Action;

pub struct ShipV2 {
    position: (i32, i32),
    waypoint: (i32, i32)
}

impl ShipV2 {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            waypoint: (10, 1)
        }
    }
    pub fn pos(&self) -> (i32, i32) {
        self.position
    }
    pub fn handle_action(&mut self, a: Action) {
        match a {
            Action::North(v) => {
                self.waypoint.1 += v;
            }
            Action::East(v) => {
                self.waypoint.0 += v;
            }
            Action::South(v) => {
                self.waypoint.1 -= v;
            }
            Action::West(v) => {
                self.waypoint.0 -= v;
            }
            Action::TurnLeft(d) => {
                Self::rotate(&mut self.waypoint, -d);
            }
            Action::TurnRight(d) => {
                Self::rotate(&mut self.waypoint, d);
            }
            Action::Forward(v) => {
                self.position.0 += self.waypoint.0 * v;
                self.position.1 += self.waypoint.1 * v;
            }
        };
    }

    fn rotate(w: &mut (i32, i32), d: i32) {
        match d % 360 {
            0 => {},
            -270 | 90 => {
                let t = -w.0;
                w.0 = w.1;
                w.1 = t;
            }
            -180 | 180 => {
                w.0 = -w.0;
                w.1 = -w.1;
            }
            -90 | 270 => {
                let t = -w.1;
                w.1 = w.0;
                w.0 = t;
            }
            _ => panic!("unexpected rotation: {}", d)
        }
    }
}