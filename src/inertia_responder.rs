use actix_web::{HttpRequest, HttpResponse, Responder};
use futures_util::future::LocalBoxFuture;
use serde::Serialize;

use crate::Inertia;

pub struct InertiaResponder<T: Serialize> {
    component: String,
    props: T,
}

impl<T: Serialize> InertiaResponder<T> {
    pub fn new(component: impl Into<String>, props: T) -> Self {
        Self {
            component: component.into(),
            props,
        }
    }
}

impl<T> Responder for InertiaResponder<T>
where
    T: Serialize + 'static,
{
    type Body = actix_web::body::BoxBody;
    type Future = LocalBoxFuture<'static, HttpResponse<Self::Body>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let req = req.clone();
        let component = self.component;
        let props = self.props;

        Box::pin(async move {
            let inertia = Inertia::new(component, props, req.uri().to_string());
            inertia.into_response(&req).await
        })
    }
}
