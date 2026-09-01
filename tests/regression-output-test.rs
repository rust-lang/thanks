// Test that we have the expected output for each version from 0.1.0 through
// 1.95.0 (and the all-time totals from those versions).
// The expected output is compared against the CSV output generated before the
// test was run; use `cargo run --release -- csv` to create that output.
// Set `TESTS_UPDATE_EXPECTED=1` before running the test to update the expected
// output files automatcally.
// Since the output can change as the rust-lang/rust .mailmap file is adjusted,
// these tests are run against the .mailmap as of commit
// 0490dd938541ad996c5ad1ec6e274012afe3e1d4, see .github/workflows/ci.yml
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

struct TestFailure {
    file: String,
    diff: String,
}

fn assert_file_content_matches(
    expected: &Path,
    actual: &Path,
    version: &OsStr,
) -> Result<(), TestFailure> {
    use std::fmt::Write;

    // Running the thanks command via `std::process::Command` is a lot slower
    // that if we just require that it have been run beforehand
    let actual_content = fs::read_to_string(actual).expect(&format!(
        "The actual output at {} should exist. Did you forget to run \
            thanks before running the tests?",
        actual.display()
    ));
    if std::env::var("TESTS_UPDATE_EXPECTED").is_ok() {
        fs::write(expected, &actual_content).expect("Able to write to the expected output file");
    }

    let expected_content = fs::read_to_string(expected).unwrap();

    // Print the content as multiline strings rather than just all on one line
    // the way assert_eq! would, so that various diff tools can be used to
    // understand the comparison
    if expected_content != actual_content {
        let mut total_diff = String::new();
        for diff in diff::lines(&expected_content, &actual_content) {
            match diff {
                diff::Result::Left(l) => writeln!(total_diff, "-{l}").unwrap(),
                diff::Result::Both(l, _) => writeln!(total_diff, " {l}").unwrap(),
                diff::Result::Right(r) => writeln!(total_diff, "+{r}").unwrap(),
            }
        }

        Err(TestFailure {
            file: version.to_str().unwrap().to_string(),
            diff: total_diff,
        })
    } else {
        Ok(())
    }
}

#[test]
fn verify_generated_output() {
    let output_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("output")
        .join("csv");

    let expectation_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("expected");

    let mut failures = vec![];
    for entry in fs::read_dir(expectation_dir).unwrap() {
        let entry = entry.unwrap();
        let expected_file = entry.path();
        assert_eq!("csv", expected_file.extension().unwrap());
        let binding = expected_file.with_extension("csv");
        let version = binding.file_name().unwrap();
        if let Err(failure) =
            assert_file_content_matches(&expected_file, &output_dir.join(version), version)
        {
            failures.push(failure);
        }
    }

    failures.sort_by_key(|f| f.file.clone());
    for failure in &failures {
        eprintln!("Diff failed for {}", failure.file);
        eprintln!("----------");
        eprintln!("{}", failure.diff);
        eprintln!("----------");
    }
    if !failures.is_empty() {
        panic!(
            r#"Diffs failed for {}

Run with TESTS_UPDATE_EXPECTED=1 to bless the expected snapshots.
"#,
            failures
                .into_iter()
                .map(|f| f.file)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
