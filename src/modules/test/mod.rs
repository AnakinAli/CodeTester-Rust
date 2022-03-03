use std::process::Command;
use std::fs;
use crate::modules::payload::Lines;


fn contents_of_sample_output_file(output_path: &str) -> String {
    fs::read_to_string(output_path.to_string())
        .expect("error")
}

fn search_data_line_by_line(expected_output: &String, real_output: &String) -> Lines {
    let expected: Vec<&str> = expected_output.split('\n').collect();
    let real: Vec<&str> = real_output.split('\n').collect();

    if real.len() != expected.len() {
        return Lines {
            n: 0,
            expected: expected_output.to_string(),
            result: real_output.to_string(),
        };
    }

    let count = expected.len();

    for i in 0..count {
        if expected[i] != real[i] {
            return Lines {
                n: i as u8 + 1,
                expected: expected[i].to_string(),
                result: real[i].to_string(),
            };
        }
    }

    // fallback one
    return Lines {
        n: 0,
        expected: String::new(),
        result: String::new(),
    };
}

fn get_directory_file_path(file_path: &str) -> String {
    let paths = file_path.split('/');
    return file_path.replace(paths.last().unwrap(), "").to_string();
}

fn handle_pips(path: &str) {
    let command_pips = format!("\
    cd {} && \
    pip freeze > requirements.txt &&\
    pip install -r requirements.txt", get_directory_file_path(path));

    Command::new("sh")
        .arg("-c")
        .arg(command_pips)
        .output()
        .expect("failed to execute process");
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

        handle_pips(path);

        let output =
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process");

        // Outputs
        let expected_output = contents_of_sample_output_file(output_path);
        let real_output = String::from_utf8_lossy(&output.stdout).to_string();

        let has_errors = expected_output != real_output;

        let lines = search_data_line_by_line(&expected_output, &real_output);


        return Output {
            has_errors,
            line: lines.n,
            expected_output: lines.expected,
            real_output: lines.result,
        };
    }
}