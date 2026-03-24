use litmus_model::term_output::{TermColor, TermLine, TermOutput, TermSpan};

/// Parse raw ANSI-escaped bytes into a structured `TermOutput`.
///
/// Uses a cell grid of `cols × rows` to handle cursor movement and overwrites,
/// then collapses adjacent cells with identical attributes into `TermSpan`s.
pub fn parse_ansi(input: &[u8], cols: u16, rows: u16, id: &str, name: &str) -> TermOutput {
    let mut grid = Grid::new(cols as usize, rows as usize);
    let mut parser = vte::Parser::new();

    for &byte in input {
        parser.advance(&mut grid, byte);
    }

    grid.to_term_output(id, name, cols, rows)
}

// -- Cell grid --

#[derive(Debug, Clone, Copy, PartialEq)]
struct CellAttrs {
    fg: TermColor,
    bg: TermColor,
    bold: bool,
    italic: bool,
    dim: bool,
    underline: bool,
}

impl Default for CellAttrs {
    fn default() -> Self {
        Self {
            fg: TermColor::Default,
            bg: TermColor::Default,
            bold: false,
            italic: false,
            dim: false,
            underline: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Cell {
    ch: char,
    attrs: CellAttrs,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            attrs: CellAttrs::default(),
        }
    }
}

struct Grid {
    cols: usize,
    rows: usize,
    cells: Vec<Vec<Cell>>,
    cursor_row: usize,
    cursor_col: usize,
    /// The highest row that has been written to (for trimming output).
    max_row: usize,
    attrs: CellAttrs,
}

impl Grid {
    fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            cells: vec![vec![Cell::default(); cols]; rows],
            cursor_row: 0,
            cursor_col: 0,
            max_row: 0,
            attrs: CellAttrs::default(),
        }
    }

    fn put_char(&mut self, ch: char) {
        if self.cursor_row < self.rows && self.cursor_col < self.cols {
            self.cells[self.cursor_row][self.cursor_col] = Cell {
                ch,
                attrs: self.attrs,
            };
            if self.cursor_row > self.max_row {
                self.max_row = self.cursor_row;
            }
            self.cursor_col += 1;
        }
    }

    fn newline(&mut self) {
        self.cursor_col = 0;
        if self.cursor_row + 1 < self.rows {
            self.cursor_row += 1;
        }
        // Scroll handling could be added here for rows >= self.rows
    }

    fn carriage_return(&mut self) {
        self.cursor_col = 0;
    }

    fn apply_sgr(&mut self, params: &[&[u16]]) {
        let mut i = 0;
        while i < params.len() {
            let p = if params[i].is_empty() { 0 } else { params[i][0] };
            match p {
                0 => self.attrs = CellAttrs::default(),
                1 => self.attrs.bold = true,
                2 => self.attrs.dim = true,
                3 => self.attrs.italic = true,
                4 => self.attrs.underline = true,
                22 => {
                    self.attrs.bold = false;
                    self.attrs.dim = false;
                }
                23 => self.attrs.italic = false,
                24 => self.attrs.underline = false,
                // Standard foreground colors 30-37
                30..=37 => self.attrs.fg = TermColor::Ansi((p - 30) as u8),
                // Extended foreground: 38;5;N or 38;2;R;G;B
                38 => {
                    i += 1;
                    if i < params.len() {
                        let sub = if params[i].is_empty() { 0 } else { params[i][0] };
                        match sub {
                            5 => {
                                // 256-color
                                i += 1;
                                if i < params.len() {
                                    let n = if params[i].is_empty() { 0 } else { params[i][0] };
                                    self.attrs.fg = color_from_index(n);
                                }
                            }
                            2 => {
                                // Truecolor
                                if i + 3 <= params.len() {
                                    let r = if params[i + 1].is_empty() { 0 } else { params[i + 1][0] };
                                    let g = if params[i + 2].is_empty() { 0 } else { params[i + 2][0] };
                                    let b = if i + 3 < params.len() && !params[i + 3].is_empty() {
                                        params[i + 3][0]
                                    } else {
                                        0
                                    };
                                    self.attrs.fg = TermColor::Rgb(r as u8, g as u8, b as u8);
                                    i += 3;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                39 => self.attrs.fg = TermColor::Default,
                // Standard background colors 40-47
                40..=47 => self.attrs.bg = TermColor::Ansi((p - 40) as u8),
                // Extended background: 48;5;N or 48;2;R;G;B
                48 => {
                    i += 1;
                    if i < params.len() {
                        let sub = if params[i].is_empty() { 0 } else { params[i][0] };
                        match sub {
                            5 => {
                                i += 1;
                                if i < params.len() {
                                    let n = if params[i].is_empty() { 0 } else { params[i][0] };
                                    self.attrs.bg = color_from_index(n);
                                }
                            }
                            2 => {
                                if i + 3 <= params.len() {
                                    let r = if params[i + 1].is_empty() { 0 } else { params[i + 1][0] };
                                    let g = if params[i + 2].is_empty() { 0 } else { params[i + 2][0] };
                                    let b = if i + 3 < params.len() && !params[i + 3].is_empty() {
                                        params[i + 3][0]
                                    } else {
                                        0
                                    };
                                    self.attrs.bg = TermColor::Rgb(r as u8, g as u8, b as u8);
                                    i += 3;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                49 => self.attrs.bg = TermColor::Default,
                // Bright foreground 90-97
                90..=97 => self.attrs.fg = TermColor::Ansi((p - 90 + 8) as u8),
                // Bright background 100-107
                100..=107 => self.attrs.bg = TermColor::Ansi((p - 100 + 8) as u8),
                _ => {} // Ignore unsupported SGR params
            }
            i += 1;
        }
    }

    fn to_term_output(&self, id: &str, name: &str, cols: u16, rows: u16) -> TermOutput {
        let line_count = self.max_row + 1;
        let mut lines = Vec::with_capacity(line_count);

        for row in &self.cells[..line_count] {
            lines.push(collapse_row(row));
        }

        TermOutput {
            id: id.to_string(),
            name: name.to_string(),
            cols,
            rows,
            lines,
        }
    }
}

/// Map a 256-color index to the appropriate TermColor variant.
fn color_from_index(n: u16) -> TermColor {
    let n = n.min(255) as u8;
    if n < 16 {
        TermColor::Ansi(n)
    } else {
        TermColor::Indexed(n)
    }
}

/// Collapse a row of cells into spans, merging adjacent cells with same attributes.
/// Trims trailing default-attr spaces.
fn collapse_row(row: &[Cell]) -> TermLine {
    // Find last non-default-space cell to trim trailing whitespace
    let last_meaningful = row
        .iter()
        .rposition(|c| c.ch != ' ' || c.attrs != CellAttrs::default());

    let end = match last_meaningful {
        Some(i) => i + 1,
        None => return TermLine::empty(),
    };

    let mut spans = Vec::new();
    let mut current_text = String::new();
    let mut current_attrs = row[0].attrs;

    for cell in &row[..end] {
        if cell.attrs == current_attrs {
            current_text.push(cell.ch);
        } else {
            if !current_text.is_empty() {
                spans.push(make_span(&current_text, &current_attrs));
            }
            current_text.clear();
            current_text.push(cell.ch);
            current_attrs = cell.attrs;
        }
    }

    if !current_text.is_empty() {
        spans.push(make_span(&current_text, &current_attrs));
    }

    TermLine::new(spans)
}

fn make_span(text: &str, attrs: &CellAttrs) -> TermSpan {
    TermSpan {
        text: text.to_string(),
        fg: attrs.fg,
        bg: attrs.bg,
        bold: attrs.bold,
        italic: attrs.italic,
        dim: attrs.dim,
        underline: attrs.underline,
    }
}

// -- VTE Perform implementation --

impl vte::Perform for Grid {
    fn print(&mut self, ch: char) {
        self.put_char(ch);
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            b'\r' => self.carriage_return(),
            b'\t' => {
                // Tab: advance to next tab stop (every 8 columns)
                let next_tab = (self.cursor_col / 8 + 1) * 8;
                while self.cursor_col < next_tab && self.cursor_col < self.cols {
                    self.put_char(' ');
                }
            }
            _ => {} // Ignore other C0 controls
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        _intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        // Collect params into a Vec<&[u16]> for easier indexing
        let param_list: Vec<&[u16]> = params.iter().collect();

        match action {
            // SGR - Select Graphic Rendition
            'm' => self.apply_sgr(&param_list),
            // CUU - Cursor Up
            'A' => {
                let n = first_param_at_least_1(&param_list) as usize;
                self.cursor_row = self.cursor_row.saturating_sub(n);
            }
            // CUD - Cursor Down
            'B' => {
                let n = first_param_at_least_1(&param_list) as usize;
                self.cursor_row = (self.cursor_row + n).min(self.rows - 1);
            }
            // CUF - Cursor Forward
            'C' => {
                let n = first_param_at_least_1(&param_list) as usize;
                self.cursor_col = (self.cursor_col + n).min(self.cols - 1);
            }
            // CUB - Cursor Back
            'D' => {
                let n = first_param_at_least_1(&param_list) as usize;
                self.cursor_col = self.cursor_col.saturating_sub(n);
            }
            // CUP - Cursor Position (row;col, 1-based)
            'H' | 'f' => {
                let row = first_param_at_least_1(&param_list) as usize;
                let col = if param_list.len() > 1 && !param_list[1].is_empty() && param_list[1][0] > 0 {
                    param_list[1][0] as usize
                } else {
                    1
                };
                self.cursor_row = (row.saturating_sub(1)).min(self.rows - 1);
                self.cursor_col = (col.saturating_sub(1)).min(self.cols - 1);
            }
            // EL - Erase in Line
            'K' => {
                let mode = first_param(&param_list, 0);
                match mode {
                    0 => {
                        // Clear from cursor to end of line
                        for c in self.cursor_col..self.cols {
                            self.cells[self.cursor_row][c] = Cell::default();
                        }
                    }
                    1 => {
                        // Clear from beginning to cursor
                        for c in 0..=self.cursor_col.min(self.cols - 1) {
                            self.cells[self.cursor_row][c] = Cell::default();
                        }
                    }
                    2 => {
                        // Clear entire line
                        for c in 0..self.cols {
                            self.cells[self.cursor_row][c] = Cell::default();
                        }
                    }
                    _ => {}
                }
            }
            // ED - Erase in Display
            'J' => {
                let mode = first_param(&param_list, 0);
                match mode {
                    0 => {
                        // Clear from cursor to end of display
                        for c in self.cursor_col..self.cols {
                            self.cells[self.cursor_row][c] = Cell::default();
                        }
                        for r in (self.cursor_row + 1)..self.rows {
                            for c in 0..self.cols {
                                self.cells[r][c] = Cell::default();
                            }
                        }
                    }
                    2 | 3 => {
                        // Clear entire display
                        for r in 0..self.rows {
                            for c in 0..self.cols {
                                self.cells[r][c] = Cell::default();
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {} // Ignore unsupported CSI sequences
        }
    }

    fn hook(&mut self, _params: &vte::Params, _intermediates: &[u8], _ignore: bool, _action: char) {}
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}
    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}
    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}
}

fn first_param(params: &[&[u16]], default: u16) -> u16 {
    params
        .first()
        .and_then(|p| p.first().copied())
        .unwrap_or(default)
}

/// Like `first_param` but treats 0 as 1 (per ECMA-48: cursor movement defaults to 1).
fn first_param_at_least_1(params: &[&[u16]]) -> u16 {
    first_param(params, 1).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &[u8]) -> TermOutput {
        parse_ansi(input, 80, 24, "test", "Test")
    }

    // -- Plain text --

    #[test]
    fn plain_text_single_line() {
        let out = parse(b"Hello, world!");
        assert_eq!(out.lines.len(), 1);
        assert_eq!(out.lines[0].spans.len(), 1);
        assert_eq!(out.lines[0].spans[0].text, "Hello, world!");
        assert_eq!(out.lines[0].spans[0].fg, TermColor::Default);
        assert_eq!(out.lines[0].spans[0].bg, TermColor::Default);
    }

    #[test]
    fn plain_text_multiline() {
        let out = parse(b"line 1\r\nline 2\r\nline 3");
        assert_eq!(out.lines.len(), 3);
        assert_eq!(out.lines[0].spans[0].text, "line 1");
        assert_eq!(out.lines[1].spans[0].text, "line 2");
        assert_eq!(out.lines[2].spans[0].text, "line 3");
    }

    #[test]
    fn empty_lines_are_preserved() {
        let out = parse(b"line 1\r\n\r\nline 3");
        assert_eq!(out.lines.len(), 3);
        assert!(out.lines[1].spans.is_empty());
    }

    // -- Basic SGR foreground colors --

    #[test]
    fn sgr_standard_fg_colors() {
        // \e[31m = red fg, \e[32m = green fg
        let out = parse(b"\x1b[31mred\x1b[32mgreen\x1b[0mnormal");
        assert_eq!(out.lines[0].spans.len(), 3);
        assert_eq!(out.lines[0].spans[0].fg, TermColor::Ansi(1));
        assert_eq!(out.lines[0].spans[0].text, "red");
        assert_eq!(out.lines[0].spans[1].fg, TermColor::Ansi(2));
        assert_eq!(out.lines[0].spans[1].text, "green");
        assert_eq!(out.lines[0].spans[2].fg, TermColor::Default);
        assert_eq!(out.lines[0].spans[2].text, "normal");
    }

    #[test]
    fn sgr_standard_bg_colors() {
        // \e[41m = red bg
        let out = parse(b"\x1b[41mred bg\x1b[0m");
        assert_eq!(out.lines[0].spans[0].bg, TermColor::Ansi(1));
    }

    // -- Bright colors --

    #[test]
    fn sgr_bright_fg_colors() {
        // \e[90m = bright black (8), \e[91m = bright red (9)
        let out = parse(b"\x1b[90mdim\x1b[91mbright");
        assert_eq!(out.lines[0].spans[0].fg, TermColor::Ansi(8));
        assert_eq!(out.lines[0].spans[1].fg, TermColor::Ansi(9));
    }

    #[test]
    fn sgr_bright_bg_colors() {
        // \e[100m = bright black bg (8)
        let out = parse(b"\x1b[100mbright bg");
        assert_eq!(out.lines[0].spans[0].bg, TermColor::Ansi(8));
    }

    // -- 256-color mode --

    #[test]
    fn sgr_256_color_fg() {
        // \e[38;5;196m = color 196 (red in cube)
        let out = parse(b"\x1b[38;5;196mred cube");
        assert_eq!(out.lines[0].spans[0].fg, TermColor::Indexed(196));
    }

    #[test]
    fn sgr_256_color_fg_ansi_range() {
        // \e[38;5;1m = should be Ansi(1) since N < 16
        let out = parse(b"\x1b[38;5;1mred");
        assert_eq!(out.lines[0].spans[0].fg, TermColor::Ansi(1));
    }

    #[test]
    fn sgr_256_color_bg() {
        // \e[48;5;232m = dark gray bg
        let out = parse(b"\x1b[48;5;232mdark bg");
        assert_eq!(out.lines[0].spans[0].bg, TermColor::Indexed(232));
    }

    // -- 24-bit truecolor --

    #[test]
    fn sgr_truecolor_fg() {
        // \e[38;2;255;128;0m = orange fg
        let out = parse(b"\x1b[38;2;255;128;0morange");
        assert_eq!(out.lines[0].spans[0].fg, TermColor::Rgb(255, 128, 0));
    }

    #[test]
    fn sgr_truecolor_bg() {
        // \e[48;2;0;128;255m = blue bg
        let out = parse(b"\x1b[48;2;0;128;255mblue bg");
        assert_eq!(out.lines[0].spans[0].bg, TermColor::Rgb(0, 128, 255));
    }

    // -- Text attributes --

    #[test]
    fn sgr_bold() {
        let out = parse(b"\x1b[1mbold text\x1b[0m");
        assert!(out.lines[0].spans[0].bold);
        assert!(!out.lines[0].spans[0].italic);
    }

    #[test]
    fn sgr_italic() {
        let out = parse(b"\x1b[3mitalic\x1b[0m");
        assert!(out.lines[0].spans[0].italic);
    }

    #[test]
    fn sgr_dim() {
        let out = parse(b"\x1b[2mdim\x1b[0m");
        assert!(out.lines[0].spans[0].dim);
    }

    #[test]
    fn sgr_underline() {
        let out = parse(b"\x1b[4munderlined\x1b[0m");
        assert!(out.lines[0].spans[0].underline);
    }

    #[test]
    fn sgr_combined_attrs() {
        // Bold + red + italic in one sequence
        let out = parse(b"\x1b[1;31;3mbold red italic\x1b[0m");
        let span = &out.lines[0].spans[0];
        assert!(span.bold);
        assert!(span.italic);
        assert_eq!(span.fg, TermColor::Ansi(1));
    }

    // -- Reset --

    #[test]
    fn sgr_reset_clears_all() {
        let out = parse(b"\x1b[1;31;3;4;41mstuff\x1b[0mplain");
        let span0 = &out.lines[0].spans[0];
        assert!(span0.bold);
        assert!(span0.italic);
        assert!(span0.underline);
        assert_eq!(span0.fg, TermColor::Ansi(1));
        assert_eq!(span0.bg, TermColor::Ansi(1));

        let span1 = &out.lines[0].spans[1];
        assert!(!span1.bold);
        assert!(!span1.italic);
        assert!(!span1.underline);
        assert_eq!(span1.fg, TermColor::Default);
        assert_eq!(span1.bg, TermColor::Default);
    }

    // -- Span collapsing --

    #[test]
    fn adjacent_same_attrs_merge() {
        // Two prints with same attrs should become one span
        let out = parse(b"\x1b[31mhello \x1b[31mworld");
        assert_eq!(out.lines[0].spans.len(), 1);
        assert_eq!(out.lines[0].spans[0].text, "hello world");
    }

    #[test]
    fn trailing_spaces_trimmed() {
        let out = parse(b"hello");
        // Should only have "hello", not "hello" + 75 spaces
        assert_eq!(out.lines[0].spans[0].text, "hello");
        assert_eq!(out.lines[0].spans.len(), 1);
    }

    // -- Cursor movement --

    #[test]
    fn cursor_up_and_overwrite() {
        // Write on row 0, go to row 1, go back up, overwrite at current col
        let out = parse(b"AAAA\r\nBBBB\x1b[ACCCC");
        // After "AAAA\r\n": row 1, col 0
        // After "BBBB": row 1, col 4
        // After CUU(1): row 0, col 4
        // After "CCCC": row 0 = "AAAACCCC"
        assert_eq!(out.lines.len(), 2);
        assert_eq!(out.lines[0].spans[0].text, "AAAACCCC");
        assert_eq!(out.lines[1].spans[0].text, "BBBB");
    }

    // -- Real-world-ish output --

    #[test]
    fn git_diff_style_output() {
        let input = b"\x1b[1mdiff --git a/foo b/foo\x1b[0m\r\n\
                       \x1b[31m-old line\x1b[0m\r\n\
                       \x1b[32m+new line\x1b[0m\r\n";
        let out = parse(input);
        assert_eq!(out.lines.len(), 3);

        // Header line is bold
        assert!(out.lines[0].spans[0].bold);
        assert_eq!(out.lines[0].spans[0].text, "diff --git a/foo b/foo");

        // Deletion is red
        assert_eq!(out.lines[1].spans[0].fg, TermColor::Ansi(1));
        assert_eq!(out.lines[1].spans[0].text, "-old line");

        // Addition is green
        assert_eq!(out.lines[2].spans[0].fg, TermColor::Ansi(2));
        assert_eq!(out.lines[2].spans[0].text, "+new line");
    }

    // -- Output metadata --

    #[test]
    fn output_metadata() {
        let out = parse_ansi(b"hello", 80, 24, "my-id", "My Name");
        assert_eq!(out.id, "my-id");
        assert_eq!(out.name, "My Name");
        assert_eq!(out.cols, 80);
        assert_eq!(out.rows, 24);
    }

    // -- Default fg/bg reset --

    #[test]
    fn sgr_default_fg_reset() {
        let out = parse(b"\x1b[31mred\x1b[39mdefault");
        assert_eq!(out.lines[0].spans[0].fg, TermColor::Ansi(1));
        assert_eq!(out.lines[0].spans[1].fg, TermColor::Default);
    }

    #[test]
    fn sgr_default_bg_reset() {
        let out = parse(b"\x1b[41mbg\x1b[49mdefault");
        assert_eq!(out.lines[0].spans[0].bg, TermColor::Ansi(1));
        assert_eq!(out.lines[0].spans[1].bg, TermColor::Default);
    }

    // -- Tab handling --

    #[test]
    fn tab_expands_to_spaces() {
        let out = parse(b"a\tb");
        let text = &out.lines[0].spans[0].text;
        // 'a' at col 0, tab advances to col 8, then 'b' at col 8
        assert_eq!(text, "a       b");
    }
}
