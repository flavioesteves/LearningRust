pub struct Response {
    status_code: Statuscode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: Statuscode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
}

pub enum Statuscode {}
