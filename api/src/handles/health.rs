// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Result};

use warp::{filters::BoxedFilter, Filter, Reply};

pub fn health_check_route() -> BoxedFilter<(impl Reply,)> {
    warp::path!("-" / "health")
        .and(warp::path::end())
        .and_then(health_check)
        .boxed()
}

async fn health_check() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    Ok(Box::new("jojo-node:ok"))
}
