use actix_web::{HttpRequest, HttpResponse, Responder};
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

impl<T: Serialize> Responder for InertiaResponder<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let inertia = Inertia::new(self.component, self.props, req.uri().to_string());

        let response = futures::executor::block_on(async { inertia.into_response(req).await });

        response
    }
}
