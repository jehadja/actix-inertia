use actix_inertia::{inertia_responder::InertiaResponder, VersionMiddleware};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json::json;
use serde_json::Value;
use std::fs::read_to_string;

#[derive(serde::Serialize)]
struct HelloProps {
    message: String,
}

async fn hello(req: HttpRequest) -> impl Responder {
    let props: HelloProps = HelloProps {
        message: "this is my message from Rust :)".to_string(),
    };
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("Hello", props).respond_to(&req).await
    } else {
        response_with_html(&req, props, "Hello".to_string())
    }
}

async fn world(req: HttpRequest) -> impl Responder {
    let props: HelloProps = HelloProps {
        message: "this is my message from Rust :) sceond page".to_string(),
    };
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("World", props).respond_to(&req).await
    } else {
        response_with_html(&req, props, "World".to_string())
    }
}

async fn version(req: HttpRequest) -> impl Responder {
    let props: HelloProps = HelloProps {
        message: "this is my message from Rust :) with Version 1".to_string(),
    };
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("VersionPage", props).respond_to(&req).await
    } else {
        response_with_html(&req, props, "VersionPage".to_string())
    }
}

fn response_with_html(req: &HttpRequest, props: HelloProps, component: String) -> HttpResponse {
    let html_path = "./my-inertia-app/public/index.html";
    let html = read_to_string(html_path).expect("Failed to read index.html");

    // Serialize the component and props to JSON
    let data_page: Value = json!({
        "component":component,
        "props": props,
        "url": req.uri().to_string()
    });

    let mut data_page_str = serde_json::to_string(&data_page).unwrap();
    data_page_str = data_page_str.replace("\"", "&quot;");

    // Replace the placeholder with the actual data-page attribute
    let html = html.replace("{{DATA_PAGE}}", &data_page_str);

    // Serve the modified HTML
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/hello", web::get().to(hello))
            .route("/world", web::get().to(world))
            .service(
                web::scope("/version")
                    .wrap(VersionMiddleware::new("1".to_string()))
                    .route("", web::get().to(version)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
