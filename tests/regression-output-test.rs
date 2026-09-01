//! Test that we have the expected output for each version from 0.1.0 through
//! 1.95.0 (and the all-time totals from those versions).
//! Since the output can change as the rust-lang/rust .mailmap file is adjusted,
//! these tests are run against the .mailmap as of the REFERENCE_COMMIT.
//!
//! Set `TESTS_UPDATE_EXPECTED=1` when running the test to update the expected
//! output files automatically.
//!
//! Set `TESTS_IN_PLACE=1` when running the test to use pre-generated data that you built locally
//! with `cargo run --release -- csv`.
//! Note that you should remove local git repositories in the `repos` directory and then do a bare
//! checkout of `rust-lang/rust` into that directory and switch its main branch to REFERENCE_COMMIT
//! before running thanks, otherwise the results might differ.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Reference Rust commit against which we compare the stored snapshots.
/// Corresponds to ~Rust 1.95.0 in May 2026.
const REFERENCE_COMMIT: &str = "0490dd938541ad996c5ad1ec6e274012afe3e1d4";

/// Checks that the generated `actual` CSV file matches the `expected` snapshot.
fn check_file(expected: &Path, actual: &Path) -> Result<(), String> {
    use std::fmt::Write;

    let actual_content = fs::read_to_string(actual).expect(&format!(
        "The actual output at {} should exist. Thanks did not generate it",
        actual.display()
    ));
    if std::env::var("TESTS_UPDATE_EXPECTED").is_ok() {
        fs::write(expected, &actual_content).expect("Unable to write to the expected output file");
    }

    let expected_content = fs::read_to_string(expected).unwrap();

    // Output the content as multiline strings rather than just all on one line
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

        Err(total_diff)
    } else {
        Ok(())
    }
}

/// Check that all files match expectations in the given `actual_dir`.
fn check_dir(expected_dir: &Path, actual_dir: &Path) {
    let mut files = std::fs::read_dir(expected_dir)
        .unwrap()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    files.sort_by_key(|e| e.path());

    let mut failures = vec![];
    for entry in files {
        let expected_file = entry.path();
        assert_eq!("csv", expected_file.extension().unwrap());
        let binding = expected_file.with_extension("csv");
        let version = binding.file_name().unwrap();
        if let Err(failure) = check_file(&expected_file, &actual_dir.join(version)) {
            failures.push((version.to_str().unwrap().to_string(), failure));
        }
    }

    failures.sort_by_key(|(version, _)| version.clone());
    for (version, diff) in &failures {
        eprintln!("Diff failed for {version}");
        eprintln!("----------");
        eprintln!("{diff}");
        eprintln!("----------");
    }
    if !failures.is_empty() {
        panic!(
            r#"Diffs failed for {}

Run with TESTS_UPDATE_EXPECTED=1 to bless the expected snapshots.
"#,
            failures
                .into_iter()
                .map(|(version, _)| version)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

#[test]
fn verify_generated_output() {
    let expected_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("expected");

    // For faster local tests, we allow running in-place on pre-generated data
    let (output_dir, _tmpdir) = if std::env::var("TESTS_IN_PLACE").is_ok_and(|v| v == "1") {
        (
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("output")
                .join("csv"),
            None,
        )
    } else {
        let binary = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("release")
            .join("thanks");
        assert!(
            binary.is_file(),
            "target/release/thanks does not exist. Please build it first with cargo build --release"
        );
        let tmp_dir = tempfile::TempDir::new().unwrap();
        let root_dir = tmp_dir.path();
        eprintln!("Generating thanks data in {}", root_dir.display());

        // Clone rust-lang/rust
        run(Command::new("git").current_dir(root_dir).args([
            "clone",
            "--bare",
            "https://github.com/rust-lang/rust.git",
            "repos/rust-lang/rust",
        ]));
        // Set it to the reference commit
        run(Command::new("git")
            .current_dir(root_dir.join("repos/rust-lang/rust"))
            .args(["branch", "-f", "main", REFERENCE_COMMIT]));
        // Run thanks
        run(Command::new(binary).current_dir(root_dir).arg("csv"));

        (root_dir.join("output").join("csv"), Some(tmp_dir))
    };
    check_dir(&expected_dir, &output_dir);
}

fn run(command: &mut Command) {
    eprintln!("Running command {command:?}");
    let status = command.status().expect("Cannot run command");
    if !status.success() {
        panic!(
            "Command failed with exit code {}",
            status.code().unwrap_or(-1)
        );
    }
}
