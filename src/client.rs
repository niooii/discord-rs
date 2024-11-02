#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;
use rand::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use reqwest::Client;
use crate::http::QueryError;
use crate::model::Snowflake;
use crate::{api, http};
use crate::model::user::MainUserData;

#[derive(Error, Debug)]
pub enum DiscordBuildError {
    #[error("QueryError: {err}")]
    QueryError { err: QueryError },

    #[error("InternetUnavailable")]
    InternetUnavailable,

    #[error("ReqwestError: {err}")]
    ReqwestError { err: reqwest::Error },
}

#[derive(Debug)]
pub struct DiscordClient {
    me: MainUserData,
    req_client: reqwest::Client,
}

impl DiscordClient {
    pub fn req_client(&self) -> Client {
        self.req_client.clone()
    }

    pub fn user_id(&self) -> &Snowflake {
        &self.me.id
    }
}

#[derive(Default)]
pub struct DiscordClientBuilder {
    auth: String,
    user_agent: String,
}

impl DiscordClientBuilder {
    /// A builder for the discord client.
    /// The authentication token is required.
    pub fn new(auth: &str) -> DiscordClientBuilder {
        DiscordClientBuilder {
            auth: auth.to_string(),
            ..Default::default()
        }
    }

    /// Builds client for use.
    pub async fn build(self) -> Result<DiscordClient, DiscordBuildError> {
        let req_client = http::build_request_client(&self.auth, &self.user_agent)
        .map_err(|e| DiscordBuildError::ReqwestError { err: e })?;

        Ok(
            DiscordClient {
                me: api::get_authenticated_user_data(req_client.clone()).await.map_err(|e| DiscordBuildError::QueryError { err: e })?,
                req_client,
            }
        )
    }

    /// Set's the user agent to the specificed string.
    pub fn set_user_agent(mut self, user_agent: &str) -> DiscordClientBuilder {
        self.user_agent = user_agent.to_string();
        self
    }

    /// Set's the user agent for the client to a random agent.
    /// Selected from a list of 1000 agents.
    pub fn set_random_agent(mut self, seed: u64) -> DiscordClientBuilder {
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let t = rng.gen_range(0..1000);

        let reader = BufReader::new(
            File::open("discord/resources/user-agents.txt").expect("Cannot open file"),
        );
        let user_agent = reader
            .lines()
            .nth(t)
            .expect("Index is out of bounds.")
            .expect("Could not read nth line.");

        self.user_agent = user_agent;

        self
    }
}


