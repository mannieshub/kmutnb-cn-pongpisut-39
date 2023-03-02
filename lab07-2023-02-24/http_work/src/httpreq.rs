
#[derive(Debug, PartialEq)]
pub enum Method {
   Get,
   Post,
   Uninitialized,
}

impl From<&str> for Method {
   fn from(s: &str) -> Method {
       match s {
           "GET" => Method::Get,
           "POST" => Method::Post,
           _ => Method::Uninitialized,
       }
   }
}

#[derive(Debug, PartialEq)]
pub enum Version {
   V1_1,
   V2_0,
   Uninitialized,
}

impl From<&str> for Version {
   fn from(s: &str) -> Version {
       match s {
           "HTTP/1.1" => Version::V1_1,
           _ => Version::Uninitialized,
       }
   }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
    QueryString(String),
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: std::collections::HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = std::collections::HashMap::new();
        let mut parsed_msg_body = "";
        // Read each line in the incoming HTTP request
        for line in req.lines() {
            // If the line read is request line, call function process_req_line()
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            // If the line read is header line, call function process_header_line()
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            //  If it is blank line, do nothing
            } else if line.len() == 0 {
            // If none of these, treat it as message body
            } else {
                parsed_msg_body = line;
            }
        }
        // Parse the incoming HTTP request into HttpRequest struct
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    // Parse the request line into words split by space separator
    let mut req_items = s.split(" ");
    let mut method = Method::Uninitialized;
    let mut resource = Resource::Uninitialized;
    let mut version = Version::Uninitialized;
    // Extract the method part of the request
    if let Some(m) = req_items.next() {
        method = Method::from(m);
    }
    // Extract the resource part of the request
    if let Some(r) = req_items.next() {
        resource = parse_resource_string(r);
    }
    // Extract the version part of the request
    if let Some(v) = req_items.next() {
        version = Version::from(v);
    }
    (method, resource, version)
}

fn parse_resource_string(s: &str) -> Resource {
    if s.contains("?") {
        Resource::QueryString(s.to_string())
    } else {
        Resource::Path(s.to_string())
    }
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    if let Some(v) = header_items.next() {
        value = v.trim().to_string();
    }

    (key, value)
}



#[test]
fn test_parse_http_request() {
    let http_request_str = "GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n";
    let http_request: HttpRequest = http_request_str.to_string().into();

    assert_eq!(http_request.method, Method::Get);
    assert_eq!(http_request.version, Version::V1_1);
    assert_eq!(http_request.resource, Resource::Path("/greeting".to_string()));

    let mut headers_expected = std::collections::HashMap::new();
    headers_expected.insert("Host".to_string(), " localhost:3000".to_string());
    headers_expected.insert("User-Agent".to_string(), " curl/7.64.1".to_string());
    headers_expected.insert("Accept".to_string(), " */*".to_string());
    assert_eq!(http_request.headers, headers_expected);

    assert_eq!(http_request.msg_body, "");
}
