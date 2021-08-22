mod actions;
mod ship_v1;
mod ship_v2;

use actions::{Action, parse_actions};
use ship_v1::ShipV1;
use ship_v2::ShipV2;


fn manhattan_distance(p: (i32, i32)) -> i32 {
    p.0.abs() + p.1.abs()
}

fn main() {
    let actions: Vec<Action> = parse_actions(include_str!("../input.txt")).expect("parsing failed");
    
    println!();
    println!("Part 1");
    let mut ship_v1 = ShipV1::new();
    for a in &actions {
        ship_v1.handle_action(*a);
    }
    println!("Answer: {}", manhattan_distance(ship_v1.pos()));
    println!("=======================");
    println!("Part 2");
    let mut ship_v2 = ShipV2::new();
    for a in &actions {
        ship_v2.handle_action(*a);
    }
    println!("Answer: {}", manhattan_distance(ship_v2.pos()));
}


#[cfg(test)]
mod tests {
    use super::{Action, ShipV1, ShipV2, manhattan_distance};

    #[test]
    fn test_part1() {
        let mut ship = ShipV1::new();
        ship.handle_action(Action::Forward(10));
        assert_eq!((10, 0), ship.pos());
        ship.handle_action(Action::North(3));
        assert_eq!((10, 3), ship.pos());
        ship.handle_action(Action::Forward(7));
        assert_eq!((17, 3), ship.pos());
        ship.handle_action(Action::TurnRight(90));
        assert_eq!((17, 3), ship.pos());
        ship.handle_action(Action::Forward(11));
        assert_eq!((17, -8), ship.pos());

        assert_eq!(25, manhattan_distance(ship.pos()));
    }

    #[test]
    fn test_part2() {
        let mut ship = ShipV2::new();
        ship.handle_action(Action::Forward(10));
        assert_eq!((100, 10), ship.pos());
        ship.handle_action(Action::North(3));
        assert_eq!((100, 10), ship.pos());
        ship.handle_action(Action::Forward(7));
        assert_eq!((170, 38), ship.pos());
        ship.handle_action(Action::TurnRight(90));
        assert_eq!((170, 38), ship.pos());
        ship.handle_action(Action::Forward(11));
        assert_eq!((214, -72), ship.pos());

        assert_eq!(286, manhattan_distance(ship.pos()));
    }
}
