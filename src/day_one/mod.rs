use itertools::Itertools;


pub fn solve(input: String) {
    let lines = input.lines();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        let mut splitted = line.split_whitespace();
        left.push(splitted.next()
                  .expect("Should have two numbers per line")
                  .parse::<u32>()
                  .expect("Should be a number"));
        right.push(splitted.next()
                  .expect("Should have two numbers per line")
                  .parse::<u32>()
                   .expect("Should be a number"));
    }
    println!("Simple solution: {}", sum_distances(left.as_mut_slice(), right.as_mut_slice()));
}




fn sum_distances(a: &mut [u32], b: &mut [u32]) -> u32 {
    assert!(a.len() == b.len());
    a.sort();
    b.sort();

    let mut sum = 0;
    let zipped = a.into_iter().zip_eq(b.into_iter());
    for (left, right) in zipped {
        sum += left.abs_diff(*right)
    }
    sum
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sum_distance() {
        let mut left = [3, 4, 2, 1, 3, 3];
        let mut right = [4, 3, 5, 3, 9, 3];
        assert_eq!(sum_distances(left.as_mut_slice(), right.as_mut_slice()), 11);
    }
}
