#[cfg(test)]
pub mod helper {
    use reqwest::blocking::Response;
    use std::{env, fs, path::PathBuf};

    const SESSION_TOKEN_PATH: &str = ".advent";

    fn get_token() -> Result<String, Box<dyn std::error::Error>> {
        let p = PathBuf::new()
            .join(env::var("HOME")?)
            .join(SESSION_TOKEN_PATH);

        fs::read_to_string(p)
            .map(|s| s.trim().to_string())
            .map_err(|e| e.into())
    }

    fn get_daily_args(day: isize) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::ClientBuilder::new().build()?;
        let session_header = format!("session={}", get_token()?);
        let s = &session_header[..];

        client
            .get(format!("https://adventofcode.com/2022/day/{}/input", day))
            .header("cookie", s)
            .send()
            .map_err(|e| e.into())
    }

    pub fn run_day<T: std::fmt::Display>(
        day: isize,
        f: fn(Response) -> Result<T, Box<dyn std::error::Error>>,
    ) {
        let response = get_daily_args(day).unwrap();
        let value = f(response);
        println!("{}", value.unwrap());
    }
}
