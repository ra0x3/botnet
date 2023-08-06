/// Botnet middleware services.
use crate::{prelude::*, utils::type_id};
use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use lazy_static::lazy_static;
use std::{
    rc::Rc,
    task::{Context, Poll},
};
use tower::{Layer, Service};

lazy_static! {

    /// Default set of field extractors to be used in botnet middleware.
    pub static ref FIELD_EXTRACTORS: FieldExtractors = FieldExtractors::default();

    /// Default set of key metadata to be used in botnet middleware.
    pub static ref KEY_METADATA: BotnetKeyMetadata = BotnetKeyMetadata::default();
}

/// Botnet middleware params.
#[derive(Clone)]
struct BotnetState {
    /// Botnet params.
    params: BotnetParams,
}

impl From<BotnetParams> for BotnetState {
    /// Create a new `BotnetState` from `BotnetParams`.
    fn from(params: BotnetParams) -> Self {
        Self { params }
    }
}

/// Botnet middleware.
#[derive(Clone)]
pub struct BotnetMiddleware {
    /// Botnet state.
    state: BotnetState,
}

impl From<BotnetParams> for BotnetMiddleware {
    /// Create a new `BotnetMiddleware` from `BotnetParams`.
    fn from(val: BotnetParams) -> Self {
        let state: BotnetState = val.into();
        Self { state }
    }
}

impl<S> Layer<S> for BotnetMiddleware {
    type Service = BotnetService<S>;

    /// Wrap the inner service with the botnet middleware.
    fn layer(&self, inner: S) -> Self::Service {
        BotnetService {
            inner,
            state: self.state.clone(),
        }
    }
}

/// Botnet middleware service.
#[derive(Clone)]
pub struct BotnetService<S> {
    /// Inner service.
    inner: S,

    /// Botnet state.
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

    /// Poll the inner service.
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    /// Call the inner service.
    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let input: Input = req.uri().into();

        let botnet = Botnet::default();
        let extractors = self.state.params.extractors();
        let metadata = self.state.params.metadata();

        let keys = self
            .state
            .params
            .config()
            .keys()
            .iter()
            .map(|k| {
                let ty_id = type_id(k.name());
                let exts = extractors.get(&ty_id).unwrap_or(&FIELD_EXTRACTORS);
                let meta = metadata.get(&ty_id).unwrap_or(&KEY_METADATA);

                BotnetKey::from_input(&input, exts, meta).unwrap_or_default()
            })
            .collect::<Vec<BotnetKey>>();

        let strategy = Rc::new(Strategy::new(self.state.params.clone()));

        let _results = keys
            .iter()
            .map(|k| {
                if strategy.entity_counting_enabled() {
                    strategy.count_entity(k).unwrap();
                }

                if strategy.kanon_enabled() {
                    strategy.is_k_anonymous(k).unwrap();
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
