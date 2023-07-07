use thiserror;
use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, ConeRoboError>;

#[derive(thiserror::Error)]
pub enum ConeRoboError {
    #[error("I-0000: Internal IO error occured")]
    I0000(#[source] std::io::Error),  // Catch-all IO Error
}

impl fmt::Debug for ConeRoboError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn error_chain_fmt(
    e: &impl error::Error,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}