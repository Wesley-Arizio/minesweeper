pub fn increment_or_switch(byte: u8) -> u8 {
    match byte {
        b' ' => b'1',
        b'0'..=b'8' => byte + 1,
        b'*' => b'*',
        _ => unreachable!(),
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    if rows == 0 {
        return vec![];
    };

    let cols = minefield[0].len();
    let mut result = vec![vec![b' '; cols]; rows];

    for i in 0..rows {
        for (j, &c) in minefield[i].as_bytes().iter().enumerate() {
            if c == b'*' {
                result[i][j] = b'*';

                // validate up values
                if i as i32 - 1 >= 0 {
                    // up
                    result[i - 1][j] = increment_or_switch(result[i - 1][j]);

                    // validate up-left diagonal
                    if j as i32 - 1 >= 0 {
                        // Up left diagonal
                        result[i - 1][j - 1] = increment_or_switch(result[i - 1][j - 1]);
                    }
                    // validate up-right diagonal
                    if j + 1 < minefield[i - 1].len() {
                        // Up right diagonal
                        result[i - 1][j + 1] = increment_or_switch(result[i - 1][j + 1]);
                    }
                }

                // validate in line values
                if j as i32 - 1 >= 0 {
                    // Left
                    result[i][j - 1] = increment_or_switch(result[i][j - 1]);
                }

                if j + 1 < minefield[i].len() {
                    // Right
                    result[i][j + 1] = increment_or_switch(result[i][j + 1]);
                }

                // validate down values
                if i + 1 < minefield.len() {
                    // Down
                    result[i + 1][j] = increment_or_switch(result[i + 1][j]);

                    if j as i32 - 1 >= 0 {
                        // Left down diagonal
                        result[i + 1][j - 1] = increment_or_switch(result[i + 1][j - 1]);
                    }

                    if j + 1 < minefield[i + 1].len() {
                        // Right down diagonal
                        result[i + 1][j + 1] = increment_or_switch(result[i + 1][j + 1]);
                    }
                }
            }
        }
    }

    result
        .into_iter()
        .map(|c| String::from_utf8(c).unwrap())
        .collect::<Vec<_>>()
}
