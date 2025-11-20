use axum::{
    extract::Request,
    response::Response,
};
use tower::{Layer, Service};
use uuid::Uuid;

#[derive(Clone)]
pub struct RequestIdLayer;

impl RequestIdLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestIdMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for RequestIdMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let request_id = Uuid::new_v4().to_string();
        req.headers_mut().insert(
            "x-request-id",
            request_id.parse().unwrap(),
        );

        let fut = self.inner.call(req);
        
        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut().insert(
                "x-request-id",
                request_id.parse().unwrap(),
            );
            Ok(res)
        })
    }
}
