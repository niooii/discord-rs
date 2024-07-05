use crate::api::cha;
use reqwest::header::{self, HeaderName};
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

pub fn build_request_client(auth: &String, ua: &String) -> Result<Client, reqwest::Error> {
    let mut headers = HeaderMap::new();

    let mut auth_value =
        header::HeaderValue::from_str(auth).expect("Failed to convert auth token to header.");
    auth_value.set_sensitive(true);
    headers.insert("Authorization", auth_value);

    let cb = ClientBuilder::new();
    cb.https_only(true)
        .cookie_store(true)
        // actual stuff for discord
        .user_agent(ua)
        .default_headers(headers)
        .build()
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Placeholder error")]
    Placeholder,

    #[error("Reqwest error")]
    ReqwestError { err: reqwest::Error },

    #[error("Serde error: {err:?}")]
    SerdeError { err: serde_json::Error },

    #[error("Unauthorized")]
    Unauthorized { res: Response },

    #[error("Ratelimit reached: try again after {retry_after} seconds")]
    RateLimitReached { retry_after: f64 },

    #[error("Unhandled Error: {error}")]
    Other { error: String },
}

async fn res_to_type<T>(res: Response) -> Result<T, QueryError>
where
    T: DeserializeOwned,
{
    let ret = res.json::<T>().await;

    if let Err(e) = ret {
        Err(QueryError::ReqwestError { err: e })
    } else {
        Ok(ret.unwrap())
    }
}

pub async fn json_to_type<T>(json: serde_json::Value) -> Result<T, QueryError>
where
    T: DeserializeOwned,
{
    let ret = serde_json::from_value::<T>(json);

    if let Err(e) = ret {
        Err(QueryError::SerdeError { err: e })
    } else {
        Ok(ret.unwrap())
    }
}

async fn validate_auth(auth: &String) {}

#[derive(Serialize)]
pub struct Payload;

pub async fn get_data<T>(client: Client, url: &str, payload: Option<Payload>, method: reqwest::Method) -> Result<T, QueryError>
where
    T: DeserializeOwned,
{
    let json = get_json(client, url, payload, method).await?;
    json_to_type::<T>(json).await
}

pub async fn get_json(client: Client, url: &str, payload: Option<Payload>, method: reqwest::Method) -> Result<serde_json::Value, QueryError> {
    let req = if let Some(p) = payload {
        client.request(method, url).json(&p)
    } else {
        client.request(method, url)
    };

    let res = req.send().await
        .map_err(|e| QueryError::ReqwestError { err: e })?;
    let value = validate_ratelimit(res).await;
    value
}

pub async fn validate_ratelimit(res: Response) -> Result<serde_json::Value, QueryError> {
    let response_text = res.text().await.unwrap();
    // if response_text.is_empty() {
    //     return 
    // }
    let json = match serde_json::from_str::<Value>(&response_text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to deserialize: {response_text}");
            // TODO! check this out its not a reqwest error wtf
            return Err(QueryError::SerdeError { err: e });
        }
    };

    if let Some(val) = json.get("retry_after") {
        return Err(QueryError::RateLimitReached { retry_after: val.as_f64().unwrap() });
    }

    Ok(json)
}