use gitko_render::{Line, Part};


pub fn color_diff_line(line: &str) -> Line {
    if line.starts_with('+') {
        Line::new(vec![
            Part::painted(
                line,
                (0, 255, 0),
                (0, 0, 0)
            )
        ])
    } else if line.starts_with('-') {
        Line::new(vec![
            Part::painted(
                line,
                (255, 0, 0),
                (0, 0, 0)
            )
        ])
    } else if line.starts_with("@@") {
        Line::new(vec![
            Part::painted(
                line,
                (0, 255, 255),
                (0, 0, 0)
            )
        ])
    } else {
        Line::plain(line)
    }
}
