#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

use err_derive::Error;
use rand::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use reqwest::Client;
use crate::http;
use crate::http::validate_ratelimit;
use http::QueryError;
use serde::Serialize;
use crate::user::*;
use crate::channel;
use channel::*;
use crate::endpoints;

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

use crate::channel::{*};
use crate::user::*;

// DISCORD STRUCTS

// OTHER STUFF

#[derive(Debug, Error)]
pub enum DiscordBuildError {
    #[error(display = "{}", auth)]
    AuthNotValid { auth: String },

    #[error(display = "InternetUnavailable")]
    InternetUnavailable,

    #[error(display = "ReqwestError: {}", err)]
    ReqwestError { err: reqwest::Error },
}

#[derive(Debug)]
pub struct DiscordClient {
    // auth: String,
    req_client: reqwest::Client,
}

impl DiscordClient {
    pub fn req_client(&self) -> Client {
        self.req_client.clone()
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
    pub fn builder(auth: &str) -> DiscordClientBuilder {
        DiscordClientBuilder {
            auth: auth.to_string(),
            ..Default::default()
        }
    }

    /// Builds client for use.
    pub fn build(self) -> Result<DiscordClient, DiscordBuildError> {
        let client = DiscordClient {
            req_client: http::build_request_client(&self.auth, &self.user_agent)
                .map_err(|e| DiscordBuildError::ReqwestError { err: e })?,
            // auth: self.auth,
        };

        Ok(client)
    }

    /// Set's the user agent to the specificed string.
    pub fn set_user_agent(&mut self, user_agent: &str) -> &mut DiscordClientBuilder {
        self.user_agent = user_agent.to_string();
        self
    }

    /// Set's the user agent for the client to a random agent.
    /// Selected from a list of 1000 agents.
    pub fn set_random_agent(&mut self, seed: u64) -> &mut DiscordClientBuilder {
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

    /// Disables all safetey mechanisms.
    pub fn i_am_really_stupid(&mut self) -> &mut DiscordClientBuilder {
        self
    }
}

