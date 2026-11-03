#![allow(missing_docs)]

use std::{error::Error as StdError, fmt, io};

use err_report::Report;

#[derive(Debug)]
struct WrappedError<E>(E);

impl<E> fmt::Display for WrappedError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Wrapped")
    }
}

impl<E> StdError for WrappedError<E>
where
    E: StdError + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[test]
fn standalone_error_single_line() {
    assert_eq!(
        "Verbatim",
        Report::new(io::Error::other("Verbatim")).to_string(),
    );

    assert_eq!(
        "Passed non-utf8 string",
        Report::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Passed non-utf8 string"
        ))
        .to_string(),
    );
}

#[test]
fn wrapped_error_single_line() {
    assert_eq!(
        "Wrapped: Verbatim",
        Report::new(WrappedError(io::Error::other("Verbatim"))).to_string(),
    );

    assert_eq!(
        "Wrapped: Passed non-utf8 string",
        Report::new(WrappedError(io::Error::new(
            io::ErrorKind::InvalidData,
            "Passed non-utf8 string"
        )))
        .to_string(),
    );
}

#[test]
fn standalone_error_pretty() {
    assert_eq!(
        "Verbatim",
        Report::new(io::Error::other("Verbatim"))
            .pretty(true)
            .to_string(),
    );

    assert_eq!(
        "Passed non-utf8 string",
        Report::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Passed non-utf8 string"
        ))
        .pretty(true)
        .to_string(),
    );
}

#[test]
fn wrapped_error_pretty() {
    assert_eq!(
        indoc::indoc! {"
            Wrapped
            
            Caused by:
                  Verbatim"},
        Report::new(WrappedError(io::Error::other("Verbatim")))
            .pretty(true)
            .to_string(),
    );

    assert_eq!(
        indoc::indoc! {"
            Wrapped

            Caused by:
                  Root error"},
        Report::new(WrappedError(io::Error::other("Root error")))
            .pretty(true)
            .to_string(),
    );

    assert_eq!(
        indoc::indoc! {"
            Wrapped

            Caused by:
               0: Wrapped
               1: Root error"},
        Report::new(WrappedError(WrappedError(io::Error::other("Root error"))))
            .pretty(true)
            .to_string(),
    );
}

#[test]
fn debug_matches_display() {
    let report = Report::new(WrappedError(io::Error::other("Verbatim")));
    assert_eq!(format!("{report:?}"), report.to_string());

    let report = report.pretty(true);
    assert_eq!(format!("{report:?}"), report.to_string());
}

#[test]
fn nested_error_single_line() {
    let report = Report::new(WrappedError(WrappedError(io::Error::other("Root error"))));
    assert_eq!(report.to_string(), "Wrapped: Wrapped: Root error");
}

#[test]
fn multiline_source_pretty() {
    let report = Report::new(WrappedError(io::Error::other("Root\nerror"))).pretty(true);
    assert_eq!(
        report.to_string(),
        "Wrapped\n\nCaused by:\n      Root\n      error"
    );
}

#[derive(Debug)]
enum CyclicError {
    SelfCycle,
    First,
    Second,
}

static SELF_CYCLE: CyclicError = CyclicError::SelfCycle;
static FIRST: CyclicError = CyclicError::First;
static SECOND: CyclicError = CyclicError::Second;

impl fmt::Display for CyclicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::SelfCycle => "Cycle",
            Self::First => "First",
            Self::Second => "Second",
        })
    }
}

impl StdError for CyclicError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            Self::SelfCycle => &SELF_CYCLE,
            Self::First => &SECOND,
            Self::Second => &FIRST,
        })
    }
}

// Stop a broken formatter before it can loop forever or exhaust memory.
struct BoundedOutput(String);

impl fmt::Write for BoundedOutput {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        if self.0.len() + text.len() > 4096 {
            return Err(fmt::Error);
        }

        self.0.push_str(text);
        Ok(())
    }
}

fn bounded_report(error: CyclicError, pretty: bool) -> String {
    use fmt::Write as _;

    let mut output = BoundedOutput(String::new());
    write!(output, "{}", Report::new(error).pretty(pretty))
        .expect("formatting must stop before the output limit");
    output.0
}

#[test]
fn cyclic_sources_single_line() {
    assert_eq!(
        bounded_report(CyclicError::SelfCycle, false),
        "Cycle: Cycle"
    );
    assert_eq!(
        bounded_report(CyclicError::First, false),
        "First: Second: First"
    );
}

#[test]
fn cyclic_sources_pretty() {
    assert_eq!(
        bounded_report(CyclicError::SelfCycle, true),
        "Cycle\n\nCaused by:\n   0: Cycle"
    );
    assert_eq!(
        bounded_report(CyclicError::First, true),
        "First\n\nCaused by:\n   0: Second\n   1: First"
    );
}
