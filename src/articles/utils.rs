use std::error::Error;

pub fn position_to_line_and_col_number(
    article_mdwn_raw_string: &String,
    position: usize,
) -> Result<(usize, usize), Box<dyn Error>> {
    if position >= article_mdwn_raw_string.len() {
        return Err("Position is out of bounds".into());
    }

    let mut line = 0;
    let mut col = 0;
    let mut current_pos = 0;

    for (i, ch) in article_mdwn_raw_string.chars().enumerate() {
        if current_pos == position {
            return Ok((line, col));
        }

        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }

        current_pos += ch.len_utf8();
    }

    Err("Failed to backtrack position".into())
}
