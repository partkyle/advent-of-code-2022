#[cfg(test)]
pub mod helper {
    use reqwest::blocking::Response;
    use std::error::Error;
    use std::io::prelude::*;
    use std::{
        env,
        fs::{self, File},
        path::PathBuf,
    };

    const SESSION_TOKEN_PATH: &str = ".advent";
    const LOCAL_CACHE_DIR: &str = ".advent";

    fn get_token() -> Result<String, Box<dyn Error>> {
        let p = PathBuf::new()
            .join(env::var("HOME")?)
            .join(SESSION_TOKEN_PATH);

        fs::read_to_string(p)
            .map(|s| s.trim().to_string())
            .map_err(|e| e.into())
    }

    fn cache_path(day: usize) -> PathBuf {
        PathBuf::new()
            .join(LOCAL_CACHE_DIR)
            .join(format!("{}.txt", day))
    }

    fn cache_path_debug(day: usize) -> PathBuf {
        PathBuf::new()
            .join(LOCAL_CACHE_DIR)
            .join(format!("{}.debug.txt", day))
    }

    fn is_daily_cached(day: usize) -> bool {
        cache_path(day).exists()
    }

    fn cache_daily_args(day: usize, r: Response) -> Result<String, Box<dyn Error>> {
        let loc = cache_path(day);
        let text = r.text()?;
        fs::create_dir_all(LOCAL_CACHE_DIR)?;
        let mut file = File::create(loc)?;
        file.write_all(text.as_bytes())?;

        Ok(text)
    }

    fn read_cache_daily_args(day: usize) -> Result<String, Box<dyn Error>> {
        fs::read_to_string(cache_path(day)).map_err(|e| e.into())
    }

    fn read_cache_daily_args_debug(day: usize) -> Result<String, Box<dyn Error>> {
        let p = cache_path_debug(day);
        if !p.exists() {
            fs::copy(cache_path(day), &p)?;
        }
        fs::read_to_string(p).map_err(|e| e.into())
    }

    fn query_daily_args(day: usize) -> Result<Response, Box<dyn Error>> {
        let client = reqwest::blocking::ClientBuilder::new().build()?;
        let session_header = format!("session={}", get_token()?);
        let s = &session_header[..];

        client
            .get(format!("https://adventofcode.com/2022/day/{}/input", day))
            .header("cookie", s)
            .send()
            .map_err(|e| e.into())
    }

    fn get_daily_args(day: usize) -> Result<String, Box<dyn Error>> {
        if is_daily_cached(day) {
            return read_cache_daily_args(day);
        }

        let response = query_daily_args(day)?;

        cache_daily_args(day, response)
    }

    pub fn run_day<T: std::fmt::Display>(day: usize, f: fn(String) -> Result<T, Box<dyn Error>>) {
        let response = get_daily_args(day).unwrap();
        let value = f(response);
        println!("{}", value.unwrap());
    }

    pub fn dbg_day<T: std::fmt::Display>(day: usize, f: fn(String) -> Result<T, Box<dyn Error>>) {
        let response = read_cache_daily_args_debug(day).unwrap();
        let value = f(response);
        println!("{}", value.unwrap());
    }
}
