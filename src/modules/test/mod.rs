use std::process::Command;
use std::fs;

fn contents_of_sample_output_file(output_path: &str) -> String {
    fs::read_to_string(output_path.to_string())
        .expect("error")
}

#[derive(Debug)]
pub struct Output {
    pub has_errors: bool,
    pub line: u8,
    pub expected_output: String,
    pub real_output: String,
}

impl Output {
    pub fn output(path: &str, input_path: &str, output_path: &str) -> Output {
        let command = format!("python3 {} {}", path, input_path);

        let output =
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process");

        // Outputs
        let mut expected_output = contents_of_sample_output_file(output_path);
        let mut real_output = String::from_utf8_lossy(&output.stdout).to_string();

        let has_errors = expected_output != real_output;

        let line: u8 = 1;


        if !has_errors {
            expected_output = String::new();
            real_output = String::new();
        }

        return Output {
            has_errors,
            line,
            expected_output,
            real_output,
        };
    }
}