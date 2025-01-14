

# Hailo RS: Minimal Rust Binding for LibHailoRT

**IMPORTANT:** This project is still a work in progress.

Hailo RS is a minimal Rust binding for **LibHailoRT** designed to simplify the deployment of AI models on the **Hailo AI Hat+** directly from a **Raspberry Pi**. This crate abstracts the complexity of Hailo?s powerful inference engine, making it easy to integrate Rust-based AI applications.

---

## Features
- Minimal and efficient Rust interface for **LibHailoRT**.
- Supports creating devices, configuring HEFs, and running inferences.
- Provides input/output virtual streams for efficient data transfer.
- Optimized for resource-constrained devices like the **Raspberry Pi**.

---

## Requirements
### Hardware:
- Raspberry Pi (e.g., 4B/3B+)
- Hailo AI Hat+

### Software:
- **Rust** (1.68 or higher)
- **LibHailoRT** installed (latest version)
- **GCC** (for compiling C/C++ bindings)
- **CMake** (for building dependencies)

---

## Installation

### 1. Clone the Repository
\`\`\`bash
git clone https://github.com/username/hailors.git
cd hailors
\`\`\`

### 2. Install Dependencies
Make sure `LibHailoRT` is installed:
\`\`\`bash
sudo dpkg -i libhailort.deb
sudo apt-get -f install  # To resolve dependencies
\`\`\`

### 3. Build the Project
\`\`\`bash
cargo build --release
\`\`\`

---

## Usage

### Example Rust Program
\`\`\`rust
use hailors::HailoDevice;

fn main() {
    // Create a HailoDevice
    let mut device = HailoDevice::new().expect("Failed to create HailoDevice");

    // Load HEF and configure virtual streams
    let hef_path = "models/shortcut_net.hef";
    let mut input_vstreams: [*mut hailors::InputVStream; 16] = [std::ptr::null_mut(); 16];
    let mut output_vstreams: [*mut hailors::OutputVStream; 16] = [std::ptr::null_mut(); 16];

    let network_group = device
        .configure_hef(hef_path, &mut input_vstreams, &mut output_vstreams)
        .expect("Failed to configure HEF");

    // Run inference
    let infer_result = device
        .infer(network_group, &mut input_vstreams, &mut output_vstreams)
        .expect("Inference failed");

    // Release the device after use
    device.release().expect("Failed to release HailoDevice");

    println!("Inference completed successfully!");
}
\`\`\`

---

## API Overview

### **HailoDevice API**
- \`HailoDevice::new() -> Result<Self, HailoStatus>\`: Creates a new device.
- \`configure_hef(&self, hef_path: &str, input_vstreams: &mut [...], output_vstreams: &mut [...]) -> Result<*mut ConfiguredNetworkGroup, HailoStatus>\`: Configures a network group using a HEF file.
- \`infer(&self, network_group: *mut ConfiguredNetworkGroup, input_vstreams: &mut [...], output_vstreams: &mut [...]) -> Result<(), HailoStatus>\`: Performs inference.
- \`release(&mut self) -> Result<(), HailoStatus>\`: Releases the device.

---

## Running the Program
Once your program is ready, you can run it:
\`\`\`bash
cargo run --release
\`\`\`

---

## Examples
### Running an Inference Test
\`\`\`bash
cargo test --features "test-linking"
\`\`\`

---

## Troubleshooting

### Undefined References
Ensure that the \`build.rs\` correctly links to the \`hailort\` library and the paths are properly set:
\`\`\`bash
cargo clean
cargo build --release --verbose
\`\`\`

### Missing Dependencies
If you encounter missing dependencies, ensure the following packages are installed:
\`\`\`bash
sudo apt-get install build-essential cmake pkg-config
\`\`\`

### Segmentation Fault
Run the program with \`gdb\` to debug:
\`\`\`bash
gdb target/debug/main
\`\`\`

### Device Detection Issues
Ensure the Hailo AI Hat+ is correctly connected and \`hailort\` services are installed.

---

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

---

## License
This project is licensed under the MIT License. See the \`LICENSE\` file for details.

---

## Acknowledgments
- [Hailo AI](https://hailo.ai) for the powerful AI acceleration platform.
- Rust community for the amazing ecosystem.
