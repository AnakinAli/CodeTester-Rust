use crate::modules::Output;

#[derive(Debug)]
pub struct Result {
    pub usid: String,
    pub different: bool,
    pub lines: Lines,
}

#[derive(Debug)]
pub struct Lines {
    pub n: u8,
    pub expected: String,
    pub result: String,
}

fn set_lines(output: &Output) -> Lines {
    Lines {
        n: output.line,
        expected: output.expected_output.to_string(),
        result: output.real_output.to_string(),
    }
}

impl Result {
    pub fn result(usid: String, output: &Output) -> Result {
        let lines = set_lines(output);

        let different = output.has_errors;

        Result {
            usid,
            different,
            lines,
        }
    }
}