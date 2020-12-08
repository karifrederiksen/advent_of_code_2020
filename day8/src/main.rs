use nom::{
    branch,
    bytes::complete,
    character::complete::{digit1, newline, space1},
    combinator, multi, sequence, IResult,
};
use smallvec::SmallVec;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
enum Instr {
    Nop(i16),
    Acc(i16),
    Jmp(i16),
}

#[derive(Clone, Copy)]
enum Sign {
    Plus,
    Minus,
}

fn parse_sign(input: &str) -> IResult<&str, Sign> {
    branch::alt((
        combinator::value(Sign::Plus, complete::tag("+")),
        combinator::value(Sign::Minus, complete::tag("-")),
    ))(input)
}

fn parse_signed_i16(input: &str) -> IResult<&str, i16> {
    combinator::map(
        sequence::tuple((
            parse_sign,
            combinator::map_res(digit1, |n: &str| n.parse::<i16>()),
        )),
        |(sign, x)| match sign {
            Sign::Plus => x,
            Sign::Minus => -x,
        },
    )(input)
}

fn parse_jmp(input: &str) -> IResult<&str, Instr> {
    combinator::map(
        sequence::preceded(
            sequence::pair(complete::tag("jmp"), space1),
            parse_signed_i16,
        ),
        Instr::Jmp,
    )(input)
}

fn parse_acc(input: &str) -> IResult<&str, Instr> {
    combinator::map(
        sequence::preceded(
            sequence::pair(complete::tag("acc"), space1),
            parse_signed_i16,
        ),
        Instr::Acc,
    )(input)
}

fn parse_nop(input: &str) -> IResult<&str, Instr> {
    combinator::map(
        sequence::preceded(
            sequence::pair(complete::tag("nop"), space1),
            parse_signed_i16,
        ),
        Instr::Nop,
    )(input)
}

fn parse_instrs(input: &str) -> IResult<&str, Vec<Instr>> {
    multi::separated_list0(newline, branch::alt((parse_jmp, parse_acc, parse_nop)))(input)
}

type SVecI16 = SmallVec<[i16; 8]>;

fn find_faulty_instr(instrs: &[Instr], has_visited: Vec<bool>) -> usize {
    let can_reach_end: Vec<i16> = {
        let mut jmp_from_table: Vec<(SVecI16, SVecI16)> = Vec::with_capacity(instrs.len() + 1);
        for _ in 0..=instrs.len() {
            jmp_from_table.push((SVecI16::new(), SVecI16::new()));
        }
        for (from_idx, (jmp, swapped_jmp)) in instrs
            .iter()
            .map(|ins| match ins {
                Instr::Acc(_) => (1, 1),
                Instr::Nop(n) => (1, *n),
                Instr::Jmp(n) => (*n, 1),
            })
            .chain(std::iter::once((0, 0)))
            .enumerate()
        {
            let from_idx = from_idx as i16;
            let to_idx_normal = from_idx + jmp;
            let to_idx_swapped = from_idx + swapped_jmp;
            jmp_from_table[to_idx_normal as usize].0.push(from_idx);
            jmp_from_table[to_idx_swapped as usize].1.push(from_idx);
        }
        let mut can_reach_end: Vec<i16> = jmp_from_table
            .iter()
            .last()
            .unwrap()
            .0
            .iter()
            .copied()
            .collect();
        let mut processing_idx = 0usize;
        while processing_idx < can_reach_end.len() {
            let idx = can_reach_end[processing_idx];
            for reachable_from in &jmp_from_table[idx as usize].0 {
                if !can_reach_end.contains(reachable_from) {
                    can_reach_end.push(*reachable_from);
                }
            }
            processing_idx += 1;
        }
        can_reach_end
    };

    // println!("can_reach_end: {}", can_reach_end.len());
    // Some node in `has_visited` should connect to `can_reach_end` when we flip its type
    for idx in has_visited
        .into_iter()
        .enumerate()
        .filter(|(_, v)| *v)
        .map(|(idx, _)| idx)
    {
        match instrs[idx] {
            Instr::Nop(n) => {
                if can_reach_end.contains(&(idx as i16 + n)) {
                    return idx;
                }
            }
            Instr::Jmp(_) => {
                if can_reach_end.contains(&(idx as i16 + 1)) {
                    return idx;
                }
            }
            _ => {}
        }
    }
    panic!("faulty instr not found")
}

fn run(instrs: &[Instr]) -> (bool, i64, Vec<bool>) {
    let mut has_visited: Vec<bool> = vec![false; instrs.len()];
    let mut accumulator = 0i64;
    let mut instr_pointer = 0i32;

    while let Some(&instr) = instrs.get(instr_pointer as usize) {
        if has_visited[instr_pointer as usize] {
            return (false, accumulator, has_visited);
        }
        has_visited[instr_pointer as usize] = true;
        match instr {
            Instr::Acc(n) => {
                accumulator += n as i64;
            }
            Instr::Jmp(n) => {
                instr_pointer = instr_pointer + n as i32 - 1;
            }
            Instr::Nop(_) => {}
        }
        instr_pointer += 1;
    }
    (true, accumulator, has_visited)
}

fn part2(instrs: &mut [Instr]) -> i64 {
    let start = Instant::now();

    let (_, _, has_visited) = run(instrs);
    let faulty_idx = find_faulty_instr(instrs, has_visited);

    instrs[faulty_idx] = match instrs[faulty_idx] {
        Instr::Nop(n) => Instr::Jmp(n),
        Instr::Jmp(n) => Instr::Nop(n),
        Instr::Acc(_) => unreachable!(),
    };

    let (success, accumulator, _) = run(instrs);
    let end = Instant::now();
    let dur: Duration = end - start;
    println!("dur: {}", dur.as_micros());
    assert!(success);
    accumulator
}

fn main() {
    let input = prelude::read_input("input.txt");
    let mut instrs: Vec<Instr> = parse_instrs(&input).unwrap().1;
    let (part1_success, accumulator, _) = run(&instrs);
    assert!(!part1_success);
    println!();
    println!("Part 1");
    println!("Answer: {}", accumulator);
    println!("=======================");
    println!("Part 2");
    println!("Answer: {}", part2(&mut instrs));
}

#[cfg(test)]
mod tests {
    use super::Instr;
    fn get_instrs() -> Vec<Instr> {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        super::parse_instrs(&input).unwrap().1
    }

    #[test]
    fn test_part1() {
        let instrs = get_instrs();
        let (success, accum, _) = super::run(&instrs);
        assert_eq!(false, success);
        assert_eq!(5, accum);
    }

    #[test]
    fn test_part2() {
        let mut instrs = get_instrs();
        let accum = super::part2(&mut instrs);
        assert_eq!(8, accum);
    }
}
