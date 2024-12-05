use array2d::Array2D;

pub fn solve(input: String) {
    let matrix = build_array(&input);
    println!("Matches: {}", count_matches(&matrix));
    println!("X-Mas: {}", count_mas(&matrix));
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

fn count_matches(matrix: &Array2D<char>) -> usize {
    let xs = find_character(matrix, 'X');
    xs.iter().map(|point| match_count(matrix, point)).sum()
}

fn count_mas(matrix: &Array2D<char>) -> usize {
    let positions = find_character(matrix, 'A');
    positions
        .iter()
        .filter(|point| match_x_mas(matrix, point))
        .count()
}

fn build_array(input: &str) -> Array2D<char> {
    let lines: Vec<&str> = input.lines().collect();
    let chars: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    Array2D::from_rows(chars.as_slice()).unwrap()
}

fn find_character(matrix: &Array2D<char>, search_char: char) -> Vec<Point> {
    let mut result = Vec::new();
    for (y, row_iter) in matrix.rows_iter().enumerate() {
        for (x, elem) in row_iter.enumerate() {
            if *elem == search_char {
                result.push(Point { x, y });
            }
        }
    }
    result
}

fn match_x_mas(matrix: &Array2D<char>, point: &Point) -> bool {
    if point.x >= 1 && point.y >= 1 {
        let diag_one: String = [
            matrix.get(point.y - 1, point.x - 1),
            matrix.get(point.y, point.x),
            matrix.get(point.y + 1, point.x + 1),
        ]
        .iter()
        .filter_map(|p| *p)
        .collect();

        let diag_two: String = [
            matrix.get(point.y + 1, point.x - 1),
            matrix.get(point.y, point.x),
            matrix.get(point.y - 1, point.x + 1),
        ]
        .iter()
        .filter_map(|p| *p)
        .collect();

        (diag_one == "MAS" || diag_one == "SAM") && (diag_two == "SAM" || diag_two == "MAS")
    } else {
        false
    }
}

fn match_left(matrix: &Array2D<char>, point: &Point) -> bool {
    point.x >= 3
        && matrix.get(point.y, point.x) == Some(&'X')
        && matrix.get(point.y, point.x - 1) == Some(&'M')
        && matrix.get(point.y, point.x - 2) == Some(&'A')
        && matrix.get(point.y, point.x - 3) == Some(&'S')
}

fn match_right(matrix: &Array2D<char>, point: &Point) -> bool {
    matrix.get(point.y, point.x) == Some(&'X')
        && matrix.get(point.y, point.x + 1) == Some(&'M')
        && matrix.get(point.y, point.x + 2) == Some(&'A')
        && matrix.get(point.y, point.x + 3) == Some(&'S')
}

fn match_up(matrix: &Array2D<char>, point: &Point) -> bool {
    point.y >= 3
        && matrix.get(point.y, point.x) == Some(&'X')
        && matrix.get(point.y - 1, point.x) == Some(&'M')
        && matrix.get(point.y - 2, point.x) == Some(&'A')
        && matrix.get(point.y - 3, point.x) == Some(&'S')
}

fn match_down(matrix: &Array2D<char>, point: &Point) -> bool {
    matrix.get(point.y, point.x) == Some(&'X')
        && matrix.get(point.y + 1, point.x) == Some(&'M')
        && matrix.get(point.y + 2, point.x) == Some(&'A')
        && matrix.get(point.y + 3, point.x) == Some(&'S')
}

fn match_down_right(matrix: &Array2D<char>, point: &Point) -> bool {
    matrix.get(point.y, point.x) == Some(&'X')
        && matrix.get(point.y + 1, point.x + 1) == Some(&'M')
        && matrix.get(point.y + 2, point.x + 2) == Some(&'A')
        && matrix.get(point.y + 3, point.x + 3) == Some(&'S')
}

fn match_down_left(matrix: &Array2D<char>, point: &Point) -> bool {
    point.x >= 3
        && matrix.get(point.y, point.x) == Some(&'X')
        && matrix.get(point.y + 1, point.x - 1) == Some(&'M')
        && matrix.get(point.y + 2, point.x - 2) == Some(&'A')
        && matrix.get(point.y + 3, point.x - 3) == Some(&'S')
}

fn match_up_left(matrix: &Array2D<char>, point: &Point) -> bool {
    point.x >= 3
        && point.y >= 3
        && matrix.get(point.y, point.x) == Some(&'X')
        && matrix.get(point.y - 1, point.x - 1) == Some(&'M')
        && matrix.get(point.y - 2, point.x - 2) == Some(&'A')
        && matrix.get(point.y - 3, point.x - 3) == Some(&'S')
}

fn match_up_right(matrix: &Array2D<char>, point: &Point) -> bool {
    point.y >= 3
        && matrix.get(point.y, point.x) == Some(&'X')
        && matrix.get(point.y - 1, point.x + 1) == Some(&'M')
        && matrix.get(point.y - 2, point.x + 2) == Some(&'A')
        && matrix.get(point.y - 3, point.x + 3) == Some(&'S')
}

fn match_count(matrix: &Array2D<char>, point: &Point) -> usize {
    let mut matches = 0;
    if match_left(matrix, point) {
        matches += 1;
    }
    if match_right(matrix, point) {
        matches += 1;
    }
    if match_up(matrix, point) {
        matches += 1;
    }
    if match_down(matrix, point) {
        matches += 1;
    }
    if match_up_left(matrix, point) {
        matches += 1;
    }
    if match_up_right(matrix, point) {
        matches += 1;
    }
    if match_down_left(matrix, point) {
        matches += 1;
    }
    if match_down_right(matrix, point) {
        matches += 1;
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_character() {
        assert_eq!(
            vec![Point { x: 0, y: 0 }],
            find_character(&build_array("X"), 'X')
        );
        assert_eq!(
            vec![Point { x: 3, y: 0 }, Point { x: 0, y: 1 }],
            find_character(&build_array("abcX\nXabc"), 'X')
        );
    }

    #[test]
    fn test_matcht() {
        let matrix = build_array("..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....");
        assert!(match_left(&matrix, &Point { x: 4, y: 1 }));
        assert!(!match_left(&matrix, &Point { x: 0, y: 0 }));
        assert!(!match_left(&matrix, &Point { x: 1, y: 4 }));

        assert!(match_right(&matrix, &Point { x: 0, y: 3 }));
        assert!(!match_right(&matrix, &Point { x: 1, y: 4 }));

        assert!(match_up(&matrix, &Point { x: 1, y: 4 }));
        assert!(!match_up(&matrix, &Point { x: 0, y: 0 }));
        assert!(!match_up(&matrix, &Point { x: 2, y: 0 }));
    }

    #[test]
    fn test_count_matches() {
        let matrix = build_array("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX");
        assert_eq!(18, count_matches(&matrix));
    }

    #[test]
    fn test_mas() {
        let matrix = build_array(
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        );
        assert!(match_x_mas(&matrix, &Point { x: 2, y: 1 }));
        assert_eq!(9, count_mas(&matrix));
    }
}
