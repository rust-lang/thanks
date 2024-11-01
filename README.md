# Thanks!

[![Build Status][status-img]][status]

[status-img]: https://github.com/rust-lang/thanks/actions/workflows/ci.yml/badge.svg?branch=master
[status]: https://github.com/rust-lang/thanks/actions/workflows/ci.yml

This is a static site generator showing people who have contributed to Rust.

To run thanks, you'll need stable Rust.

There is no need to configure anything; thanks will run everything on its own, simply `cargo run
--release` and the site will be placed in `output`.

## Identities (or "help, I'm showing up multiple times")

Thanks aggregates data from commits and PR reviews.
It bases identities on the email addresses from the commits and GitHub usernames from approvals (which it maps to email addresses via rust-lang/team).
It then uses the `.mailmap` in the rust-lang/rust repository to canonicalize the identities.

If you show up multiple times, it's likely that you have contributed under multiple email addresses and haven't added them to the mailmap.
To do this, add yourself to the mailmap `.mailmap` like the example here:

```
Ferris <ferris@rust-lang.org>
Ferris <ferris@rust-lang.org> <ferris@old1.rust-lang.org>
Ferris <ferris@rust-lang.org> <ferris@old2.rust-lang.org>
```

If you change your GitHub username or someone mistyped your GitHub username in an `r=` comment, you can re-map it by adding an entry in `src/reviewers.rs`.
This is also useful if your GitHub username is not present in rust-lang/team or you encrypted the email there.

Use the `DEBUG_EMAILS=1` environment variable locally to display the email address in the output,
which is useful for debugging these missing mailmap entries.

## Refresh time

Thanks is configured to run every night to update the latest statistics.
