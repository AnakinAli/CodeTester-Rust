use std::fs;

/// Samples input extension
const SAMPLE_INPUT_EXTENSION: &str = ".in";
/// Samples output extensions
const SAMPLE_OUTPUT_EXTENSION: &str = ".out";

#[derive(Debug)]
pub struct Sample {
    pub input_path: String,
    pub output_path: String,
}

impl Sample{
    ///
    /// Locates the sample input and output
    /// given by the admin
    /// Returns sample
    ///
    pub fn locate_samples(samples_folder_path: String) -> Sample {
        let files = fs::read_dir(samples_folder_path).unwrap();

        let mut input_path: String = "".to_string();
        let mut output_path: String = "".to_string();

        for file in files {
            let path = file.unwrap().path().display().to_string();

            if path.ends_with(&SAMPLE_INPUT_EXTENSION) {
                input_path = path;
            } else if path.ends_with(&SAMPLE_OUTPUT_EXTENSION) {
                output_path = path;
            }
        }

        return Sample {
            input_path,
            output_path,
        };
    }
}
