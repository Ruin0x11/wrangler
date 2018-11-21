pub mod result {
    use ::std::fmt;
    use ::std::error;
    pub type Result<T> = ::std::result::Result<T, Box<error::Error>>;

    #[derive(Debug)]
    pub struct Error;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "error")
        }
    }

    impl error::Error for Error {
        fn cause(&self) -> Option<&error::Error> { None }
    }
}

pub mod toml {
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    use toml::Value;
    use super::result;

    pub fn toml_string_from_file(filename: &str) -> result::Result<String> {
        let path = PathBuf::from(filename);
        let mut file = File::open(&path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(data)
    }

    pub fn toml_value_from_string(data: &str) -> result::Result<Value> {
        let it = data.parse::<Value>()?;
        Ok(it)
    }

    pub fn toml_value_from_file(filename: &str) -> result::Result<Value> {
        let toml_str = toml_string_from_file(filename)?;
        toml_value_from_string(&toml_str)
    }
}
