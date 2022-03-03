mod modules;

use modules::{Args, Sample, Project, Output, Result};


fn main() {
    let args = Args::get();

    let samples = Sample::locate_samples("./Project/Samples".to_string());

    let project = Project::locate_code_path("./Project/Code".to_string());

    let output = Output::output(&project.path, &samples.input_path, &samples.output_path);

    let result = Result::result(args.usid, &output);

    println!("{:?} ", result);
}
