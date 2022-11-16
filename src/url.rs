use std::collections::HashMap;

pub struct URLBuilder {
    protocol: String,
    host: String,
    port: u16,
    params: HashMap<String, String>,
}

impl URLBuilder {
    pub fn new() -> URLBuilder {
        URLBuilder {
            protocol: String::new(),
            host: String::new(),
            port: 0,
            params: HashMap::new(),
        }
    }

    pub fn build(&self) -> String {
        let base = format!("{}://{}", self.protocol, self.host);

        let mut url_params = String::new();

        if !self.params.is_empty() {
            url_params.push('?');

            for (param, value) in self.params.iter() {
                url_params.push_str(format!("{}={}&", param, value).as_str());
            }
        }

        match self.port {
            0 => format!("{}{}", base, url_params),
            _ => format!("{}:{}{}", base, self.port, url_params),
        }
    }

    pub fn add_param(&mut self, param: &str, value: &str) -> &mut Self {
        self.params.insert(param.to_string(), value.to_string());

        self
    }

    pub fn set_protocol(&mut self, protocol: &str) -> &mut Self {
        self.protocol = protocol.to_string();

        self
    }

    pub fn set_host(&mut self, host: &str) -> &mut Self {
        self.host = host.to_string();

        self
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;

        self
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn protocol(&self) -> &str {
        &self.protocol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_host() {
        let mut ub = URLBuilder::new();
        ub.set_host("localhost");
        assert_eq!("localhost", ub.host());
    }

    #[test]
    fn test_set_protocol() {
        let mut ub = URLBuilder::new();
        ub.set_protocol("https");
        assert_eq!("https", ub.protocol());
    }

    #[test]
    fn test_set_port() {
        let mut ub = URLBuilder::new();
        ub.set_port(8000);
        assert_eq!(8000, ub.port());
    }

    #[test]
    fn create_google_url() {
        let mut ub = URLBuilder::new();
        ub.set_protocol("http")
            .set_host("www.google.com")
            .set_port(80);
        let url = ub.build();
        assert_eq!("http://www.google.com:80", url);
    }

    #[test]
    fn create_url_without_port() {
        let mut ub = URLBuilder::new();
        ub.set_protocol("http").set_host("google.com");
        let url = ub.build();
        assert_eq!("http://google.com", url)
    }

    #[test]
    fn create_url_without_port_and_params() {
        let mut ub = URLBuilder::new();
        ub.set_protocol("http")
            .set_host("google.com")
            .add_param("gcookie", "0xcafe");
        let url = ub.build();
        assert_eq!("http://google.com?gcookie=0xcafe&", url)
    }

    #[test]
    fn create_url_with_params() {
        let mut ub = URLBuilder::new();
        ub.set_protocol("http")
            .set_host("localhost")
            .set_port(8000)
            .add_param("first", "1")
            .add_param("second", "2")
            .add_param("third", "3");

        let url = ub.build();
        assert!(url.contains("first=1"));
        assert!(url.contains("second=2"));
        assert!(url.contains("third=3"));
    }
}
