use std::collections::HashMap;

type Joltage = u8;

fn parse(s: &str) -> Vec<Joltage> {
    let mut joltages: Vec<Joltage> = s
        .split("\n")
        .into_iter()
        .map(|s| s.replace("\r", "").parse::<Joltage>().unwrap())
        .collect();
    joltages.sort_unstable();
    joltages
}

fn part1(inputs: &[Joltage]) -> usize {
    let mut d1 = 0;
    let mut d3 = 1;
    let mut current: Joltage = 0;
    for &x in inputs {
        match x - current {
            0 | 2 => {}
            1 => d1 += 1,
            3 => d3 += 1,
            _ => break,
        }
        current = x;
    }

    d1 * d3
}

fn part2(inputs: &[Joltage]) -> usize {
    const DELTAS: [u8; 3] = [1, 2, 3];
    let target = inputs.iter().copied().max().unwrap() + 3;
    let mut map: HashMap<Joltage, usize> = HashMap::new();
    map.insert(target, 1);

    for &n in inputs.iter().rev() {
        map.insert(n, DELTAS.iter().filter_map(|d| map.get(&(d + n))).sum());
    }
    DELTAS.iter().filter_map(|m| map.get(m)).sum()
}

fn main() {
    let input = parse(include_str!("input.txt"));

    println!();
    println!("Part 1");
    let part1_answer = part1(&input);
    println!("Answer: {}", part1_answer);
    println!("=======================");
    println!("Part 2");
    let part2_answer = part2(&input);
    println!("Answer: {}", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::Joltage;
    fn inputs1() -> Vec<Joltage> {
        super::parse(
            "16
10
15
5
1
11
7
19
6
12
4",
        )
    }
    fn inputs2() -> Vec<Joltage> {
        super::parse(
            "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3",
        )
    }

    #[test]
    fn part1_test() {
        let inputs1 = inputs1();
        let inputs2 = inputs2();
        assert_eq!(35, super::part1(&inputs1));
        assert_eq!(220, super::part1(&inputs2));
    }
    #[test]
    fn part2_test() {
        let inputs1 = inputs1();
        let inputs2 = inputs2();
        assert_eq!(8, super::part2(&inputs1));
        assert_eq!(19208, super::part2(&inputs2));
    }
}
