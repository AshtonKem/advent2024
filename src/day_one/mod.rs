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




fn sum_distances(a: &[u32], b: &[u32]) -> u32 {
    assert!(a.len() == b.len());
    let mut left = a.to_vec();
    let mut right = b.to_vec();

    left.sort();
    right.sort();


    let mut sum = 0;
    let zipped = left.into_iter().zip_eq(right.into_iter());
    for (x, y) in zipped {
        sum += x.abs_diff(y)
    }
    sum
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sum_distance() {
        let left = [3, 4, 2, 1, 3, 3];
        let right = [4, 3, 5, 3, 9, 3];
        assert_eq!(sum_distances(left.as_slice(), right.as_slice()), 11);
    }
}
