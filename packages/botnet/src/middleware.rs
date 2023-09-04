use crate::core::{
    config::*,
    context::BotnetContext,
    models::{input::Input, key::BotnetKey},
};
/// Botnet middleware services.
use async_std::sync::Arc;
use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Botnet middleware params.
#[derive(Clone)]
struct BotnetState<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// Botnet params.
    params: Arc<BotnetContext<E, A, C>>,
}

impl<E, A, C> From<Arc<BotnetContext<E, A, C>>> for BotnetState<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// Create a new `BotnetState` from `BotnetContext`.
    fn from(params: Arc<BotnetContext<E, A, C>>) -> Self {
        Self { params }
    }
}

/// Botnet middleware.
#[derive(Clone)]
pub struct BotnetMiddleware<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// Botnet state.
    state: Arc<BotnetState<E, A, C>>,
}

impl<E, A, C> From<Arc<BotnetContext<E, A, C>>> for BotnetMiddleware<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// Create a new `BotnetMiddleware` from `BotnetContext`.
    fn from(val: Arc<BotnetContext<E, A, C>>) -> Self {
        let state: BotnetState<E, A, C> = val.into();
        Self {
            state: Arc::new(state),
        }
    }
}

impl<S, E, A, C> Layer<S> for BotnetMiddleware<E, A, C>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    type Service = BotnetService<S, E, A, C>;

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
pub struct BotnetService<S, E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// Inner service.
    inner: S,

    /// Botnet state.
    state: Arc<BotnetState<E, A, C>>,
}

impl<S, E, A, C> Service<Request<Body>> for BotnetService<S, E, A, C>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
    E: EntityCounter + Send + Sync + 'static,
    A: Anonimity + Default + Send + Sync + 'static,
    C: RateLimit + Send + Sync + 'static,
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
        let _keys = self
            .state
            .params
            .keys()
            .iter()
            .map(|(ty_id, _)| {
                let exts = self
                    .state
                    .params
                    .get_extractors(ty_id)
                    .expect("Invalid type ID for extractors.");
                let meta = self
                    .state
                    .params
                    .get_metadata(ty_id)
                    .expect("Invalid type ID for metadata.");
                BotnetKey::from((&input, exts, meta))
            })
            .collect::<Vec<BotnetKey>>();
        req.extensions_mut().insert(self.state.params.clone());

        let fut = self.inner.call(req);

        Box::pin(async move {
            let resp: Response = fut.await?;
            Ok(resp)
        })
    }
}
