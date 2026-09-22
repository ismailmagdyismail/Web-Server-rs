#[derive(PartialEq, Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl HttpMethod {
    pub fn parse<'a>(method: &'a str) -> Result<HttpMethod, String> {
        match (method).to_lowercase().as_str() {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            "delete" => Ok(HttpMethod::Delete),
            "put" => Ok(HttpMethod::Put),
            _ => Err("Invalid Http Method".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::http_method::HttpMethod;

    #[test]
    fn test_get() {
        let method = String::from("GET");
        let method = HttpMethod::parse(&method).unwrap();

        assert_eq!(method, HttpMethod::Get);
    }

    #[test]
    fn test_put() {
        let method = String::from("PUT");
        let method = HttpMethod::parse(&method).unwrap();

        assert_eq!(method, HttpMethod::Put);
    }

    #[test]
    fn test_case_insensitve() {
        let method = String::from("geT");
        let method = HttpMethod::parse(&method).unwrap();

        assert_eq!(method, HttpMethod::Get);
    }
}
