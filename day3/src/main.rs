use prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TileType {
    Clear,
    Tree,
}

struct Terrain {
    width: usize,
    height: usize,
    tiles: Vec<TileType>,
}

impl Terrain {
    fn from_lines(lines: Vec<String>) -> Self {
        let width = match lines.get(0) {
            Some(x) => x.len(),
            None => 0,
        };
        let height = lines.len();
        let mut tiles: Vec<TileType> = vec![TileType::Clear; width * height];

        for (y, line) in lines.into_iter().enumerate() {
            for (x, tile_ch) in line.chars().enumerate() {
                let idx = y * width + x;
                tiles[idx] = match tile_ch {
                    '.' => TileType::Clear,
                    '#' => TileType::Tree,
                    _ => panic!("Unexpected tile type: {:?}", tile_ch),
                };
            }
        }
        Self {
            width,
            height,
            tiles,
        }
    }

    fn tile(&self, x: usize, y: usize) -> TileType {
        let x = x % self.width;
        let idx = y * self.width + x;
        *self.tiles.get(idx).unwrap_or(&TileType::Clear)
    }

    fn height(&self) -> usize {
        self.height
    }

    fn path_for_slope(&self, slope_x: usize, slope_y: usize) -> Vec<TileType> {
        let mut tiles: Vec<TileType> = Vec::new();
        let mut x = 0;
        let mut y = 0;
        while y <= self.height() {
            tiles.push(self.tile(x, y));
            x += slope_x;
            y += slope_y;
        }
        tiles
    }
}

fn part1(terrain: &Terrain) -> usize {
    terrain
        .path_for_slope(3, 1)
        .into_iter()
        .filter(|&x| x == TileType::Tree)
        .count()
}

fn part2(terrain: &Terrain) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .into_iter()
        .map(|(x, y)| {
            terrain
                .path_for_slope(x, y)
                .into_iter()
                .filter(|&x| x == TileType::Tree)
                .count()
        })
        .fold(1, |state, next| state * next)
}

fn main() {
    let terrain = Terrain::from_lines(read_input_lines("input.txt"));

    println!("Part 1");
    println!("Answer: {}", part1(&terrain));
    println!("=======================");
    println!("Part 2");
    println!("Answer: {}", part2(&terrain));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, Terrain};

    fn get_terrain() -> Terrain {
        let input: Vec<String> = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            .split("\n")
            .map(|x| x.to_string())
            .collect();
        Terrain::from_lines(input)
    }

    #[test]
    fn test_part1() {
        let terrain = get_terrain();
        assert_eq!(7, part1(&terrain));
    }
    #[test]
    fn test_part2() {
        let terrain = get_terrain();
        assert_eq!(336, part2(&terrain));
    }
}
