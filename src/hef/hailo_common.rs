use std::fmt;

/// Defines the default batch size used across Hailo configurations.
pub const HAILO_DEFAULT_BATCH_SIZE: u32 = 0;

/// Represents a generic result type for Hailo operations.
pub type HailoResult<T> = Result<T, HailoError>;

/// Enum for common Hailo error codes.
#[derive(Debug, Clone, PartialEq)]
pub enum HailoError {
    InvalidOperation(String),
    ParsingError(String),
    IoError(String),
    UnsupportedVersion(u32),
    MissingData(String),
}

impl fmt::Display for HailoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HailoError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            HailoError::ParsingError(msg) => write!(f, "Parsing error: {}", msg),
            HailoError::IoError(msg) => write!(f, "I/O error: {}", msg),
            HailoError::UnsupportedVersion(version) => {
                write!(f, "Unsupported version: {}", version)
            }
            HailoError::MissingData(msg) => write!(f, "Missing data: {}", msg),
        }
    }
}

impl std::error::Error for HailoError {}

/// Alignment utilities.
pub mod alignment {
    /// Aligns a given value `x` to the nearest higher multiple of `alignment`.
    pub fn align_up(x: u32, alignment: u32) -> u32 {
        if alignment == 0 {
            x
        } else {
            ((x + alignment - 1) / alignment) * alignment
        }
    }

    /// Checks if a given value `x` is aligned to `alignment`.
    pub fn is_aligned(x: u32, alignment: u32) -> bool {
        if alignment == 0 {
            true
        } else {
            x % alignment == 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_up() {
        assert_eq!(alignment::align_up(13, 4), 16);
        assert_eq!(alignment::align_up(16, 4), 16);
        assert_eq!(alignment::align_up(0, 4), 0);
        assert_eq!(alignment::align_up(7, 0), 7);
    }

    #[test]
    fn test_is_aligned() {
        assert!(alignment::is_aligned(16, 4));
        assert!(!alignment::is_aligned(13, 4));
        assert!(alignment::is_aligned(7, 0));
    }

    #[test]
    fn test_hailo_error_display() {
        let err = HailoError::InvalidOperation("Operation not allowed".to_string());
        assert_eq!(err.to_string(), "Invalid operation: Operation not allowed");
    }
}
