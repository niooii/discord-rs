use crate::channel::ChannelType;
use reqwest::header::{self, HeaderName};
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
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

    #[error("Serde error")]
    SerdeError { err: serde_json::Error },

    #[error("Unauthorized")]
    Unauthorized { res: Response },

    #[error("Wrong Channel Type")]
    WrongChannelType { correct_type: ChannelType },

    #[error("Wrong Channel Type")]
    RateLimitReached { retry_after: f64 },
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

pub async fn get_data<T>(client: &Client, url: &str, payload: Option<()>) -> Result<T, QueryError>
where
    T: DeserializeOwned,
{
    if let Some(p) = payload {
        // send payload and get response
        Err(QueryError::Placeholder)
    } else {
        // parse no payload data
        let res = client.get(url).send().await;

        if let Err(e) = res {
            return Err(QueryError::ReqwestError { err: e });
        }

        let json = validate_ratelimit(res.unwrap()).await?;

        json_to_type::<T>(json).await
    }
}

pub async fn get_as_json(client: &Client, url: &str) -> Result<serde_json::Value, QueryError> {
    get_data::<serde_json::Value>(client, url, None).await
}

pub async fn validate_ratelimit(res: Response) -> Result<serde_json::Value, QueryError> {
    let json = res.json::<serde_json::Value>().await.expect("could not turn response into json");
    if let Some(val) = json.get("retry_after") {
        return Err(QueryError::RateLimitReached { retry_after: val.as_f64().unwrap() });
    }

    Ok(json)
}