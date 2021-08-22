use nom::{
    branch,
    bytes::complete,
    bytes::streaming,
    character::complete::{digit1, space0, space1},
    combinator, IResult,
};
use smallvec::{smallvec, SmallVec};
use std::collections::{HashMap, HashSet, VecDeque};

type InternId = u32;

struct Interner {
    lookup_string: Vec<String>,
    lookup_id: HashMap<String, InternId>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            lookup_id: HashMap::new(),
            lookup_string: Vec::new(),
        }
    }

    pub fn intern<S: Into<String>>(&mut self, s: S) -> u32 {
        let text: String = s.into();
        if let Some(&id) = self.lookup_id.get(&text) {
            id
        } else {
            let id = self.lookup_string.len() as u32;
            self.lookup_string.push(text.clone());
            self.lookup_id.insert(text, id);
            id
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, id: u32) -> Option<&str> {
        self.lookup_string.get(id as usize).map(|s| s as &str)
    }
}

type ColorContains = SmallVec<[(u8, InternId); 4]>;
type ColorContainedIn = SmallVec<[InternId; 4]>;

fn parse_rule<'a>(
    interner: &mut Interner,
    input: &'a str,
) -> IResult<&'a str, (InternId, ColorContains)> {
    const COLOR_AND_ARGS_SEP: &'static str = " bags contain ";
    let (input, color) = streaming::take_until(COLOR_AND_ARGS_SEP)(input)?;
    let input = &input[COLOR_AND_ARGS_SEP.len()..];
    let color = interner.intern(color);

    let (input, rules) = parse_rule_list(interner, input)?;

    Ok((input, (color, rules)))
}

fn parse_rule_list<'a>(interner: &mut Interner, input: &'a str) -> IResult<&'a str, ColorContains> {
    let (input, args) = complete::take_till(|c| c == '.')(input)?;
    if args == "no other bags" {
        return Ok((input, SmallVec::new()));
    }
    let mut colors: ColorContains = SmallVec::new();
    let mut args: &str = space0(args)?.0;
    while args.len() > 0 {
        let (next_args, arg) = complete::take_till(|c| c == ',')(args)?;
        args = if next_args.len() > 1 {
            &next_args[2..]
        } else {
            ""
        };
        let (arg, n) = combinator::map_res(digit1, |n: &str| n.parse::<u8>())(arg)?;
        let (color, _) = space1(arg)?;
        let color: &str = (branch::alt((
            combinator::peek(streaming::take_until(" bag")),
            streaming::take_until(" bags"),
        )))(color)?
        .1;
        colors.push((n, interner.intern(color)));
    }
    Ok((input, colors))
}

fn part1(contained_in: &HashMap<InternId, ColorContainedIn>, shiny_gold: InternId) -> usize {
    let mut contain_shiny_gold = HashSet::new();
    let mut queue = VecDeque::new();
    for &bt in contained_in.get(&shiny_gold).into_iter().flatten() {
        queue.push_back(bt);
    }

    while let Some(bag_type) = queue.pop_front() {
        contain_shiny_gold.insert(bag_type);
        for &bt in contained_in.get(&bag_type).into_iter().flatten() {
            queue.push_back(bt);
        }
    }
    contain_shiny_gold.len()
}

fn part2(contains: &HashMap<InternId, ColorContains>, color: InternId) -> usize {
    contains
        .get(&color)
        .into_iter()
        .flatten()
        .map(|(n, sub_color)| (*n as usize) * (1 + part2(contains, *sub_color)))
        .sum()
}

fn contained_in(
    contains: &HashMap<InternId, ColorContains>,
) -> HashMap<InternId, ColorContainedIn> {
    let mut contained_in: HashMap<InternId, ColorContainedIn> = HashMap::new();
    for (color1, color2) in contains
        .iter()
        .flat_map(|(&color, contained)| contained.iter().map(move |(_, color2)| (*color2, color)))
    {
        if let Some(xs) = contained_in.get_mut(&color1) {
            if !xs.contains(&color2) {
                xs.push(color2);
            }
        } else {
            contained_in.insert(color1, smallvec![color2]);
        }
    }
    contained_in
}

fn main() {
    let mut interner = Interner::new();
    let contains: HashMap<InternId, ColorContains> = prelude::read_input_lines("input.txt")
        .iter()
        .map(|line| parse_rule(&mut interner, line).unwrap().1)
        .collect();

    let shiny_gold = interner.intern("shiny gold");
    println!();
    println!("Part 1");
    println!("Answer: {}", part1(&contained_in(&contains), shiny_gold));
    println!("=======================");
    println!("Part 2");
    println!("Answer: {}", part2(&contains, shiny_gold));
}

#[cfg(test)]
mod tests {
    use super::{ColorContains, InternId, Interner};
    use std::collections::HashMap;

    #[test]
    fn part1_test() {
        let mut interner = Interner::new();
        let contains: HashMap<InternId, ColorContains> =
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
                .split("\n")
                .map(|line| super::parse_rule(&mut interner, line).unwrap().1)
                .collect();

        let shiny_gold = interner.intern("shiny gold");
        assert_eq!(4, super::part1(&super::contained_in(&contains), shiny_gold));
    }

    #[test]
    fn part2_test() {
        let mut interner = Interner::new();
        let contains: HashMap<InternId, ColorContains> = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            .split("\n")
            .map(|line| super::parse_rule(&mut interner, line).unwrap().1)
            .collect();

        let shiny_gold = interner.intern("shiny gold");
        assert_eq!(126, super::part2(&contains, shiny_gold));
    }
}
