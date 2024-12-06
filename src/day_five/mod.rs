use std::{collections::{HashMap, HashSet}, str::FromStr};


pub fn solve(input: String) {
    println!("Simple solution: {}", solve_simple(&input));
}


pub fn solve_simple(input: &str) -> usize {
    let mut orderings: Vec<Ordering> =  Vec::new();
    let mut sum = 0;
    let mut parse_ordering = true;
    for line in input.lines() {
        if line.is_empty() {
            parse_ordering = false;
        } else if parse_ordering {
            orderings.push(line.parse().expect("Should be able to parse ordering"));
        } else {
            let numbers: Vec<usize> =  line.split(",").map(|c| c.parse().expect("Should have numbers")).collect();
            if check_ordering(&numbers, &orderings) {
                sum += numbers.get((numbers.len() - 1) / 2).unwrap_or(&0);
            }
        }
    }
    sum
}


struct Ordering {
    before: usize,
    after: usize
}

#[derive(Debug)]
struct ParseOrderingError;

impl FromStr for Ordering {
    type Err = ParseOrderingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("|")
            .ok_or(ParseOrderingError)?;

        let a_fromstr = a.parse::<usize>().map_err(|_| ParseOrderingError)?;
        let b_fromstr = b.parse::<usize>().map_err(|_| ParseOrderingError)?;
        Ok(Ordering {
            before: a_fromstr,
            after: b_fromstr,
        })
    }
}

fn check_ordering(pages: &[usize], ordering: &[Ordering]) -> bool {
    let mut exclusions: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut seen: HashSet<usize> = HashSet::new();
    for order in ordering {
        let mut default = HashSet::new();
        default.insert(order.after);
        exclusions.entry(order.before).and_modify(|s| {s.insert(order.after);}).or_insert(default);
    }
        pages.iter()
        .map(|page| {
            let result = exclusions.get(page)
                .map(|set| set.intersection(&seen).collect::<Vec<&usize>>().is_empty())
                .unwrap_or(true);
            seen.insert(*page);
            result
        })
        .all(|x| x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let orderings = [
            Ordering { before: 47, after: 53 },
            Ordering { before: 97, after: 13 },
            Ordering { before: 97, after: 61 },
            Ordering { before: 97, after: 47 },
            Ordering { before: 75, after: 29 },
            Ordering { before: 61, after: 13 },
            Ordering { before: 75, after: 53 },
            Ordering { before: 29, after: 13 },
            Ordering { before: 97, after: 29 },
            Ordering { before: 53, after: 29 },
            Ordering { before: 61, after: 53 },
            Ordering { before: 97, after: 53 },
            Ordering { before: 61, after: 29 },
            Ordering { before: 47, after: 13 },
            Ordering { before: 75, after: 47 },
            Ordering { before: 97, after: 75 },
            Ordering { before: 47, after: 61 },
            Ordering { before: 75, after: 61 },
            Ordering { before: 47, after: 29 },
            Ordering { before: 75, after: 13 },
            Ordering { before: 53, after: 13 },
        ];
        assert!(check_ordering(&[75,47,61,53,29], &orderings));
        assert!(!check_ordering(&[75,97,47,61,53], &orderings));
    }
}
