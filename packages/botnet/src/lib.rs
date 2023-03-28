use crate::prelude::Input;
use axum::http::Request;
use std::task::{Context, Poll};
use tower::{Layer, Service};

pub mod prelude {
    pub use botnet_core::prelude::*;
}

pub mod macros {
    pub use botnet_macros::*;
}

#[derive(Clone)]
pub struct BotnetConfig {}

#[derive(Clone)]
struct BotnetState {
    config: BotnetConfig,
}

#[derive(Clone)]
pub struct BotnetMiddleware {
    state: BotnetState,
}

impl From<&BotnetConfig> for BotnetMiddleware {
    fn from(config: &BotnetConfig) -> Self {
        Self {
            state: BotnetState {
                config: config.clone(),
            },
        }
    }
}

impl<S> Layer<S> for BotnetMiddleware {
    type Service = BotnetService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        BotnetService {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct BotnetService<S> {
    inner: S,
    state: BotnetState,
}

impl<S, B> Service<Request<B>> for BotnetService<S>
where
    S: Service<Request<B>>,
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
