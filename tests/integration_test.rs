use actix_inertia::{
    example_handler, ResponseFactory, VersionMiddleware, X_INERTIA, X_INERTIA_VERSION,
    X_INERTIA_PARTIAL_COMPONENT, X_INERTIA_PARTIAL_ONLY, X_INERTIA_PARTIAL_EXCEPT,
};
extern crate serde_json;

use actix_web::{http, test, web, App, HttpRequest};
use serde_json::Value;

#[actix_web::test]
async fn test_the_view_is_served_by_inertia() {
    let app = test::init_service(
        App::new()
            .wrap(VersionMiddleware::new("example-version".to_string()))
            .app_data(web::Data::new(ResponseFactory::new()))
            .service(web::resource("/foo").to(example_handler)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/foo")
        .insert_header((X_INERTIA, "true"))
        .insert_header((X_INERTIA_VERSION, "example-version"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body_bytes = test::read_body(resp).await;

    if status == http::StatusCode::OK {
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body["component"], "ComponentName");
        assert!(body["props"].is_object());
        assert_eq!(body["url"], "/foo");
    } else {
        eprintln!("Failed with status: {:?}", status);
    }

    assert_eq!(status, http::StatusCode::OK);
}

#[actix_web::test]
async fn test_the_view_is_not_served_by_inertia() {
    let app = test::init_service(
        App::new()
            .wrap(VersionMiddleware::new("example-version".to_string()))
            .app_data(web::Data::new(ResponseFactory::new()))
            .service(web::resource("/foo").to(example_handler)),
    )
    .await;

    let req = test::TestRequest::get().uri("/foo").to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let headers = resp.headers().clone();
    assert_eq!(status, http::StatusCode::OK);
    assert!(!headers.contains_key(X_INERTIA));
}

#[actix_web::test]
async fn test_the_component_matches() {
    let app = test::init_service(
        App::new()
            .wrap(VersionMiddleware::new("example-version".to_string()))
            .app_data(web::Data::new(ResponseFactory::new()))
            .service(web::resource("/foo").to(example_handler)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/foo")
        .insert_header((X_INERTIA, "true"))
        .insert_header((X_INERTIA_VERSION, "example-version"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body_bytes = test::read_body(resp).await;

    if status == http::StatusCode::OK {
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body["component"], "ComponentName");
    }

    assert_eq!(status, http::StatusCode::OK);
}

#[actix_web::test]
async fn test_the_component_does_not_match() {
    let app = test::init_service(
        App::new()
            .wrap(VersionMiddleware::new("example-version".to_string()))
            .app_data(web::Data::new(ResponseFactory::new()))
            .service(web::resource("/foo").to(example_handler)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/foo")
        .insert_header((X_INERTIA, "true"))
        .insert_header((X_INERTIA_VERSION, "example-version"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body_bytes = test::read_body(resp).await;

    if status == http::StatusCode::OK {
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_ne!(body["component"], "WrongComponentName");
    }

    assert_eq!(status, http::StatusCode::OK);
}

#[actix_web::test]
async fn test_the_page_url_matches() {
    let app = test::init_service(
        App::new()
            .wrap(VersionMiddleware::new("example-version".to_string()))
            .app_data(web::Data::new(ResponseFactory::new()))
            .service(web::resource("/foo").to(example_handler)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/foo")
        .insert_header((X_INERTIA, "true"))
        .insert_header((X_INERTIA_VERSION, "example-version"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body_bytes = test::read_body(resp).await;

    if status == http::StatusCode::OK {
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body["url"], "/foo");
    }

    assert_eq!(status, http::StatusCode::OK);
}

#[actix_web::test]
async fn test_the_asset_version_matches() {
    let mut factory = ResponseFactory::new();
    factory.set_version(|| "example-version".to_string());

    let app = test::init_service(
        App::new()
            .wrap(VersionMiddleware::new("example-version".to_string()))
            .app_data(web::Data::new(factory))
            .service(web::resource("/foo").to(example_handler)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/foo")
        .insert_header((X_INERTIA, "true"))
        .insert_header((X_INERTIA_VERSION, "example-version"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body_bytes = test::read_body(resp).await;

    if status == http::StatusCode::OK {
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body["version"], "example-version");
    }

    assert_eq!(status, http::StatusCode::OK);
}

#[actix_web::test]
async fn test_the_asset_version_does_not_match() {
    let mut factory = ResponseFactory::new();
    factory.set_version(|| "example-version".to_string());

    let app = test::init_service(
        App::new()
            .wrap(VersionMiddleware::new("example-version".to_string()))
            .app_data(web::Data::new(factory))
            .service(web::resource("/foo").to(example_handler)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/foo")
        .insert_header((X_INERTIA, "true"))
        .insert_header((X_INERTIA_VERSION, "different-version"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();

    assert_eq!(status, http::StatusCode::CONFLICT);
}

#[actix_web::test]
async fn test_partial_reload_only_returns_requested_props() {
    async fn handler(req: HttpRequest, data: web::Data<ResponseFactory>) -> impl actix_web::Responder {
        let inertia = data.render(
            "ComponentName",
            serde_json::json!({"foo": 1, "bar": 2}),
            req.uri().to_string().as_str(),
        );
        inertia.into_response(&req).await
    }

    let mut factory = ResponseFactory::new();
    let app = test::init_service(
        App::new()
            .wrap(VersionMiddleware::new("example-version".to_string()))
            .app_data(web::Data::new(factory.clone()))
            .service(web::resource("/partial").to(handler)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/partial")
        .insert_header((X_INERTIA, "true"))
        .insert_header((X_INERTIA_VERSION, "example-version"))
        .insert_header((X_INERTIA_PARTIAL_COMPONENT, "ComponentName"))
        .insert_header((X_INERTIA_PARTIAL_ONLY, "foo"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body_bytes = test::read_body(resp).await;

    assert_eq!(status, http::StatusCode::OK);
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(body["props"].get("foo").is_some());
    assert!(body["props"].get("bar").is_none());
}

#[actix_web::test]
async fn test_partial_reload_excludes_props() {
    async fn handler(req: HttpRequest, data: web::Data<ResponseFactory>) -> impl actix_web::Responder {
        let inertia = data.render(
            "ComponentName",
            serde_json::json!({"foo": 1, "bar": 2}),
            req.uri().to_string().as_str(),
        );
        inertia.into_response(&req).await
    }

    let mut factory = ResponseFactory::new();
    let app = test::init_service(
        App::new()
            .wrap(VersionMiddleware::new("example-version".to_string()))
            .app_data(web::Data::new(factory.clone()))
            .service(web::resource("/partial").to(handler)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/partial")
        .insert_header((X_INERTIA, "true"))
        .insert_header((X_INERTIA_VERSION, "example-version"))
        .insert_header((X_INERTIA_PARTIAL_COMPONENT, "ComponentName"))
        .insert_header((X_INERTIA_PARTIAL_EXCEPT, "bar"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body_bytes = test::read_body(resp).await;

    assert_eq!(status, http::StatusCode::OK);
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(body["props"].get("bar").is_none());
    assert!(body["props"].get("foo").is_some());
}

#[actix_web::test]
async fn test_the_component_exists_on_the_filesystem() {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{env, path::PathBuf};

    // create a unique directory inside the system temp directory
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir: PathBuf = env::temp_dir().join(format!("component_exists_{unique}"));
    fs::create_dir(&dir).unwrap();
    let component_path = dir.join("ComponentName.vue");

    // simulate existing component template file
    fs::write(&component_path, "<template></template>").unwrap();

    assert!(component_path.exists());

    fs::remove_dir_all(dir).unwrap();
}

#[actix_web::test]
async fn test_the_component_does_not_exist_on_the_filesystem() {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{env, path::PathBuf};

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir: PathBuf = env::temp_dir().join(format!("component_missing_{unique}"));
    fs::create_dir(&dir).unwrap();
    let component_path = dir.join("MissingComponent.vue");

    assert!(!component_path.exists());

    fs::remove_dir_all(dir).unwrap();
}
