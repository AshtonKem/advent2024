use itertools::Itertools;


pub fn solve(input: String) {
    let data = input.lines().map(|line| parse_report(line)).collect();
    println!("Number of safe reports: {}", count_safe_reports(data));
}

fn count_safe_reports(input: Vec<Vec<u32>>) -> u32 {
    input.iter()
        .filter(|report| safe_report(report.as_slice()))
        .count()
        .try_into().unwrap()
}

fn parse_report(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|token| token.parse::<u32>().expect("Should be a number"))
        .collect()
}

fn safe_report(report: &[u32]) -> bool {
    (increasing(report) || decreasing(report)) && ranges(report)
}

fn increasing(report: &[u32]) -> bool {
    report.into_iter()
         .tuple_windows()
         .all(|(a, b)| a < b)
}

fn decreasing(report: &[u32]) -> bool {
    report.into_iter()
         .tuple_windows()
         .all(|(a, b)| a > b)
}

fn ranges(report: &[u32]) -> bool {
    report.into_iter()
         .tuple_windows()
         .all(|(a, b)| 1 <= a.abs_diff(*b) && 3 >= a.abs_diff(*b))
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_increasing() {
        assert!(increasing(&[1,2,3]));
        assert!(increasing(&[1,5,35]));
        assert!(!increasing(&[1,5,5,20]));
        assert!(!increasing(&[1,5,4]));
    }

    #[test]
    fn test_decreasing() {
        assert!(decreasing(&[3,2,1]));
        assert!(decreasing(&[6,4,1]));
        assert!(!decreasing(&[6,4,4]));
        assert!(!decreasing(&[6,4,5]));
    }

    #[test]
    fn test_ranges() {
        assert!(ranges(&[1,2,3]));
        assert!(ranges(&[1,3,5]));
        assert!(ranges(&[1,4,7]));
        assert!(ranges(&[7, 4, 1]));
        assert!(!ranges(&[8, 4, 1]));
    }

}
