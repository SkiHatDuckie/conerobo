use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, ConeRoboError>;

pub enum ConeRoboError {
    I0000(std::io::Error),  // Catch-all IO Error
    I0001(String),
    I0002(String),
}

impl fmt::Display for ConeRoboError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConeRoboError::I0000(_) => {
                write!(f, "I-0000: Internal IO error occured")
            } ConeRoboError::I0001(ref e) => {
                write!(f, "I-0001: Menu option index \"{0}\" out of bounds", e)
            } ConeRoboError::I0002(ref e) => {
                write!(f, "I-0002: Next menu \"{0}\" does not exist", e)
            }
        }
    }
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

impl error::Error for ConeRoboError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ConeRoboError::I0000(ref err) => Some(err),
            ConeRoboError::I0001(_) => None,
            ConeRoboError::I0002(_) => None
        }
    }
}