#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pos {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Range {
    pub start: Pos,
    pub end: Pos,
}

pub fn create_pos(line: u32, column: u32) -> Pos {
    Pos {
        line: line,
        column: column,
    }
}

pub fn create_range(start: Pos, end: Pos) -> Range {
    Range {
        start: start,
        end: end,
    }
}
