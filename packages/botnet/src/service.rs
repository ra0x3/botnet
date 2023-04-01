use crate::{
    core::{task::Strategy, Botnet, BotnetParams},
    prelude::{BotnetKey, FieldExtractors, Input, KeyMetadata},
    utils::type_id,
};
use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use lazy_static::lazy_static;
use std::{
    rc::Rc,
    task::{Context, Poll},
};
use tower::{Layer, Service};

lazy_static! {
    pub static ref FIELD_EXTRACTORS: FieldExtractors = FieldExtractors::default();
    pub static ref KEY_METADATA: KeyMetadata = KeyMetadata::default();
}

#[derive(Clone)]
struct BotnetState {
    params: BotnetParams,
}

#[derive(Clone)]
pub struct BotnetMiddleware {
    state: BotnetState,
}

impl From<BotnetParams> for BotnetMiddleware {
    fn from(val: BotnetParams) -> Self {
        Self {
            state: BotnetState { params: val },
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

impl<S> Service<Request<Body>> for BotnetService<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let input: Input = req.uri().into();

        let botnet = Botnet::default();

        let keys = self
            .state
            .params
            .config
            .keys()
            .iter()
            .map(|k| {
                let ty_id = type_id(k.name());
                let exts = self
                    .state
                    .params
                    .extractors
                    .get(&ty_id)
                    .unwrap_or(&FIELD_EXTRACTORS);
                let meta = self
                    .state
                    .params
                    .metadata
                    .get(&ty_id)
                    .unwrap_or(&KEY_METADATA);

                BotnetKey::from_input(&input, exts, meta).unwrap_or_default()
            })
            .collect::<Vec<BotnetKey>>();

        let strategy = Rc::new(Strategy::new(self.state.params.clone()));

        let _counts = keys
            .iter()
            .filter_map(|k| {
                let s = strategy.clone();
                if s.clone().entity_counting_enabled() {
                    let _ = s.clone().count_entity(&k);
                    None
                } else {
                    None
                }
            })
            .collect::<Vec<()>>();

        req.extensions_mut().insert(botnet);

        let fut = self.inner.call(req);

        Box::pin(async move {
            let resp: Response = fut.await?;
            Ok(resp)
        })
    }
}
