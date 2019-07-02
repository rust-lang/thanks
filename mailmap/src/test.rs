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
