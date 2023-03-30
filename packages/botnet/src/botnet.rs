use crate::{
    core::config::BotnetConfig,
    prelude::{BotnetKey, Extractors, InMemory, Input, Metadata},
    utils::type_id,
};
use axum::http::Request;
use std::{
    collections::HashMap,
    task::{Context, Poll},
};
use tower::{Layer, Service};

#[derive(Clone, Default)]
pub struct Botnet {
    pub keys: HashMap<usize, BotnetKey>,
    pub metadata: Metadata,
    pub extractors: Extractors,
    pub db: Option<InMemory>,
    pub config: BotnetConfig,
}

impl From<BotnetConfig> for Botnet {
    fn from(val: BotnetConfig) -> Self {
        Self {
            config: val,
            ..Self::default()
        }
    }
}

#[derive(Clone)]
struct BotnetState {
    botnet: Botnet,
}

#[derive(Clone)]
pub struct BotnetMiddleware {
    state: BotnetState,
}

impl From<Botnet> for BotnetMiddleware {
    fn from(val: Botnet) -> Self {
        Self {
            state: BotnetState { botnet: val },
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
        let input: Input = req.uri().into();

        let _keys = self
            .state
            .botnet
            .config
            .keys()
            .iter()
            .map(|k| {
                let ty_id = type_id(k.name());
                let exts = self.state.botnet.extractors.get(&ty_id);
                let meta = self.state.botnet.metadata.get(&ty_id);
                BotnetKey::from_input(&input, exts, meta).unwrap()
            })
            .collect::<Vec<BotnetKey>>();

        self.inner.call(req)
    }
}
