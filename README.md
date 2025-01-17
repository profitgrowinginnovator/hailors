
# Hailo RS: Minimal Rust Binding for LibHailoRT

Hailo RS is a minimal Rust binding for **LibHailoRT** designed to simplify the deployment of AI models on the **Hailo AI Hat+** directly from a **Raspberry Pi**. This crate abstracts the complexity of Hailo?s powerful inference engine, making it easy to integrate Rust-based AI applications.

---

## Features
- Minimal and efficient Rust interface for **LibHailoRT**.
- Supports creating devices, configuring HEFs, and running inferences.
- Provides input/output virtual streams for efficient data transfer.
- Optimized for resource-constrained devices like the **Raspberry Pi**.
- Can be used as a crate or CLI for easy integration.

---

## Requirements

### Hardware:
- Raspberry Pi (e.g. 5)
- Hailo AI Hat+

### Software:
- **Rust** (1.68 or higher)
- **LibHailoRT** installed (latest version)
- **GCC** (for compiling C/C++ bindings)
- **CMake** (for building dependencies)

For setting up the Hailo AI Hat+ on a Raspberry Pi, follow [this guide](https://www.raspberrypi.com/documentation/computers/ai.html).

---

## Installation

### Using as a Crate
To include `hailors` in your Rust project, add this to your `Cargo.toml`:

```toml
[dependencies]
hailors = "0.1.0"
```

Then import it in your Rust code:

```rust
use hailors::HailoDevice;
```

### Using as a CLI
To install the CLI globally, run:

```bash
cargo install hailors
```

After installation, use the `hailors-cli` command:

```bash
hailors-cli --hef ./hef/yolov8s_h8.hef --input ./images/dog.rgb --network yolo-detection --threshold 0.85
```

### From Source

1. Clone the Repository:

```bash
git clone https://github.com/username/hailors.git
cd hailors
```

2. Install Dependencies:

```bash
sudo dpkg -i libhailort.deb
sudo apt-get -f install  # Resolve dependencies
```

3. Build the Project:

```bash
cargo build --release
```

---

## Usage

### CLI Example
To detect a dog using the example image and HEF provided:

```bash
hailors-cli --hef ./hef/yolov8s_h8.hef --input ./images/dog.rgb --network yolo-detection --threshold 0.85
```

### Example Rust Program
```rust
use hailors::{HailoDevice, network::YoloDetection};

fn main() {
    // Create a HailoDevice
    let hef_path = "./hef/yolov8s_h8.hef";
    let device = HailoDevice::new(hef_path).expect("Failed to create HailoDevice");

    // Define a YOLO network type
    let yolo_network = YoloDetection {
        num_classes: 80,
        max_bboxes_per_class: 100,
        threshold: 0.85,
    };

    // Read the input RGB file
    let input_file_path = "./images/dog.rgb";
    let input_data = std::fs::read(input_file_path).expect("Failed to read input file");

    // Verify that the input file size matches the expected input frame size
    assert_eq!(input_data.len(), device.input_frame_size, "Input file size does not match expected frame size");

    // Perform inference
    device.write_input(&input_data).expect("Failed to write input frame");
    let detections = device.read_output(&yolo_network).expect("Failed to read and parse output");

    // Check for the presence of the "dog" class (class ID 16 in COCO)
    let dog_detected = detections.iter().any(|d| d.class_id == 16 && d.confidence >= 0.85);

    if dog_detected {
        println!("Dog detected in the image!");
    } else {
        println!("No dog detected in the image.");
    }
}
```

---

## Running the Program
Once your program is ready, you can run it:

```bash
cargo run --release
```

---

## Examples

### Running an Inference Test
Run all tests in the crate to verify functionality:

```bash
cargo test
```

### Starting from a Fresh Device
If you?re setting up a new Raspberry Pi for development, use the provided `install.sh` script:

```bash
./install.sh
```

This script installs all required dependencies and sets up the environment.

---

## Troubleshooting

### Undefined References
Ensure that the `build.rs` correctly links to the `hailort` library and the paths are properly set:

```bash
cargo clean
cargo build --release --verbose
```

### Missing Dependencies
If you encounter missing dependencies, ensure the following packages are installed:

```bash
sudo apt-get install build-essential cmake pkg-config
```

### Segmentation Fault
Run the program with `gdb` to debug:

```bash
gdb target/debug/main
```

### Device Detection Issues
Ensure the Hailo AI Hat+ is correctly connected and `hailort` services are installed.

---

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

---

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## Acknowledgments
- [Hailo AI](https://hailo.ai) for the powerful AI acceleration platform.
- Rust community for the amazing ecosystem.
