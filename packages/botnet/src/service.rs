use crate::{
    core::{task::Strategy, BotnetMeta},
    prelude::{BotnetKey, FieldExtractors, Input, KeyMetadata},
    utils::type_id,
};
use axum::http::Request;
use lazy_static::lazy_static;
use std::task::{Context, Poll};
use tower::{Layer, Service};

lazy_static! {
    pub static ref FIELD_EXTRACTORS: FieldExtractors = FieldExtractors::default();
    pub static ref KEY_METADATA: KeyMetadata = KeyMetadata::default();
}

#[derive(Clone)]
struct BotnetState {
    botnet: BotnetMeta,
}

#[derive(Clone)]
pub struct BotnetMiddleware {
    state: BotnetState,
}

impl From<BotnetMeta> for BotnetMiddleware {
    fn from(val: BotnetMeta) -> Self {
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
                let exts = self
                    .state
                    .botnet
                    .extractors
                    .get(&ty_id)
                    .unwrap_or(&FIELD_EXTRACTORS);
                let meta = self
                    .state
                    .botnet
                    .metadata
                    .get(&ty_id)
                    .unwrap_or(&KEY_METADATA);
                BotnetKey::from_input(&input, exts, meta).unwrap_or_default()
            })
            .collect::<Vec<BotnetKey>>();

        let _strategy = Strategy::new(self.state.botnet.clone());

        self.inner.call(req)
    }
}
