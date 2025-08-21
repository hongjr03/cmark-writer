//! Utility functions and character escaping functionality.

use std::borrow::Cow;

/// A trait for character escaping behavior
pub trait Escapes {
    /// Checks if the string needs escaping
    fn str_needs_escaping(s: &str) -> bool;

    /// Returns true if the character needs to be escaped
    fn char_needs_escaping(c: char) -> bool;

    /// Returns the escaped version of a character (if needed)
    fn escape_char(c: char) -> Option<&'static str>;
}

/// Markdown escaping implementation for CommonMark
pub struct CommonMarkEscapes;

impl Escapes for CommonMarkEscapes {
    fn str_needs_escaping(s: &str) -> bool {
        s.chars().any(Self::char_needs_escaping)
    }

    fn char_needs_escaping(c: char) -> bool {
        matches!(c, '\\' | '*' | '_' | '[' | ']' | '<' | '>' | '`')
    }

    fn escape_char(c: char) -> Option<&'static str> {
        match c {
            '\\' => Some(r"\\"),
            '*' => Some(r"\*"),
            '_' => Some(r"\_"),
            '[' => Some(r"\["),
            ']' => Some(r"\]"),
            '<' => Some(r"\<"),
            '>' => Some(r"\>"),
            '`' => Some(r"\`"),
            _ => None,
        }
    }
}

/// A wrapper for efficient escaping
pub struct Escaped<'a, E: Escapes> {
    inner: &'a str,
    _phantom: std::marker::PhantomData<E>,
}

impl<'a, E: Escapes> Escaped<'a, E> {
    /// Create a new Escaped wrapper
    pub fn new(s: &'a str) -> Self {
        Self {
            inner: s,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E: Escapes> std::fmt::Display for Escaped<'_, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.inner.chars() {
            if E::char_needs_escaping(c) {
                f.write_str(E::escape_char(c).unwrap())?;
            } else {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

/// Escapes a string using the specified escaping strategy
pub fn escape_str<E: Escapes>(s: &str) -> Cow<'_, str> {
    if E::str_needs_escaping(s) {
        Cow::Owned(format!("{}", Escaped::<E>::new(s)))
    } else {
        Cow::Borrowed(s)
    }
}
