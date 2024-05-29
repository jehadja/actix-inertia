pub mod actix;
pub mod inertia_responder;
pub use actix::{example_handler, ResponseFactory, VersionMiddleware};

pub static X_INERTIA: &str = "X-Inertia";
pub static X_INERTIA_VERSION: &str = "X-Inertia-Version";
pub static X_INERTIA_LOCATION: &str = "X-Inertia-Location";
pub static X_INERTIA_ERROR_BAG: &str = "X-Inertia-Error-Bag";
pub static X_INERTIA_PARTIAL_COMPONENT: &str = "X-Inertia-Partial-Component";
pub static X_INERTIA_PARTIAL_ONLY: &str = "X-Inertia-Partial-Data";
pub static X_INERTIA_PARTIAL_EXCEPT: &str = "X-Inertia-Partial-Except";

pub struct Inertia<T> {
    component: String,
    props: T,
    url: Option<String>,
}
