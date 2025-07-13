use crate::{
    Inertia, X_INERTIA, X_INERTIA_ERROR_BAG, X_INERTIA_LOCATION,
    X_INERTIA_PARTIAL_COMPONENT, X_INERTIA_PARTIAL_EXCEPT, X_INERTIA_PARTIAL_ONLY,
    X_INERTIA_VERSION,
};
use actix_service::{forward_ready, Service, Transform};
use actix_web::body::EitherBody;
use actix_web::web;
use actix_web::{
    dev::ServiceRequest, dev::ServiceResponse, http, Error, HttpRequest, HttpResponse, Result,
};
use futures::future::{ok, Ready};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
pub struct InertiaResponse<T> {
    component: String,
    props: T,
    url: String,
    version: Option<String>,
}

#[derive(Serialize)]
pub struct HtmlResponseContext {
    data_page: String,
}

#[derive(Clone)]
pub struct ResponseFactory {
    root_view: String,
    shared_props: Arc<Mutex<serde_json::Value>>,
    version: Option<Arc<dyn Fn() -> String + Send + Sync>>,
}

impl ResponseFactory {
    pub fn new() -> Self {
        Self {
            root_view: "app".to_string(),
            shared_props: Arc::new(Mutex::new(serde_json::Value::Object(Default::default()))),
            version: None,
        }
    }

    pub fn set_root_view(&mut self, name: &str) {
        self.root_view = name.to_string();
    }

    pub fn share(&self, key: &str, value: serde_json::Value) {
        let mut shared_props = self.shared_props.lock().unwrap();
        shared_props[key] = value;
    }

    pub fn get_shared(&self, key: Option<&str>) -> serde_json::Value {
        let shared_props = self.shared_props.lock().unwrap();
        match key {
            Some(k) => shared_props
                .get(k)
                .cloned()
                .unwrap_or(serde_json::Value::Null),
            None => shared_props.clone(),
        }
    }

    pub fn flush_shared(&self) {
        let mut shared_props = self.shared_props.lock().unwrap();
        *shared_props = serde_json::Value::Object(Default::default());
    }

    pub fn set_version<F>(&mut self, version: F)
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        self.version = Some(Arc::new(version));
    }

    pub fn get_version(&self) -> String {
        match &self.version {
            Some(version_fn) => version_fn(),
            None => "".to_string(),
        }
    }

    pub fn render<T: Serialize>(
        &self,
        component: &str,
        props: T,
        url: &str,
    ) -> Inertia<serde_json::Value> {
        let shared_props = self.get_shared(None);
        let mut props = serde_json::to_value(props).unwrap();
        if let serde_json::Value::Object(ref mut p) = props {
            if let serde_json::Value::Object(ref s) = shared_props {
                p.extend(s.clone());
            }
        }
        Inertia::new(component.to_string(), props, url.to_string())
    }

    pub fn location(&self, url: &str) -> HttpResponse {
        HttpResponse::Conflict()
            .append_header((X_INERTIA_LOCATION, url))
            .finish()
    }
}

impl<T: Serialize> Inertia<T> {
    pub fn new(component: String, props: T, url: String) -> Self {
        Self {
            component,
            props,
            url: Some(url),
        }
    }

    pub async fn into_response(self, req: &HttpRequest) -> HttpResponse {
        let version = req
            .app_data::<web::Data<ResponseFactory>>()
            .and_then(|factory| {
                let v = factory.get_version();
                if v.is_empty() {
                    None
                } else {
                    Some(v)
                }
            });

        let mut props_value = serde_json::to_value(self.props).unwrap_or_else(|_| serde_json::Value::Null);

        if let serde_json::Value::Object(ref mut map) = props_value {
            let partial_component = req
                .headers()
                .get(X_INERTIA_PARTIAL_COMPONENT)
                .and_then(|v| v.to_str().ok());
            let partial_only = req
                .headers()
                .get(X_INERTIA_PARTIAL_ONLY)
                .and_then(|v| v.to_str().ok());
            let partial_except = req
                .headers()
                .get(X_INERTIA_PARTIAL_EXCEPT)
                .and_then(|v| v.to_str().ok());

            let should_filter = match partial_component {
                Some(comp) => comp == self.component,
                None => partial_only.is_some() || partial_except.is_some(),
            };

            if should_filter {
                if let Some(data) = partial_only {
                    let keys: Vec<&str> = data.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
                    map.retain(|k, _| keys.iter().any(|key| key == &k.as_str()));
                }

                if let Some(excepts) = partial_except {
                    for key in excepts.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
                        map.remove(key);
                    }
                }
            }
        }

        let inertia_response = InertiaResponse {
            component: self.component,
            props: props_value,
            url: self.url.unwrap_or_else(|| req.uri().to_string()),
            version,
        };

        if req.headers().contains_key(X_INERTIA) {
            let mut response = HttpResponse::Ok()
                .content_type("application/json")
                .append_header((X_INERTIA, "true"))
                .json(inertia_response);

            if let Some(error_bag) = req.headers().get(X_INERTIA_ERROR_BAG) {
                response.headers_mut().append(
                    http::header::HeaderName::from_static("x-inertia-error-bag"),
                    error_bag.clone(),
                );
            }

            if let Some(partial_component) = req.headers().get(X_INERTIA_PARTIAL_COMPONENT) {
                response.headers_mut().append(
                    http::header::HeaderName::from_static("x-inertia-partial-component"),
                    partial_component.clone(),
                );
            }

            if let Some(partial_only) = req.headers().get(X_INERTIA_PARTIAL_ONLY) {
                response.headers_mut().append(
                    http::header::HeaderName::from_static("x-inertia-partial-data"),
                    partial_only.clone(),
                );
            }

            if let Some(partial_except) = req.headers().get(X_INERTIA_PARTIAL_EXCEPT) {
                response.headers_mut().append(
                    http::header::HeaderName::from_static("x-inertia-partial-except"),
                    partial_except.clone(),
                );
            }

            response
        } else {
            let ctx = HtmlResponseContext {
                data_page: serde_json::to_string(&inertia_response).unwrap(), // Handle error in real scenario
            };
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(ctx.data_page)
        }
    }
}

// Example handler
pub async fn example_handler(
    req: HttpRequest,
    data: web::Data<ResponseFactory>,
) -> impl actix_web::Responder {
    let inertia = data.render(
        "ComponentName",
        serde_json::json!({"prop_key": "prop_value"}),
        req.uri().to_string().as_str(),
    );
    #[cfg(debug_assertions)]
    eprintln!("Handler - Request: {:?}", req);

    inertia.into_response(&req).await
}

pub struct VersionMiddleware {
    version: String,
}

impl VersionMiddleware {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}

impl<S, B> Transform<S, ServiceRequest> for VersionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = VersionMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(VersionMiddlewareService {
            service,
            version: self.version.clone(),
        })
    }
}

pub struct VersionMiddlewareService<S> {
    service: S,
    version: String,
}

impl<S, B> Service<ServiceRequest> for VersionMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        #[cfg(debug_assertions)]
        eprintln!("Middleware - ServiceRequest: {:?}", req);

        if req.method() == http::Method::GET && req.headers().contains_key(X_INERTIA) {
            let request_version = req
                .headers()
                .get(X_INERTIA_VERSION)
                .map(|v| v.to_str().unwrap_or("").to_string());

            #[cfg(debug_assertions)]
            eprintln!("Middleware - Request Version: {:?}", request_version);

            if request_version.is_none() || request_version.as_deref() != Some(&self.version) {
                let uri = format!(
                    "{}?location={}",
                    "/inertia-rs/version-conflict",
                    req.uri().path()
                );

                Box::pin(async move {
                    let (req, _) = req.into_parts();
                    let res = HttpResponse::Conflict()
                        .append_header((X_INERTIA_LOCATION, uri))
                        .finish()
                        .map_into_right_body();
                    Ok(ServiceResponse::new(req, res))
                })
            } else {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            }
        } else {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            })
        }
    }
}
