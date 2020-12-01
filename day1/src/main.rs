use std::fs;

type ExpenseReportEntry = i64;

fn read_input() -> Vec<ExpenseReportEntry> {
    fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| {
            x.parse::<ExpenseReportEntry>()
                .expect("Failed to parse expense report entry")
        })
        .collect()
}

fn main() {
    let mut entries = read_input();
    entries.sort();
    println!("Part 1");
    match part1(&entries) {
        None => println!("Answer: not found"),
        Some(x) => println!("Answer: {}", x),
    };
    println!("=======================");
    println!("Part 2");
    match part2(&entries) {
        None => println!("Answer: not found"),
        Some(x) => println!("Answer: {}", x),
    };
}

fn part1(entries: &Vec<ExpenseReportEntry>) -> Option<ExpenseReportEntry> {
    for x1_idx in 0..entries.len() {
        let x1 = entries[x1_idx];
        for &x2 in &entries[(x1_idx + 1)..entries.len()] {
            if x1 + x2 == 2020 {
                return Some(x1 * x2);
            }
        }
    }
    None
}

fn part2(entries: &Vec<ExpenseReportEntry>) -> Option<ExpenseReportEntry> {
    for x1_idx in 0..entries.len() {
        let x1 = entries[x1_idx];
        for x2_idx in (x1_idx + 1)..entries.len() {
            let x2 = entries[x2_idx];
            for &x3 in &entries[(x2_idx + 1)..entries.len()] {
                if x1 + x2 + x3 == 2020 {
                    return Some(x1 * x2 * x3);
                }
            }
        }
    }
    None
}

#[test]
fn test() {
    let entries: Vec<ExpenseReportEntry> = vec![1721, 979, 366, 299, 675, 1456];

    assert_eq!(part1(&vec![]), None);
    assert_eq!(part1(&entries), Some(514579));
    assert_eq!(part2(&vec![]), None);
    assert_eq!(part2(&entries), Some(241861950));
}
