use reqwest::blocking::{get, Response};
pub use reqwest::Result;

const PATH: &str = "https://api.papermc.io/v2";

#[derive(Clone, Debug)]
struct Path(String);

impl Path {
    fn new(location: &str) -> Self {
        Self(location.to_string())
    }

    fn push(&mut self, path: &str) {
        let mut temp = self.0.to_owned();
        temp.push('/');
        temp.push_str(path);
        self.0 = temp
    }
}

pub struct Api(Path);

impl Api {
    pub fn new() -> Self {
        Self(Path::new(PATH))
    }

    pub fn get(&mut self, location: &str, path: Option<&str>) -> Result<Response> {
        let mut new = self.0.clone();

        if let Some(path) = path {
            new.push(path)
        }
        new.push(location);

        let res = get(new.0.as_str())?;
        self.0 = new;
        Ok(res)
    }

    pub fn get_once(location: &str, path: Option<&str>) -> Result<Response> {
        let mut new = Path::new(PATH);
        if let Some(path) = path {
            new.push(path)
        }
        new.push(location);
        get(new.0.as_str())
    }
}
