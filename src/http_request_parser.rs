use std::collections::HashMap;

use crate::http_request_line::HttpRequestLine;

#[derive(Debug)]
pub struct HttpRequest<'a> {
    pub request_line: HttpRequestLine<'a>,
    pub headers: HashMap<&'a str, &'a str>,
}

impl<'a> HttpRequest<'a> {
    pub fn parse(lines: &'a Vec<String>) -> Result<HttpRequest<'a>, String> {
        if lines.len() < 1 {
            return Err("Empty Http Request recieved, encountered 0 request lines".into());
        }
        let http_request_line = HttpRequestLine::parse(&lines[0])?;
        let mut headers: HashMap<&'a str, &'a str> = HashMap::new();
        lines.iter().enumerate().for_each(|(i, line)| {
            if i == 0 {
                return;
            }
            let key_value: Vec<&str> = line.split(':').collect();
            if key_value.len() != 2 {
                return;
            }
            headers.insert(key_value[0], key_value[1]);
            return;
        });
        Ok(HttpRequest {
            request_line: http_request_line,
            headers,
        })
    }
}