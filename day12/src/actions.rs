
#[derive(Debug, Clone, Copy)]
pub enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    TurnLeft(i32),
    TurnRight(i32),
    Forward(i32),
}

fn parse_u32(s: &str) -> Result<(&str, u32), &str> {
    let mut s = s;
    let mut chars: String = String::new();

    while let Some(ch) = s.chars().next() {
        if ch.is_numeric() {
            chars.push(ch);
            s = &s[1..];
        } else {
            break;
        }
    }

    match chars.parse::<u32>() {
        Err(_) => Err(s),
        Ok(n) => Ok((s, n))
    }
}

fn parse_ch_u32_pair(s: &str) -> Result<(&str, char, u32), &str> {
    let ch: char = match s.chars().next() {
        Some(ch) if ch.is_alphabetic() => ch,
        Some(_) => return Err(s),
        None => return Err(""),
    };
    let s = &s[1..];

    let (s, n) = match parse_u32(s) {
        Ok(x) => x,
        Err(s) => return Err(s)
    };

    Ok((s, ch, n))
}

fn parse_action(s: &str) -> Result<(&str, Action), &str> {
    let (s, a, n) = match parse_ch_u32_pair(s) {
        Ok(x) => x,
        Err(s) => return Err(s),
    };
    let n = n as i32;
    match a {
        'N' => Ok((s, Action::North(n))),
        'S' => Ok((s, Action::South(n))),
        'E' => Ok((s, Action::East(n))),
        'W' => Ok((s, Action::West(n))),
        'L' => Ok((s, Action::TurnLeft(n))),
        'R' => Ok((s, Action::TurnRight(n))),
        'F' => Ok((s, Action::Forward(n))),
        _ => Err(s)
    }
}

fn trim_next(s: &str) -> &str {
    let mut s = s;
    while let Some(ch) = s.chars().next() {
        if ch == ' ' || ch == '\n' || ch == '\r' {
            s = &s[1..];
        } else {
            break;
        }
    }

    s
}

pub fn parse_actions(s: &str) -> Result<Vec<Action>, &str> {
    let mut actions = Vec::new();
    let mut s = s;

    while s != "" {
        let (s_, a) = match parse_action(s) {
            Ok(x) => x,
            Err(rest) => return Err(rest)
        };

        actions.push(a);
        s = trim_next(s_);
    }
    Ok(actions)
}
