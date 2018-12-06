use std::cmp::{self, Ordering};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};
use std::rc::Rc;

pub enum Error {
    SubspanRangeError,
}

/// A small, `Copy`, value representing a position in a `CodeMap`'s file.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Pos(u32);

impl Add<u64> for Pos {
    type Output = Pos;
    fn add(self, other: u64) -> Pos {
        Pos(self.0 + other as u32)
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0)
    }
}

impl Sub<Pos> for Pos {
    type Output = u64;
    fn sub(self, other: Pos) -> u64 {
        (self.0 - other.0) as u64
    }
}

impl Pos {
    pub fn from_usize(u: usize) -> Self {
        Pos(u as u32)
    }
}

/// A range of text within a `CodeMap`
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Span {
    pub low: Pos,
    pub high: Pos,
}

impl Span {
    /// Makes a span from offsets relative to the start of this span.
    pub fn subspan(&self, begin: u64, end: u64) -> Result<Span, Error> {
        if end < begin {
            return Err(Error::SubspanRangeError);
        }
        if self.low + end > self.high {
            return Err(Error::SubspanRangeError);
        }

        Ok(Span {
            low: self.low + begin,
            high: self.low + end,
        })
    }

    /// Checks if a span is contained within this span.
    pub fn contains(&self, other: Span) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    /// The position in the codemap representing the first byte of the span.
    pub fn low(&self) -> Pos {
        self.low
    }

    /// The position after the last byte of the span.
    pub fn high(&self) -> Pos {
        self.high
    }

    /// The length in bytes of the text of the span
    pub fn len(&self) -> u64 {
        self.high - self.low
    }

    /// Create a span that encloses both `self` and `other`.
    pub fn merge(&self, other: Span) -> Span {
        Span {
            low: cmp::min(self.low, other.low),
            high: cmp::max(self.high, other.high),
        }
    }
}

pub const DUMMY_SPAN: Span = Span {
    low: Pos(0),
    high: Pos(0),
};

pub struct SourceMap {
    files: Vec<Rc<SourceFile>>,
    _allow_priv: (),
}

impl SourceMap {
    pub fn new() -> SourceMap {
        SourceMap {
            files: vec![],
            _allow_priv: (),
        }
    }

    pub fn add_file(&mut self, name: String, source: String) -> Rc<SourceFile> {
        let low = self.end_pos() + 1;
        let high = low + source.len() as u64;
        let mut lines = vec![low];
        lines.extend(
            source
                .match_indices('\n')
                .map(|(p, _)| low + (p + 1) as u64),
        );

        let file = Rc::new(SourceFile {
            span: Span { low, high },
            name,
            source,
            lines,
        });

        self.files.push(file.clone());
        file
    }

    fn end_pos(&self) -> Pos {
        self.files.last().map(|x| x.span.high).unwrap_or(Pos(0))
    }

    pub fn find_file(&self, pos: Pos) -> &Rc<SourceFile> {
        self.files
            .binary_search_by(|file| {
                if file.span.high < pos {
                    Ordering::Less
                } else if file.span.low > pos {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .ok()
            .map(|i| &self.files[i])
            .expect("Mapping unknown source location")
    }

    pub fn look_up_pos(&self, pos: Pos) -> Position {
        let file = self.find_file(pos);
        let position = file.find_line_col(pos).expect("Failed to find position");
        Position {
            file: file.clone(),
            position,
        }
    }

    pub fn look_up_span(&self, span: Span) -> SpanPosition {
        let file = self.find_file(span.low);
        let begin = file
            .find_line_col(span.low)
            .expect("Mapping unknown source location");
        let end = file
            .find_line_col(span.high)
            .expect("Mapping unknown source location");
        SpanPosition {
            file: file.clone(),
            begin,
            end,
        }
    }
}

pub struct SourceFile {
    pub span: Span,
    name: String,
    source: String,
    lines: Vec<Pos>,
}

impl SourceFile {
    /// Gets the name of the file
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the line number of a Pos.
    ///
    /// The lines are 0-indexed (first line is numbered 0)
    pub fn find_line(&self, pos: Pos) -> Result<usize, ()> {
        if pos < self.span.low {
            return Err(()); // TODO: Error reporting
        }
        if pos > self.span.high {
            return Err(()); // TODO: Error reporting
        }
        match self.lines.binary_search(&pos) {
            Ok(i) => Ok(i),
            Err(i) => Ok(i - 1),
        }
    }

    pub fn find_line_col(&self, pos: Pos) -> Result<LineCol, ()> {
        let line = self.find_line(pos)?;
        let line_span = self.line_span(line)?;
        let byte_col = pos - line_span.low;
        let column = self.source_slice(line_span)?[..byte_col as usize]
            .chars()
            .count();

        Ok(LineCol { line, column })
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn source_slice(&self, span: Span) -> Result<&str, ()> {
        if !self.span.contains(span) {
            return Err(()); // TODO: Error reporting
        }
        Ok(&self.source
            [((span.low - self.span.low) as usize)..((span.high - self.span.low) as usize)])
    }

    pub fn line_span(&self, line: usize) -> Result<Span, ()> {
        if line >= self.lines.len() {
            return Err(()); // TODO: Error reporting
        }
        Ok(Span {
            low: self.lines[line],
            high: *self.lines.get(line + 1).unwrap_or(&self.span.high),
        })
    }

    pub fn source_line(&self, line: usize) -> Result<&str, ()> {
        Ok(self
            .source_slice(self.line_span(line)?)?
            .trim_right_matches(&['\n', '\r'][..]))
    }

    pub fn num_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn start_pos(&self) -> Pos {
        self.span.low
    }

    pub fn end_pos(&self) -> Pos {
        self.span.high
    }
}

impl fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "File({:?})", self.name)
    }
}

impl PartialEq for SourceFile {
    fn eq(&self, other: &SourceFile) -> bool {
        self as *const _ == other as *const _
    }
}

impl Eq for SourceFile {}

impl Hash for SourceFile {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.span.hash(hasher);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LineCol {
    pub line: usize,
    pub column: usize,
}

pub struct Position {
    pub file: Rc<SourceFile>,
    pub position: LineCol,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}:{}:{}",
            self.file.name,
            self.position.line + 1,
            self.position.column + 1
        )
    }
}

pub struct SpanPosition {
    pub file: Rc<SourceFile>,
    pub begin: LineCol,
    pub end: LineCol,
}

impl fmt::Display for SpanPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.begin == self.end {
            write!(
                f,
                "{}:{}:{}",
                self.file.name,
                self.begin.line + 1,
                self.begin.column + 1
            )
        } else {
            write!(
                f,
                "{}:{}:{}: {}:{}",
                self.file.name,
                self.begin.line + 1,
                self.begin.column + 1,
                self.end.line + 1,
                self.end.column + 1
            )
        }
    }
}
