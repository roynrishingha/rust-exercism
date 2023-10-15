// Instructions
// Add the mine counts to a completed Minesweeper board.

// Minesweeper is a popular game where the user has to find the mines using numeric hints that indicate how many mines are directly adjacent (horizontally, vertically, diagonally) to a square.

// In this exercise you have to create some code that counts the number of mines adjacent to a given empty square and replaces that square with the count.

// The board is a rectangle composed of blank space (' ') characters. A mine is represented by an asterisk ('*') character.

// If a given space has no adjacent mines at all, leave that square blank.

// Examples
// For example you may receive a 5 x 4 board like this (empty spaces are represented here with the '·' character for display on screen):

// ·*·*·
// ··*··
// ··*··
// ·····
// And your code will transform it into this:

// 1*3*1
// 13*31
// ·2*2·
// ·111·
// Performance Hint
// All the inputs and outputs are in ASCII. Rust Strings and &str are utf8, so while one might expect "Hello".chars() to be simple, it actually has to check each char to see if it's 1, 2, 3 or 4 u8s long. If we know a &str is ASCII then we can call .as_bytes() and refer to the underlying data via a &[u8] slice. Iterating over a u8 slice of ASCII is much quicker as there are no codepoints involved - every ASCII char is one u8 long.

// Can you complete the challenge without cloning the input?

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let offset: Vec<_> = [-1, -1, -1, 0, 0, 1, 1, 1]
        .iter()
        .zip([-1, 0, 1, -1, 1, -1, 0, 1])
        .collect();
    let width = if minefield.len() > 0 {
        minefield[0].len() as i32
    } else {
        0
    };
    let height = minefield.len() as i32;

    minefield
        .iter()
        .enumerate()
        .map(|(i, r)| {
            r.char_indices()
                .map(|(j, c)| match c {
                    '*' => c,
                    _ => match offset
                        .iter()
                        .map(|&(n, m)| (i as i32 + n, j as i32 + m))
                        .filter(|&(n, m)| n >= 0 && n < height && m >= 0 && m < width)
                        .filter(|&(n, m)| minefield[n as usize].as_bytes()[m as usize] == b'*')
                        .count()
                    {
                        0 => ' ',
                        mines => (mines as u8 + '0' as u8) as char,
                    },
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn remove_annotations(board: &[&str]) -> Vec<String> {
        board.iter().map(|r| remove_annotations_in_row(r)).collect()
    }

    fn remove_annotations_in_row(row: &str) -> String {
        row.chars()
            .map(|ch| match ch {
                '*' => '*',
                _ => ' ',
            })
            .collect()
    }

    fn run_test(test_case: &[&str]) {
        let cleaned = remove_annotations(test_case);
        let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
        let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, annotate(&cleaned_strs));
    }

    #[test]
    fn no_rows() {
        #[rustfmt::skip]
    run_test(&[
    ]);
    }

    #[test]
    fn no_columns() {
        #[rustfmt::skip]
    run_test(&[
        "",
    ]);
    }

    #[test]
    fn no_mines() {
        #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
    }

    #[test]
    fn board_with_only_mines() {
        #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
    }

    #[test]
    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
    }

    #[test]
    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
    }

    #[test]
    fn horizontal_line() {
        #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
    }

    #[test]
    fn horizontal_line_mines_at_edges() {
        #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
    }

    #[test]
    fn vertical_line() {
        #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
    }

    #[test]
    fn vertical_line_mines_at_edges() {
        #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
    }

    #[test]
    fn cross() {
        #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
    }

    #[test]
    fn large_board() {
        #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
    }
}
