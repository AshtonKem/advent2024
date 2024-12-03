use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
struct Mul {
    a: u32,
    b: u32,
}

#[derive(Debug)]
struct ParseMulError;

impl FromStr for Mul {
    type Err = ParseMulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .strip_prefix("mul(")
            .and_then(|str| str.strip_suffix(")"))
            .and_then(|str| str.split_once(","))
            .ok_or(ParseMulError)?;

        let a_fromstr = a.parse::<u32>().map_err(|_| ParseMulError)?;
        let b_fromstr = b.parse::<u32>().map_err(|_| ParseMulError)?;
        Ok(Mul {
            a: a_fromstr,
            b: b_fromstr,
        })
    }
}

fn multiply(mul: &Mul) -> u32 {
    mul.a * mul.b
}

fn find_matches(input: &String) -> Vec<Mul> {
    let re = Regex::new(r"mul\((\d*)\,(\d*)\)").unwrap();
    re.find_iter(input)
        .map(|f| f.as_str().parse::<Mul>().expect("Should parse into mul"))
        .collect()
}

pub fn solve(input: String) {
    println!("Simple sum: {}", add_matches(&input));
}

fn add_matches(input: &String) -> u32 {
    find_matches(input)
        .into_iter()
        .map(|mul| multiply(&mul))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_matches() {
        assert_eq!(
            161,
            add_matches(
                &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_string()
            )
        );
    }
}
