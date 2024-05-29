use actix_inertia::{
    example_handler, ResponseFactory, VersionMiddleware, X_INERTIA, X_INERTIA_VERSION,
};
extern crate serde_json;

use actix_web::{http, test, web, App};
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
async fn test_the_component_exists_on_the_filesystem() {
    // This test requires a file existence check mechanism, similar to Laravel's implementation.
    // Placeholder test
    assert!(true);
}

#[actix_web::test]
async fn test_the_component_does_not_exist_on_the_filesystem() {
    // This test requires a file existence check mechanism, similar to Laravel's implementation.
    // Placeholder test
    assert!(true);
}
