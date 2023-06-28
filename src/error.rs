use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, ConeRoboError>;

pub enum ConeRoboError {
    I0000(std::io::Error),  // Catch-all IO Error
}

impl fmt::Display for ConeRoboError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConeRoboError::I0000(..) => {
                log::error!("I-0000: Internal IO error.");
                write!(f, "I-0000: Internal IO error occured. See log for details.")
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
            ConeRoboError::I0000(ref err) => Some(err)
        }
    }
}

impl From<std::io::Error> for ConeRoboError {
    fn from(e: std::io::Error) -> Self {
        Self::I0000(e)
    }
}