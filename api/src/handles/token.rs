// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;

use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

use crate::context::context::Context;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTokenRequest {
    pub id: String,
    pub permissions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

impl From<String> for TokenResponse {
    fn from(token: String) -> Self {
        TokenResponse {
            access_token: token,
        }
    }
}

pub fn create_token(ctx: Context) -> BoxedFilter<(impl Reply,)> {
    warp::path("token")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json::<CreateTokenRequest>())
        .and(ctx.filter())
        .and_then(handle_create_token)
        .boxed()
}

async fn handle_create_token(
    info: CreateTokenRequest,
    ctx: Context,
) -> Result<impl Reply, Rejection> {
    let claims = ctx
        .jwt_claim(info.id, info.permissions)
        .expect("Failed to encode");
    Ok(warp::reply::json(&TokenResponse::from(
        ctx.jwt_encode(claims).unwrap(),
    )))
}
