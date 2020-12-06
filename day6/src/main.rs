use std::collections::HashSet;

type AnswerSet = HashSet<char>;

type GroupAnswers = Vec<AnswerSet>;

fn parse_input(text: &str) -> Vec<GroupAnswers> {
    text.trim()
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|line| line.chars().collect::<AnswerSet>())
                .collect::<GroupAnswers>()
        })
        .collect::<Vec<GroupAnswers>>()
}

fn main() {
    let group_answers = parse_input(&prelude::read_input("input.txt"));
    println!();
    println!("Part 1");
    println!("Answer: {}", part1(&group_answers));
    println!("=======================");
    println!("Part 2");
    println!("Answer: {}", part2(&group_answers));
}

fn part1(answers: &Vec<GroupAnswers>) -> usize {
    answers
        .iter()
        .map(|x| x.iter().flatten().copied().collect::<HashSet<char>>().len())
        .sum()
}

fn part2(answers: &Vec<GroupAnswers>) -> usize {
    answers
        .iter()
        .map(|group| {
            group[1..]
                .iter()
                .fold(group[0].clone(), |c, n| {
                    c.intersection(n).copied().collect()
                })
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    fn get_inputs() -> Vec<super::GroupAnswers> {
        super::parse_input(
            "abc

a
b
c

ab
ac

a
a
a
a

b
",
        )
    }
    #[test]
    fn test_part1() {
        let group_answers = get_inputs();
        assert_eq!(11, super::part1(&group_answers));
    }

    #[test]
    fn test_part2() {
        let group_answers = get_inputs();
        assert_eq!(6, super::part2(&group_answers));
    }
}
