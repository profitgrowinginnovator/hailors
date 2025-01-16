#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)] 
#[allow(dead_code)]
pub enum HailoStatus {
    Success = 0,  // No error
    Uninitialized = 1,  // No error code initialized
    InvalidArgument = 2,  // Invalid argument passed
    OutOfHostMemory = 3,  // Cannot allocate more memory on host
    Timeout = 4,  // Timeout occurred
    InsufficientBuffer = 5,  // Buffer is insufficient
    InvalidOperation = 6,  // Invalid operation
    NotImplemented = 7,  // Functionality not implemented
    InternalFailure = 8,  // Unexpected internal failure
    DataAlignmentFailure = 9,  // Data alignment issue
    ChunkTooLarge = 10,  // Chunk size exceeds limit
    CloseFailure = 12,  // Failed to close resource
    OpenFileFailure = 13,  // Failed to open file
    FileOperationFailure = 14,  // File operation failed
    UnsupportedControlProtocolVersion = 15,  // Unsupported protocol version
    UnsupportedFirmwareVersion = 16,  // Unsupported firmware version
    InvalidControlResponse = 17,  // Invalid control response
    FirmwareControlFailure = 18,  // Firmware control failed
    NotFound = 61,  // Element not found
    CommunicationClosed = 62,  // Communication closed
    StreamAbort = 63,  // Stream aborted
    DriverNotInstalled = 64,  // Driver not installed or running
    NotAvailable = 65,  // Component not available
    Unsupported = 79,  // Operation not supported
}
#[allow(dead_code)]
impl HailoStatus {
    pub fn from_i32(value: i32) -> Self {
        match value {
            0 => HailoStatus::Success,
            1 => HailoStatus::Uninitialized,
            2 => HailoStatus::InvalidArgument,
            3 => HailoStatus::OutOfHostMemory,
            4 => HailoStatus::Timeout,
            5 => HailoStatus::InsufficientBuffer,
            6 => HailoStatus::InvalidOperation,
            7 => HailoStatus::NotImplemented,
            8 => HailoStatus::InternalFailure,
            9 => HailoStatus::DataAlignmentFailure,
            10 => HailoStatus::ChunkTooLarge,
            12 => HailoStatus::CloseFailure,
            13 => HailoStatus::OpenFileFailure,
            14 => HailoStatus::FileOperationFailure,
            15 => HailoStatus::UnsupportedControlProtocolVersion,
            16 => HailoStatus::UnsupportedFirmwareVersion,
            17 => HailoStatus::InvalidControlResponse,
            18 => HailoStatus::FirmwareControlFailure,
            61 => HailoStatus::NotFound,
            62 => HailoStatus::CommunicationClosed,
            63 => HailoStatus::StreamAbort,
            64 => HailoStatus::DriverNotInstalled,
            65 => HailoStatus::NotAvailable,
            79 => HailoStatus::Unsupported,
            _ => HailoStatus::InternalFailure,  // Default case for unknown statuses
        }
    }
}

impl std::fmt::Display for HailoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HailoStatus::Success => write!(f, "Success"),
            HailoStatus::Uninitialized => write!(f, "Uninitialized"),
            HailoStatus::InvalidArgument => write!(f, "Invalid Argument"),
            HailoStatus::OutOfHostMemory => write!(f, "Out of Host Memory"),
            HailoStatus::Timeout => write!(f, "Timeout"),
            HailoStatus::InsufficientBuffer => write!(f, "Insufficient Buffer"),
            HailoStatus::InvalidOperation => write!(f, "Invalid Operation"),
            HailoStatus::NotImplemented => write!(f, "Not Implemented"),
            HailoStatus::InternalFailure => write!(f, "Internal Failure"),
            HailoStatus::DataAlignmentFailure => write!(f, "Data Alignment Failure"),
            HailoStatus::ChunkTooLarge => write!(f, "Chunk Too Large"),
            HailoStatus::CloseFailure => write!(f, "Close Failure"),
            HailoStatus::OpenFileFailure => write!(f, "Open File Failure"),
            HailoStatus::FileOperationFailure => write!(f, "File Operation Failure"),
            HailoStatus::UnsupportedControlProtocolVersion => write!(f, "Unsupported Control Protocol Version"),
            HailoStatus::UnsupportedFirmwareVersion => write!(f, "Unsupported Firmware Version"),
            HailoStatus::InvalidControlResponse => write!(f, "Invalid Control Response"),
            HailoStatus::FirmwareControlFailure => write!(f, "Firmware Control Failure"),
            HailoStatus::NotFound => write!(f, "Not Found"),
            HailoStatus::CommunicationClosed => write!(f, "Communication Closed"),
            HailoStatus::StreamAbort => write!(f, "Stream Abort"),
            HailoStatus::DriverNotInstalled => write!(f, "Driver Not Installed"),
            HailoStatus::NotAvailable => write!(f, "Not Available"),
            HailoStatus::Unsupported => write!(f, "Unsupported"),
        }
    }
}

