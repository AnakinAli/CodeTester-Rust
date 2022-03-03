mod modules;

use modules::{Args, Sample, Project, Output, Result};

extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use]
extern crate serde_derive;

use std::process::{Command};


fn main() {
    let args = Args::get();

    let url = format!("{url}/api/project/{usid}", url = args.url, usid = args.usid);

    let samples = Sample::locate_samples("./Project/Samples".to_string());

    let project = Project::locate_code_path("./Project/Code".to_string());

    let output = Output::output(&project.path, &samples.input_path, &samples.output_path);

    let result = Result::result(args.usid, &output);

    let count_errors = 6;

    make_request(url, result, count_errors);
}

fn make_request(url: String, result: Result, mut count_errors: u8) -> u8 {
    let payload = serde_json::to_string(&result).unwrap();

    let command = format!("curl --location --request POST {url} --header 'Content-Type: application/json' --data-raw '{payload}'",
                          url = url, payload = payload);

    let output =
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");

    return if output.status.success() {
        0
    } else {

        if count_errors == 0 {
            return 0;
        }
        count_errors = count_errors - 1;
        make_request(url, result, count_errors)
    }
}