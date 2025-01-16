use hailors::HailoDevice;
fn main() {
    // Step 1: Create a new HailoDevice
    match HailoDevice::new() {
        Ok(mut device) => {
            println!("Device created successfully!");

            // Step 2: Release the device
            if let Err(e) = device.release() {
                println!("Failed to release device: {}", e);
            } else {
                println!("Device released successfully!");
            }
        }
        Err(e) => {
            println!("Failed to create device: {}", e);
        }
    }
}