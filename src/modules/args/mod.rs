fn get_nth_element(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}
#[derive(Debug)]
pub struct Args {
    pub usid: String,
    pub url: String,
}

impl Args {
    pub fn get()->Self{
        Args{
            usid: get_nth_element(1),
            url: get_nth_element(2),
        }
    }
}