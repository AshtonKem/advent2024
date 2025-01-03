use std::ops::Range;

use itertools::Itertools;

pub fn solve(input: String) {
    let data: Vec<Vec<u32>> = input.lines().map(parse_report).collect();
    println!(
        "Number of safe reports: {}",
        count_safe_reports(data.as_slice())
    );
    println!(
        "Number of safe reports (damped): {}",
        count_safe_reports_damped(data.as_slice())
    );
}

fn count_safe_reports(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .filter(|report| safe_report(report.as_slice()))
        .count()
        .try_into()
        .unwrap()
}

fn count_safe_reports_damped(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .filter(|report| safe_report_damped(report.as_slice()))
        .count()
        .try_into()
        .unwrap()
}

fn parse_report(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|token| token.parse::<u32>().expect("Should be a number"))
        .collect()
}

fn safe_report(report: &[u32]) -> bool {
    (increasing(report) || decreasing(report)) && ranges(report)
}

fn safe_report_damped(report: &[u32]) -> bool {
    if safe_report(report) {
        true
    } else {
        for index in (Range {
            start: 0,
            end: report.len(),
        }) {
            let mut report_copy: Vec<u32> = report.to_vec();
            report_copy.remove(index);
            if safe_report(report_copy.as_slice()) {
                return true;
            }
        }
        false
    }
}

fn increasing(report: &[u32]) -> bool {
    report.iter().tuple_windows().all(|(a, b)| a < b)
}

fn decreasing(report: &[u32]) -> bool {
    report.iter().tuple_windows().all(|(a, b)| a > b)
}

fn ranges(report: &[u32]) -> bool {
    report
        .iter()
        .tuple_windows()
        .all(|(a, b)| 1 <= a.abs_diff(*b) && 3 >= a.abs_diff(*b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing() {
        assert!(increasing(&[1, 2, 3]));
        assert!(increasing(&[1, 5, 35]));
        assert!(!increasing(&[1, 5, 5, 20]));
        assert!(!increasing(&[1, 5, 4]));
    }

    #[test]
    fn test_decreasing() {
        assert!(decreasing(&[3, 2, 1]));
        assert!(decreasing(&[6, 4, 1]));
        assert!(!decreasing(&[6, 4, 4]));
        assert!(!decreasing(&[6, 4, 5]));
    }

    #[test]
    fn test_ranges() {
        assert!(ranges(&[1, 2, 3]));
        assert!(ranges(&[1, 3, 5]));
        assert!(ranges(&[1, 4, 7]));
        assert!(ranges(&[7, 4, 1]));
        assert!(!ranges(&[8, 4, 1]));
    }
}
