

#[cfg(test)]
mod tests {
    
    use std::fs;
    use super::hef::hef::HefData; // Import the HefData type

    // Helper function to load a test HEF file
    fn load_test_hef(file_name: &str) -> Vec<u8> {
        let path = format!("test_data/{}", file_name);
        fs::read(path).expect("Failed to read test HEF file")
    }

    #[test]
    fn test_parse_pose_hef() {
        let hef_data = load_test_hef("./hef/yolov8s_pose_h8.hef");
        match HefData::parse(hef_data) {
            Ok(parsed_data) => {
                println!("Successfully parsed Pose HEF file!");
                parsed_data.print();
            }
            Err(e) => panic!("Failed to parse Pose HEF file: {}", e),
        }
    }

    #[test]
    fn test_parse_detection_hef() {
        let hef_data = load_test_hef("./hef/yolov8s_h8.hef");
        match HefData::parse(hef_data) {
            Ok(parsed_data) => {
                println!("Successfully parsed Detection HEF file!");
                parsed_data.print();
            }
            Err(e) => panic!("Failed to parse Detection HEF file: {}", e),
        }
    }
}
