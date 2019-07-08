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

pub trait HasPos {
    fn pos(&self) -> Pos;
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

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
