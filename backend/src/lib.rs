//! backend/src/lib.rs
pub mod authentication;
pub mod configuration;
pub mod routes;
pub mod session_wrapper;
pub mod startup;
pub mod surrealdb_repo;
pub mod telemetry;

/// Helper that iterates through chain of errors to provide the root cause.
/// Made to be used in `Debug` implements for `Error` types!
pub fn error_chain_helper(
    err: &impl std::error::Error,
    fmt: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    // Write into bugger
    writeln!(fmt, "{}\n", err)?;
    let mut current_err = err.source();

    // Will run until `current` gives `None`
    while let Some(cause) = current_err {
        writeln!(fmt, "Caused by:\n\t{}", cause)?;
        current_err = cause.source();
    }
    Ok(())
}
