fn parse(s: &str) -> Vec<u64> {
    s.split("\n")
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn main() {
    let input = parse(include_str!("input.txt"));

    println!();
    println!("Part 1");
    let part1_answer = part1(&input, 25).unwrap();
    println!("Answer: {}", part1_answer);
    println!("=======================");
    println!("Part 2");
    let part2_answer = part2(&input, part1_answer).unwrap();
    println!("Answer: {}", part2_answer);
}

fn part1(buffer: &[u64], preamble_size: usize) -> Option<u64> {
    for i in preamble_size..buffer.len() {
        let n = buffer[i];
        let preamble = get_preamble(buffer, i, preamble_size);
        if !any_match(preamble, n) {
            return Some(n);
        }
    }
    None
}

fn get_preamble(buffer: &[u64], offset: usize, size: usize) -> &[u64] {
    &buffer[(offset - size)..offset]
}

fn any_match(buffer: &[u64], target: u64) -> bool {
    for (idx, n) in buffer.iter().enumerate() {
        for m in buffer[(idx + 1)..].iter() {
            if n + m == target && n != m {
                return true;
            }
        }
    }
    false
}

fn part2(buffer: &[u64], target: u64) -> Option<u64> {
    for i in 0..buffer.len() {
        let mut sum: u64 = 0;
        for (i2, &n) in buffer[i..].iter().enumerate() {
            let next_sum = sum + n;
            if next_sum == target {
                let range = &buffer[i..=(i + i2)];
                let max = range.iter().max().unwrap();
                let min = range.iter().min().unwrap();
                return Some(max + min);
            }
            if next_sum > target {
                break;
            }
            sum = next_sum;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    fn inputs() -> Vec<u64> {
        super::parse(
            "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
        )
    }
    #[test]
    fn test_part1() {
        let input = inputs();
        assert_eq!(Some(127), super::part1(&input, 5));
    }

    #[test]
    fn test_part2() {
        let input = inputs();
        assert_eq!(Some(62), super::part2(&input, 127));
    }
}
