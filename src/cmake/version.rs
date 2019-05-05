use std::error::Error;
use std::fmt;

use std::str::FromStr;

// Represents CMake version number
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u8,
    minor: u8,
    revision: Option<u8>,
}

impl Version {
    pub fn new(major: u8, minor: u8) -> Self {
        Version {
            major,
            minor,
            revision: None,
        }
    }

    pub fn with_revision(major: u8, minor: u8, revision: u8) -> Self {
        let revision = if revision == 0 { None } else { Some(revision) };

        Version {
            major,
            minor,
            revision,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)?;

        if let Some(revision) = self.revision {
            write!(f, ".{}", revision)?;
        }

        Ok(())
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<&str> = s.trim().split(".").collect();

        if !(nums.len() > 1 && nums.len() < 4) {
            return Err(ParseVersionError);
        }

        let major: u8 = nums[0].parse()?;
        let minor: u8 = nums[1].parse()?;
        let mut revision: Option<u8> = None;

        if let Some(rev) = nums.get(2) {
            revision = Some(rev.parse()?);
        }

        Ok(Version {
            major,
            minor,
            revision,
        })
    }
}

#[derive(Debug)]
pub struct ParseVersionError;

impl fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ParseVersionError {
    fn description(&self) -> &str {
        "Given string is not a cmake version"
    }
}

impl From<std::num::ParseIntError> for ParseVersionError {
    fn from(_err: std::num::ParseIntError) -> Self {
        ParseVersionError
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let ver1 = Version::new(2, 8);
        let ver2 = Version::with_revision(3, 5, 1);
        let ver3 = Version::from_str("2.8").unwrap();
        let ver4 = Version::from_str("3.5.1").unwrap();

        assert_eq!(ver1.major, 2);
        assert_eq!(ver1.minor, 8);
        assert_eq!(ver1.revision, None);
        assert_eq!(ver1.to_string(), "2.8");

        assert_eq!(ver2.major, 3);
        assert_eq!(ver2.minor, 5);
        assert_eq!(ver2.revision, Some(1));
        assert_eq!(ver2.to_string(), "3.5.1");

        assert_eq!(ver1, ver3);
        assert_eq!(ver2, ver4);
    }

    #[test]
    fn order() {
        let ver1 = Version::new(2, 8);
        let ver2 = Version::new(3, 5);
        let ver3 = Version::with_revision(3, 5, 0);
        let ver4 = Version::with_revision(3, 5, 1);
        let ver5 = Version::with_revision(3, 5, 6);
        let ver6 = Version::with_revision(3, 5, 6);

        assert!(ver1 < ver2 && ver2 == ver3 && ver3 < ver4 && ver4 < ver5 && ver5 <= ver6);
    }
}
