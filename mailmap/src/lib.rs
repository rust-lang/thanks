use std::fmt;
use std::hash::Hash;
use std::pin::Pin;
use std::ptr::NonNull;

use uncased::{Uncased, UncasedStr};

#[cfg(test)]
mod test;

/// Loads a mailmap from the string passed in.
///
/// The format is the same as used by `git`; specifically:
///
/// * `Canonical Name <canonical email> Current Name <current email>`
///   * This changes authors matching both name and email to the canonical forms.
/// * `Canonical Name <current email>`
///   * This changes all entries with this email to new name, regardless of their current name.
/// * `Canonical Name <canonical email> <current email>`
///   * This changes all entries with the current email to the canonical name and email.
/// * `<canonical email> <current email>`
///   * This changes all entries with the current email to the canonical email.
pub struct Mailmap {
    buffer: Pin<Box<str>>,
    entries: Vec<RawMapEntry>,
}

impl fmt::Debug for Mailmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for entry in &self.entries {
            // these entries were created from this buffer
            let entry = unsafe { entry.to_entry(&self.buffer) };
            list.entry(&entry);
        }
        list.finish()
    }
}

#[derive(Copy, Clone)]
struct RawMapEntry {
    canonical_name: Option<NonNull<UncasedStr>>,
    canonical_email: Option<NonNull<UncasedStr>>,
    current_name: Option<NonNull<UncasedStr>>,
    current_email: Option<NonNull<UncasedStr>>,
}

impl RawMapEntry {
    unsafe fn to_entry<'a>(self, _: &'a str) -> MapEntry<'a> {
        MapEntry {
            canonical_name: self.canonical_name.map(|v| &*v.as_ptr()),
            canonical_email: self.canonical_email.map(|v| &*v.as_ptr()),
            current_name: self.current_name.map(|v| &*v.as_ptr()),
            current_email: self.current_email.map(|v| &*v.as_ptr()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct MapEntry<'a> {
    canonical_name: Option<&'a UncasedStr>,
    canonical_email: Option<&'a UncasedStr>,
    current_name: Option<&'a UncasedStr>,
    current_email: Option<&'a UncasedStr>,
}

impl<'a> MapEntry<'a> {
    fn to_raw_entry(self) -> RawMapEntry {
        RawMapEntry {
            canonical_name: self.canonical_name.map(|v| v.into()),
            canonical_email: self.canonical_email.map(|v| v.into()),
            current_name: self.current_name.map(|v| v.into()),
            current_email: self.current_email.map(|v| v.into()),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Author {
    pub name: Uncased<'static>,
    pub email: Uncased<'static>,
}

impl Author {
    pub fn new(name: String, email: String) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
        }
    }
}

impl fmt::Debug for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

impl Mailmap {
    pub fn from_string(file: String) -> Result<Mailmap, Box<dyn std::error::Error>> {
        let file = Pin::new(file.into_boxed_str());
        let mut entries = Vec::with_capacity(file.lines().count());
        for (idx, line) in file.lines().enumerate() {
            if let Some(entry) = parse_line(&line, idx + 1) {
                entries.push(entry.to_raw_entry());
            }
        }
        Ok(Mailmap {
            buffer: file,
            entries,
        })
    }

    pub fn canonicalize(&self, author: &Author) -> Author {
        for entry in &self.entries {
            // these entries were created from this buffer
            let entry = unsafe { entry.to_entry(&self.buffer) };
            if let Some(email) = entry.current_email {
                if let Some(name) = entry.current_name {
                    if author.name == name && author.email == email {
                        return Author::new(
                            entry.canonical_name.unwrap_or(&author.name).to_string(),
                            entry.canonical_email.expect("canonical email").to_string(),
                        );
                    }
                } else {
                    if author.email == email {
                        return Author::new(
                            entry.canonical_name.unwrap_or(&author.name).to_string(),
                            entry.canonical_email.expect("canonical email").to_string(),
                        );
                    }
                }
            }
        }

        author.clone()
    }
}

fn read_email<'a>(line: &mut &'a str) -> Option<&'a UncasedStr> {
    if !line.starts_with('<') {
        return None;
    }

    let end = line
        .find('>')
        .unwrap_or_else(|| panic!("could not find email end in {:?}", line));
    let ret = &line[1..end];
    *line = &line[end + 1..];
    Some(ret.into())
}

fn read_name<'a>(line: &mut &'a str) -> Option<&'a UncasedStr> {
    let end = if let Some(end) = line.find('<') {
        end
    } else {
        return None;
    };
    let ret = line[..end].trim();
    *line = &line[end..];
    if ret.is_empty() {
        None
    } else {
        Some(ret.into())
    }
}

fn read_comment(line: &mut &str) -> bool {
    if line.trim().starts_with('#') {
        *line = "";
        true
    } else {
        false
    }
}

fn parse_line(mut line: &str, idx: usize) -> Option<MapEntry<'_>> {
    let mut entry = MapEntry {
        canonical_name: None,
        canonical_email: None,
        current_name: None,
        current_email: None,
    };
    loop {
        line = line.trim_start();
        if read_comment(&mut line) || line.trim().is_empty() {
            break;
        }

        if let Some(email) = read_email(&mut line) {
            if entry.canonical_email.is_none() {
                entry.canonical_email = Some(email);
            } else {
                if entry.current_email.is_some() {
                    eprintln!("malformed mailmap on line {}: too many emails", idx);
                } else {
                    entry.current_email = Some(email);
                }
            }
        } else if let Some(name) = read_name(&mut line) {
            if entry.canonical_name.is_none() {
                entry.canonical_name = Some(name);
            } else {
                if entry.current_name.is_some() {
                    eprintln!("malformed mailmap on line {}: too many names", idx);
                } else {
                    entry.current_name = Some(name);
                }
            }
        } else {
            break;
        }
    }

    if entry.canonical_email.is_some() && entry.current_email.is_none() {
        entry.current_email = entry.canonical_email;
    }

    if entry.canonical_name.is_some()
        || entry.canonical_email.is_some()
        || entry.current_name.is_some()
        || entry.current_email.is_some()
    {
        Some(entry)
    } else {
        None
    }
}
