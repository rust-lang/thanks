use std::fmt;
use std::pin::Pin;
use std::ptr::NonNull;

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
    canonical_name: Option<NonNull<str>>,
    canonical_email: Option<NonNull<str>>,
    current_name: Option<NonNull<str>>,
    current_email: Option<NonNull<str>>,
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
    canonical_name: Option<&'a str>,
    canonical_email: Option<&'a str>,
    current_name: Option<&'a str>,
    current_email: Option<&'a str>,
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
    pub name: String,
    pub email: String,
}

impl fmt::Debug for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

impl Author {
    pub fn new(sig: git2::Signature<'_>) -> Self {
        let name = sig.name().unwrap_or_else(|| panic!("no name for {}", sig));
        let email = sig
            .email()
            .unwrap_or_else(|| panic!("no email for {}", sig));

        Author {
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}

impl Mailmap {
    fn read_from_repo(repo: &git2::Repository) -> Result<String, Box<dyn std::error::Error>> {
        Ok(String::from_utf8(
            repo.revparse_single("master")?
                .peel_to_commit()?
                .tree()?
                .get_name(".mailmap")
                .unwrap()
                .to_object(&repo)?
                .peel_to_blob()?
                .content()
                .into(),
        )?)
    }

    pub fn from_repo(repo: &git2::Repository) -> Result<Mailmap, Box<dyn std::error::Error>> {
        Self::from_string(Self::read_from_repo(repo)?)
    }

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
                        return Author {
                            name: entry.canonical_name.unwrap_or(&author.name).to_owned(),
                            email: entry.canonical_email.expect("canonical email").to_owned(),
                        };
                    }
                } else {
                    if author.email == email {
                        return Author {
                            name: entry.canonical_name.unwrap_or(&author.name).to_owned(),
                            email: entry.canonical_email.expect("canonical email").to_owned(),
                        };
                    }
                }
            }
        }

        author.clone()
    }
}

fn read_email<'a>(line: &mut &'a str) -> Option<&'a str> {
    if !line.starts_with('<') {
        return None;
    }

    let end = line
        .find('>')
        .unwrap_or_else(|| panic!("could not find email end in {:?}", line));
    let ret = &line[1..end];
    *line = &line[end + 1..];
    Some(ret)
}

fn read_name<'a>(line: &mut &'a str) -> Option<&'a str> {
    let end = if let Some(end) = line.find('<') {
        end
    } else {
        return None;
    };
    let ret = &line[..end].trim();
    *line = &line[end..];
    if ret.is_empty() {
        None
    } else {
        Some(ret)
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

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_parser {
        ($parser:ident, $input:expr, $exp:expr $(,)*) => {{
            let mut input = $input;
            let ret = $parser(&mut input);
            assert_eq!(input, $exp);
            ret
        }};
    }

    #[test]
    fn comment_1() {
        test_parser!(read_comment, "# foo", "");
    }

    #[test]
    fn comment_2() {
        test_parser!(read_comment, "bar # foo", "bar # foo");
    }

    #[test]
    fn email_1() {
        assert_eq!(
            test_parser!(read_email, "<foo@example.com>", ""),
            Some("foo@example.com")
        );
    }

    #[test]
    fn email_2() {
        assert_eq!(
            test_parser!(
                read_email,
                "<foo@example.com> <foo2@example.com>",
                " <foo2@example.com>"
            ),
            Some("foo@example.com")
        );
    }

    #[test]
    fn email_3() {
        assert_eq!(
            test_parser!(
                read_email,
                "Bar <foo@example.com> <foo2@example.com>",
                "Bar <foo@example.com> <foo2@example.com>",
            ),
            None
        );
    }

    #[test]
    fn name_1() {
        assert_eq!(
            test_parser!(
                read_name,
                "Canonical Name <foo@example.com>",
                "<foo@example.com>"
            ),
            Some("Canonical Name"),
        );
    }

    #[test]
    fn line_1() {
        assert_eq!(
            parse_line("Joe Bob <email1> <email2>", 0),
            Some(MapEntry {
                canonical_name: Some("Joe Bob"),
                canonical_email: Some("email1"),
                current_name: None,
                current_email: Some("email2"),
            })
        );
    }

    #[test]
    fn line_2() {
        assert_eq!(
            parse_line("Joe Bob <email1>", 0),
            Some(MapEntry {
                canonical_name: Some("Joe Bob"),
                canonical_email: Some("email1"),
                current_name: None,
                current_email: Some("email1"),
            })
        );
    }
}
