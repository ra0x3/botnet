/// Botnet middleware services.
use crate::{prelude::*, utils::type_id};
use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Botnet middleware params.
#[derive(Clone)]
struct BotnetState {
    /// Botnet params.
    params: BotnetContext,
}

impl From<BotnetContext> for BotnetState {
    /// Create a new `BotnetState` from `BotnetContext`.
    fn from(params: BotnetContext) -> Self {
        Self { params }
    }
}

/// Botnet middleware.
#[derive(Clone)]
pub struct BotnetMiddleware {
    /// Botnet state.
    state: BotnetState,
}

impl From<BotnetContext> for BotnetMiddleware {
    /// Create a new `BotnetMiddleware` from `BotnetContext`.
    fn from(val: BotnetContext) -> Self {
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

        let context = BotnetContext::default();
        let extractors = self.state.params.extractors();
        let metadata = self.state.params.metadata();

        let _keys = self
            .state
            .params
            .config()
            .keys
            .iter()
            .map(|k| {
                let ty_id = type_id(k.name.as_str());
                let exts = extractors.get(&ty_id).unwrap();
                let meta = metadata.get(&ty_id).unwrap();

                BotnetKey::from_input(&input, exts, meta).unwrap_or_default()
            })
            .collect::<Vec<BotnetKey>>();

        req.extensions_mut().insert(context);

        let fut = self.inner.call(req);

        Box::pin(async move {
            let resp: Response = fut.await?;
            Ok(resp)
        })
    }
}
