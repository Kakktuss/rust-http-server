use std::fmt::Error;
use std::fs::File;
use std::io::Read;
use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config
{
    pub host: String,
    pub port: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigBuilder
{
    pub host: Option<String>,
    pub port: Option<String>,
}

impl ConfigBuilder
{
    pub fn from_config(file: &str) -> Result<Self, Error>
    {
        let mut file = match File::open(&file) {
            Err(e) => return Err(Error::from(e)),
            Ok(file ) => file
        };

        let mut content = String::new();

        file.read_to_string(&mut content);

        let result: Self = match serde_json::from_str(&content) {
            Err(e) => return Err(Error::from(e)),
            Ok(result) => result
        };

        Ok(result)
    }

    pub fn set_host(&mut self, host: String) -> &mut Self
    {
        self.host = Some(host.to_string());
        self
    }

    pub fn set_port(&mut self, port: String) -> &mut Self
    {
        self.port = Some(port.to_string());
        self
    }

    pub fn build(self) -> Config
    {
        Config {
            host: self.host.unwrap_or_else(|| "127.0.0.1".to_string()),
            port: self.port.unwrap_or_else(|| "")
        }
    }
}

impl Default for ConfigBuilder
{
    fn default() -> Self {
        ConfigBuilder
        {
            host: None,
            port: None
        }
    }
}