use num::FromPrimitive;
use num_derive::FromPrimitive;
use reqwest::header;
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

use crate::api::DiscordError;

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

    #[error("Discord error: {error}")]
    DiscordError { error: DiscordError },

    #[error("Unhandled Error: {error}")]
    Other { error: String },
}

#[derive(FromPrimitive)]
#[repr(i64)]
pub enum DiscordErrorCode {
    Unauthorized = 0,
    WriteChannelRateLimitReached = 20028,
    Unknown = -1
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

pub async fn get_struct<T>(client: Client, url: &str, method: reqwest::Method) -> Result<T, QueryError>
where
    T: DeserializeOwned,
{
    let json = get_json(client, url, method).await?;
    json_to_type::<T>(json).await
}

pub async fn get_struct_body<T: DeserializeOwned, S: Serialize>(client: Client, url: &str, body: S, method: reqwest::Method) -> Result<T, QueryError>
{
    let json = get_json_body(client, url, body, method).await?;
    json_to_type::<T>(json).await
}

pub async fn get_json(client: Client, url: &str, method: reqwest::Method) -> Result<serde_json::Value, QueryError> {
    let res = client.request(method, url).send().await
        .map_err(|e| QueryError::ReqwestError { err: e })?;

    let value = validate_response(res).await;
    value
}

pub async fn get_json_body<T: Serialize>(client: Client, url: &str, body: T, method: reqwest::Method) -> Result<serde_json::Value, QueryError> {
    let res = client.request(method, url).json(&body).send().await
        .map_err(|e| QueryError::ReqwestError { err: e })?;
    let value = validate_response(res).await;
    value
}

fn err_or_json(json: Value) -> Result<Value, QueryError> {
    if let Some(code) = json.get("code") {
        let code = code.as_i64().unwrap();
        let err = match DiscordErrorCode::from_i64(code)
            .unwrap_or(DiscordErrorCode::Unknown) 
        {
            DiscordErrorCode::Unauthorized => DiscordError::AuthenticationFail,
            DiscordErrorCode::WriteChannelRateLimitReached => {
                DiscordError::RateLimitReached { 
                    retry_after: json.get("retry_after").unwrap().as_f64().unwrap() 
                }
            },
            DiscordErrorCode::Unknown => return None,
        };

        return Some(err);
    }

    None
}

pub async fn validate_response(res: Response) -> Result<Value, QueryError> {
    let response_text = res.text().await.unwrap();
    let json = match serde_json::from_str::<Value>(&response_text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to deserialize: {response_text}");
            return Err(QueryError::SerdeError { err: e });
        }
    };

    if let Some(e) = err_from_message(&json) {
        Err(QueryError::DiscordError { error: e })
    } else {
        Ok(json)
    }
}