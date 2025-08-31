use actix_web::{HttpRequest, HttpResponse};
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

    pub async fn respond_to(self, req: &HttpRequest) -> HttpResponse {
        let inertia = Inertia::new(self.component, self.props, req.uri().to_string());

        inertia.into_response(req).await
    }
}
