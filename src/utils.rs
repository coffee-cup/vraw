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
