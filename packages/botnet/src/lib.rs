use crate::prelude::{Extractors, InMemory, Input, Key, Metadata};
use axum::http::Request;
use std::{
    collections::HashMap,
    task::{Context, Poll},
};
use tower::{Layer, Service};

pub mod core {
    pub use botnet_core::{botnet_key, *};
}

pub mod prelude {
    pub use botnet_core::prelude::*;
}

pub mod macros {
    pub use botnet_macros::*;
}

#[derive(Clone)]
pub struct BotnetStateConfig<K: Key + Clone> {
    pub keys: HashMap<usize, Box<K>>,
    pub metadata: Metadata,
    pub extractors: Extractors,
    pub db: Option<InMemory>,
}

#[derive(Clone)]
struct BotnetState<K>
where
    K: Key + Clone,
{
    config: BotnetStateConfig<K>,
}

#[derive(Clone)]
pub struct BotnetMiddleware<K: Key + Clone> {
    state: BotnetState<K>,
}

impl<K> From<&BotnetStateConfig<K>> for BotnetMiddleware<K>
where
    K: Key + Clone,
{
    fn from(config: &BotnetStateConfig<K>) -> Self {
        Self {
            state: BotnetState {
                config: config.clone(),
            },
        }
    }
}

impl<S, K> Layer<S> for BotnetMiddleware<K>
where
    K: Key + Clone,
{
    type Service = BotnetService<S, K>;

    fn layer(&self, inner: S) -> Self::Service {
        BotnetService {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct BotnetService<S, K: Key + Clone> {
    inner: S,
    state: BotnetState<K>,
}

impl<S, B, K> Service<Request<B>> for BotnetService<S, K>
where
    S: Service<Request<B>>,
    K: Key + Clone,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let _config = &self.state.config;
        let _input: Input = req.uri().into();

        // todo handle the input as needed here

        self.inner.call(req)
    }
}
