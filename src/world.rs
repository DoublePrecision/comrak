use std::fmt::Display;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::Library;
use typst::LibraryExt;
use typst::World;

#[derive(Debug)]
pub struct TypstMathError;

impl Display for TypstMathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TypstMathError")
    }
}

pub struct PartialLazyWorld {
    pub fonts: Box<[Font]>,
    pub book: LazyHash<FontBook>,
    pub library: LazyHash<Library>,
}

static PARTIAL_WORLD: Lazy<PartialLazyWorld> = Lazy::new(|| preload());

/// Main interface that determines the environment for Typst.
pub struct MinimalWorld {
    /// The content of a source.
    source: Source,
}

fn preload() -> PartialLazyWorld {
    let mut book = FontBook::new();
    let mut fonts = Vec::new();

    let buffer = Bytes::new(include_bytes!("../NewCMMath-Regular.otf"));
    for font in Font::iter(buffer) {
        book.push(font.info().clone());
        fonts.push(font);
    }

    let fonts = fonts.into_boxed_slice();

    PartialLazyWorld {
        fonts,
        book: LazyHash::new(book),
        library: LazyHash::new(Library::default()),
    }
}

impl MinimalWorld {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: Source::detached(source),
        }
    }
}

impl World for MinimalWorld {
    /// Standard library.
    fn library(&self) -> &LazyHash<Library> {
        &PARTIAL_WORLD.library
    }

    /// Metadata about all known Books.
    fn book(&self) -> &LazyHash<FontBook> {
        &PARTIAL_WORLD.book
    }

    /// Accessing the main source file.
    fn main(&self) -> FileId {
        self.source.id()
    }

    /// Accessing a specified source file (based on `FileId`).
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(PathBuf::new()))
        }
    }

    /// Accessing a specified file (non-file).
    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(PathBuf::new()))
    }

    /// Accessing a specified font per index of font book.
    fn font(&self, id: usize) -> Option<Font> {
        PARTIAL_WORLD.fonts.get(id).cloned()
    }

    /// Get the current date.
    ///
    /// Optionally, an offset in hours is given.
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None
    }
}
