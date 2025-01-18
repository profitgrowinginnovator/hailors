#[derive(Debug)]
pub enum ContextSwitchActionType {
    None,
    ActivateConfigChannel,
    DeactivateConfigChannel,
    WriteDataCcw,
    // Add the rest of the action types here...
}

pub trait ContextSwitchConfigAction {
    fn get_type(&self) -> ContextSwitchActionType;
    fn supports_repeated_block(&self) -> bool;
    fn serialize(&self) -> Result<Vec<u8>, String>; // Simplified for Rust
}

// Example of a specific action
#[derive(Debug)]
pub struct NoneAction;

impl ContextSwitchConfigAction for NoneAction {
    fn get_type(&self) -> ContextSwitchActionType {
        ContextSwitchActionType::None
    }

    fn supports_repeated_block(&self) -> bool {
        false
    }

    fn serialize(&self) -> Result<Vec<u8>, String> {
        Ok(vec![]) // No data for NoneAction
    }
}

// Other actions follow the same structure...
