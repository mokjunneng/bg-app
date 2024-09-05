use std::str::FromStr;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}
impl FromStr for HttpMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    method: Option<HttpMethod>,
    path: Option<String>,
    headers: Option<HttpHeaders>,
    body: Option<String>,
}
impl HttpRequest {
    pub fn empty() -> Self {
        Self {
            method: None,
            path: None,
            headers: None,
            body: None,
        }
    }

    pub fn set_header(&mut self, header: &str, val: &str) {
        match &mut self.headers {
            None => {
                self.headers = Some(HttpHeaders::empty());
                self.headers.as_mut().unwrap().set_header(header, val)
            }
            Some(headers) => headers.set_header(header, val),
        }
    }
}

#[derive(Debug)]
struct HttpHeaders {
    host: Option<String>,
    user_agent: Option<String>,
    accept: Option<String>,
    content_type: Option<String>,
}
impl HttpHeaders {
    pub fn empty() -> Self {
        Self {
            host: None,
            user_agent: None,
            accept: None,
            content_type: None,
        }
    }

    pub fn set_header(&mut self, header: &str, val: &str) {
        let val = Some(val.to_string());
        match header {
            "Host" => self.host = val,
            "User-Agent" => self.user_agent = val,
            "Accept" => self.accept = val,
            "Content-Type" => self.content_type = val,
            _ => return,
        }
    }
}

pub fn parse_request(buffer: &[u8]) -> HttpRequest {
    let buffer_str = String::from_utf8(buffer.to_vec()).unwrap();
    println!("Buffer string: {}", buffer_str);

    let mut request = HttpRequest::empty();

    // Parse first line to determine the HTTP method
    let mut lines = buffer_str.lines();
    let mut first_line = lines.next().unwrap().split(' ');
    let method = first_line.next().unwrap();
    println!("HTTP Method: {}", method);
    let path = first_line.next().unwrap();
    println!("HTTP Path: {}", path);

    request.method = Some(HttpMethod::from_str(method).unwrap());
    request.path = Some(path.to_string());

    for line in lines.by_ref().into_iter() {
        if line.is_empty() {
            break;
        }

        // Set Headers
        let mut line_split = line.split(": ");
        let header = line_split.next().unwrap();
        let header_value = line_split.next().unwrap();
        request.set_header(header, header_value);
    }

    // Set Body
    request.body = Some(lines.fold("".to_string(), |body, line| body + line));
    println!("Request: {:?}", request);
    request
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_REQUEST: &'static str = "GET / HTTP/1.1
Content-Type: application/json
User-Agent: PostmanRuntime/7.41.1
Accept: */*
Postman-Token: 528a9c5d-0c58-4b21-8eb2-ffcad859bbd2
Host: 127.0.0.1:7878
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
Content-Length: 28

{
    \"message\": \"Hello\"
}
";

    #[test]
    fn parse_request_nominal() {
        let buffer = &TEST_REQUEST.as_bytes();
        parse_request(buffer);
        assert!(true);
    }
}
