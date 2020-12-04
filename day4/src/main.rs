#![feature(str_split_once)]
use prelude::*;

type Passport = Vec<(String, String)>;

fn parse(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|pass: &str| {
            let xs: Passport = pass
                .split(" ")
                .flat_map(|x| x.split("\n"))
                .filter(|&x| x != "")
                .map(|kvp| {
                    println!("{}", kvp);
                    let (k, v) = kvp.split_once(':').unwrap();
                    (k.to_string(), v.to_string())
                })
                .collect();
            xs
        })
        .collect()
}

fn has_required_fields(passports: &Passport) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|&k| passports.iter().any(|(k2, _)| k2 == k))
}

fn is_valid(passports: &Passport) -> bool {
    if !has_required_fields(passports) {
        return false;
    };
    for (k, v) in passports {
        match k as &str {
            "byr" => match v.parse::<u32>() {
                Ok(v) if v < 1920 || v > 2002 => return false,
                Err(_) => return false,
                _ => {}
            },
            "iyr" => match v.parse::<u32>() {
                Ok(v) if v < 2010 || v > 2020 => return false,
                Err(_) => return false,
                _ => {}
            },
            "eyr" => match v.parse::<u32>() {
                Ok(v) if v < 2020 || v > 2030 => return false,
                Err(_) => return false,
                _ => {}
            },
            "hgt" => {
                if v.len() < 2 {
                    return false;
                }
                let unit_type = &v[(v.len() - 2)..];
                let v = match (&v[0..v.len() - 2]).parse::<u32>() {
                    Ok(v) => v,
                    _ => return false,
                };
                match unit_type {
                    "cm" => {
                        if v < 150 || v > 193 {
                            return false;
                        }
                    }
                    "in" => {
                        if v < 59 || v > 76 {
                            return false;
                        }
                    }
                    _ => return false,
                };
            }
            "hcl" => {
                if v.len() != 7
                    || v.chars().nth(0).unwrap() != '#'
                    || !v.chars().skip(1).all(|c| c.is_digit(16))
                {
                    return false;
                }
            }
            "ecl" => match v as &str {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => return false,
            },
            "pid" => {
                if v.len() != 9 || !v.chars().all(|c| c.is_digit(10)) {
                    return false;
                }
            }
            _ => {}
        };
    }
    true
}

fn part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|&x| has_required_fields(x)).count()
}

fn part2(passports: &[Passport]) -> usize {
    passports.iter().filter(|&x| is_valid(x)).count()
}

fn main() {
    let input = read_input("input.txt");
    let passports = parse(&input);

    for p in passports.iter() {
        println!("{:?}", p);
    }

    println!("Part 1");
    println!("Answer: {}", part1(&passports));
    println!("=======================");
    println!("Part 2");
    println!("Answer: {}", part2(&passports));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let passports = super::parse(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
        );

        assert_eq!(2, super::part1(&passports));
    }

    #[test]
    fn part2() {
        let invalid_passports = super::parse(
            "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
        );
        let valid_passports = super::parse(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        );
        assert_eq!(0, super::part2(&invalid_passports));
        assert_eq!(valid_passports.len(), super::part2(&valid_passports));
    }
}
