use crate::http_method::HttpMethod;

#[derive(Debug)]
pub struct HttpRequestLine<'a> {
    pub method: HttpMethod,
    pub uri: &'a str,
    pub version: &'a str,
}

impl<'a> HttpRequestLine<'a> {
    pub fn parse(request_line: &'a str) -> Result<HttpRequestLine<'a>, String> {
        let splitted: Vec<&str> = request_line.split(' ').collect();
        if splitted.len() < 3 {
            return Result::Err("Invalid request line encountered".into());
        }

        let method = HttpMethod::parse(&splitted[0])?;
        let uri = &splitted[1];
        let version = &splitted[2];

        Ok(HttpRequestLine {
            method,
            uri,
            version,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{http_method::HttpMethod, http_request_line::HttpRequestLine};

    #[test]
    fn test_req_line_parsing() {
        let request_line = String::from("GET / HTTP/1.1");
        let parsed_req_line = HttpRequestLine::parse(&request_line).unwrap();

        assert_eq!(parsed_req_line.uri, "/");
        assert_eq!(parsed_req_line.version, "HTTP/1.1");
        assert_eq!(parsed_req_line.method, HttpMethod::Get);
    }
}
