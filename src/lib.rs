#![cfg_attr(feature = "nightly", feature(try_from))]
#[cfg(feature = "nightly")]
use std::convert::TryFrom;
use std::fmt;
use std::io;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OsError {
    code: i32,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NoOsError;

impl OsError {
    /// Creates a new instance of an `OsError` from a particular OS error code.
    ///
    /// # Examples
    ///
    /// On Linux:
    ///
    /// ```
    /// # if cfg!(target_os = "linux") {
    /// use std::io;
    ///
    /// let error = os_error::OsError::new(98);
    /// assert_eq!(error.kind(), io::ErrorKind::AddrInUse);
    /// # }
    /// ```
    ///
    /// On Windows:
    ///
    /// ```
    /// # if cfg!(windows) {
    /// use std::io;
    ///
    /// let error = os_error::OsError::new(10048);
    /// assert_eq!(error.kind(), io::ErrorKind::AddrInUse);
    /// # }
    /// ```
    pub fn new(code: i32) -> OsError {
        OsError { code: code }
    }

    /// Returns an error representing the last OS error which occurred.
    ///
    /// This function reads the value of `errno` for the target platform (e.g.
    /// `GetLastError` on Windows) and will return a corresponding instance of
    /// `OsError` for the error code.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_error::OsError;
    ///
    /// println!("last OS error: {:?}", OsError::last_os_error());
    /// ```
    pub fn last_os_error() -> OsError {
        OsError::new(io::Error::last_os_error().raw_os_error().unwrap())
    }

    /// Returns the OS error that this error represents.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_error::OsError;
    ///
    /// fn main() {
    ///     // Will print "raw OS error: ...".
    ///     println!("raw OS error: {:?}", OsError::last_os_error().code());
    /// }
    /// ```
    pub fn code(&self) -> i32 {
        self.code
    }

    /// Returns the corresponding `ErrorKind` for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_error::OsError;
    ///
    /// fn main() {
    ///     // Will print "No inner error".
    ///     println!("{:?}", OsError::last_os_error());
    /// }
    /// ```
    pub fn kind(&self) -> io::ErrorKind {
        self.to_error().kind()
    }

    fn to_error(&self) -> io::Error {
        io::Error::from_raw_os_error(self.code)
    }
}

impl fmt::Debug for OsError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let error: io::Error = self.to_error();

        fmt.debug_struct("OsError")
            .field("code", &self.code)
            .field("kind", &error.kind())
            .finish()
    }
}

impl fmt::Display for OsError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", &self.to_error())
    }
}

#[cfg(feature = "nightly")]
impl TryFrom<io::Error> for OsError {
    type Error = NoOsError;

    fn try_from(error: io::Error) -> Result<OsError, NoOsError> {
        match error.raw_os_error() {
            Some(code) => Ok(OsError { code }),
            None => Err(NoOsError),
        }
    }
}

impl Into<io::Error> for OsError {
    fn into(self) -> io::Error {
        self.to_error()
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use super::OsError;
    #[cfg(feature = "nightly")]
    use super::NoOsError;
    #[cfg(feature = "nightly")]
    use std::convert::TryFrom;
    #[cfg(feature = "nightly")]
    use std::convert::TryInto;

    const CODE: i32 = 6;

    #[test]
    fn test_fmt_display() {
        let err = OsError::new(CODE);
        let io_error = io::Error::from_raw_os_error(CODE);

        assert_eq!(format!("{}", err), format!("{}", io_error));
    }

    #[test]
    fn test_fmt_debug() {
        let kind = io::ErrorKind::Other;
        let err = OsError::new(CODE);

        let expected = format!("OsError {{ code: {:?}, kind: {:?} }}", CODE, kind);
        assert_eq!(format!("{:?}", err), expected);
    }

    #[test]
    #[cfg(feature = "nightly")]
    fn from_io_error() {
        let os_error = OsError::try_from(io::Error::from_raw_os_error(CODE));
        assert_eq!(os_error, Ok(OsError{ code: CODE }));

        let os_error = OsError::try_from(io::Error::new(io::ErrorKind::AddrInUse, "NoOsError"));
        assert_eq!(os_error, Err(NoOsError));
    }

    #[test]
    #[cfg(feature = "nightly")]
    fn into_os_error() {
        let os_error: Result<OsError, _> = io::Error::from_raw_os_error(CODE).try_into();
        assert_eq!(os_error, Ok(OsError{ code: CODE }));

        let os_error: Result<OsError, _> =
            io::Error::new(io::ErrorKind::AddrInUse, "NoOsError").try_into();
        assert_eq!(os_error, Err(NoOsError));
    }
}
