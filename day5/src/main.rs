#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SeatPosition {
    row: u8,
    column: u8,
}

impl SeatPosition {
    fn from_partition_code(code: &str) -> Option<Self> {
        if code.chars().count() != 10 {
            return None;
        }
        let row: u8 = code[..7]
            .chars()
            .enumerate()
            .map(|(idx, c)| match c {
                'B' => (2 as u8).pow(6 - idx as u32),
                'F' => 0,
                _ => panic!("unexpected row code: {:?}", c),
            })
            .sum();
        let column: u8 = code[7..]
            .chars()
            .enumerate()
            .map(|(idx, c)| match c {
                'R' => (2 as u8).pow(2 - idx as u32),
                'L' => 0,
                _ => panic!("unexpected column code: {:?}", c),
            })
            .sum();
        Some(Self { row, column })
    }
    fn id(&self) -> u32 {
        (self.row as u32) * 8 + (self.column as u32)
    }
}

fn part1(seats: &Vec<SeatPosition>) -> usize {
    seats.iter().map(|x| x.id() as usize).max().unwrap()
}

fn part2(seats: &Vec<SeatPosition>) -> usize {
    for (prev_idx, seat) in seats[1..seats.len() - 1].iter().enumerate() {
        let prev = &seats[prev_idx];
        if prev.id() + 1 != seat.id() {
            return prev.id() as usize + 1;
        }
    }
    panic!("seat not found")
}

fn print_seat_grid(seats: &Vec<SeatPosition>) {
    let mut grid: [[bool; 8]; 128] = [[false; 8]; 128];
    for seat in seats {
        grid[seat.row as usize][seat.column as usize] = true;
    }
    for (row_idx, row) in grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().any(|&x| x))
    {
        print!("{:03}: ", row_idx);
        for &col in row.iter() {
            if col {
                print!("T")
            } else {
                print!("_")
            }
        }
        println!()
    }
}

fn main() {
    let mut seats: Vec<SeatPosition> = prelude::read_input_lines("input.txt")
        .into_iter()
        .map(|seat_code| SeatPosition::from_partition_code(&seat_code).unwrap())
        .collect();
    seats.sort();

    print_seat_grid(&seats);
    println!();
    println!("Part 1");
    println!("Answer: {}", part1(&seats));
    println!("=======================");
    println!("Part 2");
    println!("Answer: {}", part2(&seats));
}

#[cfg(test)]
mod tests {
    use super::SeatPosition;
    #[test]
    fn part1() {
        let test_data = vec![
            ("BFFFBBFRRR", (70, 7, 567)),
            ("FFFBBBFRRR", (14, 7, 119)),
            ("BBFFBBFRLL", (102, 4, 820)),
        ];
        for (seat_code, (row, column, id)) in test_data {
            let seat = SeatPosition::from_partition_code(seat_code).unwrap();
            assert_eq!(row, seat.row);
            assert_eq!(column, seat.column);
            assert_eq!(id, seat.id());
        }
    }
}
