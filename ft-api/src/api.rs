#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub result: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error: String,
}

#[derive(Deserialize, Debug)]
pub enum Error {
    InvalidAuthCode,
    RepoNotFound,
    CollectionNotFound,
    InvalidID,
    HashNotMatching,
    InvalidFileName(String),
    BadFTD(String),
    NoPermission(String),
    DBError,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::InvalidAuthCode => "InvalidAuthCode".to_string(),
            Error::RepoNotFound => "RepoNotFound".to_string(),
            Error::CollectionNotFound => "CollectionNotFound".to_string(),
            Error::InvalidID => "InvalidID".to_string(),
            Error::HashNotMatching => "HashNotMatching".to_string(),
            Error::InvalidFileName(name) => format!("InvalidFileName: {}", name),
            Error::BadFTD(s) => format!("BadFTD: {}", s),
            Error::NoPermission(p) => format!("NoPermission: {}", p),
            Error::DBError => "DBError".to_string(),
        }
    }
}

fn to_url(url: &str) -> String {
    // TODO: read domain from config
    format!("http://127.0.0.1:3000{}?realm_mode=api", url)
}

fn get_util(url: &str) -> crate::Result<serde_json::Value> {
    let client = reqwest::blocking::Client::new();
    match client
        .get(to_url(url))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("User-Agent", "rust")
        .send()
    {
        Ok(response) => {
            if response.status() != reqwest::StatusCode::OK {
                Err(crate::error::Error::APIResponseNotOk("api response not OK".to_string()).into())
            } else {
                response.json().map_err(|e| e.into())
            }
        }
        Err(e) => Err(crate::error::Error::APIError { error: e }.into()),
    }
}

// TODO: convert it to a macro so key values can be passed easily
pub fn get<T: serde::de::DeserializeOwned, K, V>(
    url: &str,
    _query: std::collections::HashMap<K, V>,
) -> crate::Result<ApiResponse<T>>
where
    K: Into<String>,
    V: Into<String>,
{
    match get_util(url) {
        Ok(response) => serde_json::from_value(response)
            .map_err(|e| crate::error::Error::DeserializeError(e.to_string()).into()),
        Err(e) => Err(e),
    }
}

fn post_util<B: Into<reqwest::blocking::Body>>(
    url: &str,
    body: B,
) -> crate::Result<serde_json::Value> {
    let client = reqwest::blocking::Client::new();
    match client
        .post(to_url(url))
        .body(body)
        .header("content-type", "application/json")
        .header("Accept", "application/json")
        .header("user-agent", "rust")
        .send()
    {
        Ok(response) => {
            if response.status() != reqwest::StatusCode::OK {
                Err(
                    crate::error::Error::APIResponseNotOk("post api response not OK".to_string())
                        .into(),
                )
            } else {
                response.json().map_err(|e| e.into())
            }
        }
        Err(e) => Err(crate::error::Error::APIError { error: e }.into()),
    }
}

pub fn post<T: serde::de::DeserializeOwned, B: Into<reqwest::blocking::Body>>(
    url: &str,
    body: B,
) -> crate::Result<ApiResponse<T>> {
    match post_util(url, body) {
        Ok(response) => serde_json::from_value(response)
            .map_err(|e| crate::error::Error::DeserializeError(e.to_string()).into()),
        Err(e) => Err(e),
    }
}
