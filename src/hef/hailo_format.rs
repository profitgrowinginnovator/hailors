use serde::{Serialize, Deserialize};
/// Enum representing the type of a Hailo format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HailoFormatType {
    Float32,
    Uint8,
    Int8,
    Float16,
    Other(u32), // Catch-all for unknown types
}

/// Enum representing the order of a Hailo format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HailoFormatOrder {
    NHWC,
    NCHW,
    HailoNmsOnChip,
    HailoNmsHost,
    Other(u32), // Catch-all for unknown orders
}

/// Struct representing a Hailo format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HailoFormat {
    pub format_type: HailoFormatType,
    pub format_order: HailoFormatOrder,
    pub flags: u32, // Additional flags associated with the format
}

impl HailoFormat {
    /// Creates a new `HailoFormat` with the specified type, order, and flags.
    pub fn new(format_type: HailoFormatType, format_order: HailoFormatOrder, flags: u32) -> Self {
        HailoFormat {
            format_type,
            format_order,
            flags,
        }
    }

    /// Converts a raw u32 into a `HailoFormatType`.
    pub fn from_raw_type(raw_type: u32) -> HailoFormatType {
        match raw_type {
            0 => HailoFormatType::Float32,
            1 => HailoFormatType::Uint8,
            2 => HailoFormatType::Int8,
            3 => HailoFormatType::Float16,
            other => HailoFormatType::Other(other),
        }
    }

    /// Converts a raw u32 into a `HailoFormatOrder`.
    pub fn from_raw_order(raw_order: u32) -> HailoFormatOrder {
        match raw_order {
            0 => HailoFormatOrder::NHWC,
            1 => HailoFormatOrder::NCHW,
            2 => HailoFormatOrder::HailoNmsOnChip,
            3 => HailoFormatOrder::HailoNmsHost,
            other => HailoFormatOrder::Other(other),
        }
    }

    /// Retrieves a description of the Hailo format.
    pub fn description(&self) -> String {
        format!(
            "Type: {:?}, Order: {:?}, Flags: {:#X}",
            self.format_type, self.format_order, self.flags
        )
    }
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hailo_format_creation() {
        let format = HailoFormat::new(HailoFormatType::Float32, HailoFormatOrder::NHWC, 0x0);
        assert_eq!(format.format_type, HailoFormatType::Float32);
        assert_eq!(format.format_order, HailoFormatOrder::NHWC);
        assert_eq!(format.flags, 0x0);
    }

    #[test]
    fn test_hailo_format_from_raw() {
        let format_type = HailoFormat::from_raw_type(1);
        assert_eq!(format_type, HailoFormatType::Uint8);

        let format_order = HailoFormat::from_raw_order(2);
        assert_eq!(format_order, HailoFormatOrder::HailoNmsOnChip);
    }
}
