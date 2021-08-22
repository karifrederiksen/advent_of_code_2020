#[derive(Copy, Debug, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    SeatEmpty,
    SeatOccupied,
}

type Grid = Vec<Vec<Tile>>;

type Change = (Tile, usize, usize);

fn parse(s: &str) -> Grid {
    s.split("\n")
        .map(|s| {
            s.replace("\r", "").chars()
                .map(|c| match c {
                    '.' => Tile::Floor,
                    'L' => Tile::SeatEmpty,
                    '#' => Tile::SeatOccupied,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

const ADJACENCY_OFFSET: [(i16, i16); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn get_adjacent_part1(grid: &Grid, x: usize, y: usize) -> Vec<Tile> {
    ADJACENCY_OFFSET
        .iter()
        .flat_map(|&(dx, dy)| {
            grid.get((y as i16 + dy) as usize)
                .map(|line| line.get((x as i16 + dx) as usize).copied())
                .flatten()
        })
        .collect()
}

fn any_adjacent_part1(grid: &Grid, x: usize, y: usize) -> bool {
    get_adjacent_part1(grid, x, y)
        .into_iter()
        .any(|t| t == Tile::SeatOccupied)
}

fn min_adjacent_part1(grid: &Grid, x: usize, y: usize, n: usize) -> bool {
    get_adjacent_part1(grid, x, y)
        .into_iter()
        .filter(|&t| t == Tile::SeatOccupied)
        .count()
        >= n
}

fn step_part1(grid: &Grid) -> Vec<Change> {
    let mut changes = Vec::new();
    let width = grid[0].len();
    for y in 0..grid.len() {
        for x in 0..width {
            match grid[y][x] {
                Tile::Floor => {}
                Tile::SeatEmpty => {
                    if !any_adjacent_part1(grid, x, y) {
                        changes.push((Tile::SeatOccupied, x, y));
                    }
                }
                Tile::SeatOccupied => {
                    if min_adjacent_part1(grid, x, y, 4) {
                        changes.push((Tile::SeatEmpty, x, y));
                    }
                }
            }
        }
    }
    changes
}

fn get_adjacent_part2(grid: &Grid, x: usize, y: usize) -> Vec<Tile> {
    ADJACENCY_OFFSET
        .iter()
        .flat_map(|&direction| any_adjacent_part2b(grid, x as i16, y as i16, direction))
        .collect()
}

fn any_adjacent_part2(grid: &Grid, x: usize, y: usize) -> bool {
    get_adjacent_part2(grid, x, y)
        .into_iter()
        .any(|t| t == Tile::SeatOccupied)
}

fn any_adjacent_part2b(grid: &Grid, x: i16, y: i16, (dx, dy): (i16, i16)) -> Option<Tile> {
    let x = x + dx;
    let y = y + dy;
    if x < 0 || y < 0 {
        return None;
    }
    match grid
        .get(y as usize)
        .map(|line| line.get(x as usize).copied())
        .flatten()
    {
        Some(Tile::Floor) => any_adjacent_part2b(grid, x, y, (dx, dy)),
        a => a,
    }
}

fn min_adjacent_part2(grid: &Grid, x: usize, y: usize, n: usize) -> bool {
    get_adjacent_part2(grid, x, y)
        .into_iter()
        .filter(|&t| t == Tile::SeatOccupied)
        .count()
        >= n
}

fn step_part2(grid: &Grid) -> Vec<Change> {
    let mut changes = Vec::new();
    let width = grid[0].len();
    for y in 0..grid.len() {
        for x in 0..width {
            match grid[y][x] {
                Tile::Floor => {}
                Tile::SeatEmpty => {
                    if !any_adjacent_part2(grid, x, y) {
                        changes.push((Tile::SeatOccupied, x, y));
                    }
                }
                Tile::SeatOccupied => {
                    if min_adjacent_part2(grid, x, y, 5) {
                        changes.push((Tile::SeatEmpty, x, y));
                    }
                }
            }
        }
    }
    changes
}

fn apply_change_set(grid: &mut Grid, change_set: Vec<Change>) {
    for (tile, x, y) in change_set {
        grid[y][x] = tile;
    }
}

fn occupied_seats(grid: &Grid) -> usize {
    grid.iter()
        .map(|line| line.iter().filter(|&&t| t == Tile::SeatOccupied).count())
        .sum()
}

fn part1(grid: &mut Grid) -> usize {
    loop {
        let changes = step_part1(grid);
        if changes.len() == 0 {
            break;
        }
        apply_change_set(grid, changes);
    }
    occupied_seats(grid)
}

fn part2(grid: &mut Grid) -> usize {
    loop {
        let changes = step_part2(grid);
        if changes.len() == 0 {
            break;
        }
        apply_change_set(grid, changes);
    }
    occupied_seats(grid)
}

fn main() {
    let grid: Grid = parse(include_str!("input.txt"));

    println!();
    println!("Part 1");
    let mut grid_part1 = grid.clone();
    let part1_answer = part1(&mut grid_part1);
    println!("Answer: {}", part1_answer);
    println!("=======================");
    println!("Part 2");
    let mut grid_part2 = grid.clone();
    let part2_answer = part2(&mut grid_part2);
    println!("Answer: {}", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::{Grid, Tile};

    fn unparse_line(line: &Vec<Tile>) -> String {
        line.iter()
            .map(|&t| match t {
                Tile::Floor => '.',
                Tile::SeatEmpty => 'L',
                Tile::SeatOccupied => '#',
            })
            .collect()
    }

    fn inputs() -> Grid {
        super::parse(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        )
    }
    #[test]
    fn part1_test() {
        let mut grid = inputs();

        let step_data = [
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
            "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
            "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
            "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
            "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
        ];

        for (step_idx, step_grid) in step_data.iter().enumerate() {
            let changes = super::step_part1(&grid);
            assert_ne!(0, changes.len());
            super::apply_change_set(&mut grid, changes);
            for (line_idx, (line_orig, line_test)) in
                grid.iter().zip(step_grid.split("\n")).enumerate()
            {
                assert_eq!(
                    unparse_line(line_orig),
                    line_test,
                    "after step {} line {}",
                    step_idx + 1,
                    line_idx
                );
            }
        }

        assert_eq!(37, super::occupied_seats(&grid));
    }
    #[test]
    fn part2_test() {
        let mut grid = inputs();

        let step_data = [
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
            "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
            "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#",
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#",
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
        ];

        for (step_idx, step_grid) in step_data.iter().enumerate() {
            let changes = super::step_part2(&grid);
            assert_ne!(0, changes.len());
            super::apply_change_set(&mut grid, changes);
            for (line_idx, (line_orig, line_test)) in
                grid.iter().zip(step_grid.split("\n")).enumerate()
            {
                assert_eq!(
                    unparse_line(line_orig),
                    line_test,
                    "after step {} line {}",
                    step_idx + 1,
                    line_idx
                );
            }
        }

        assert_eq!(26, super::occupied_seats(&grid));
    }
}
