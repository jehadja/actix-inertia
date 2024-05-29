 # Actix-Inertia

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jehadja/actix-inertia/blob/main/LICENSE)

Actix-Inertia is a Rust library that integrates Inertia.js with the Actix web framework. It enables you to build modern single-page applications (SPAs) using server-side routing and controllers.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Inertia.js allows you to build modern SPAs without the complexity of client-side routers. Actix-Inertia provides seamless integration with Actix, allowing you to use Inertia.js with Rust.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
actix-inertia = "0.1.0"
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage

### Setting up the Server

Create a file `main.rs` with the following content:

```rust
use actix_inertia::{ResponseFactory, VersionMiddleware, example_handler};
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let response_factory = ResponseFactory::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(response_factory.clone()))
            .wrap(VersionMiddleware::new("1.0".to_string()))
            .route("/example", web::get().to(example_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Middleware

To use the version middleware, include it in your Actix app setup as shown above. This ensures that requests are properly handled according to the Inertia.js versioning mechanism.

### Example

An example handler that uses Inertia:

```rust
use actix_inertia::{InertiaResponder, VersionMiddleware};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ExampleProps {
    key: String,
}

async fn example_handler(req: HttpRequest) -> impl Responder {
    let props = ExampleProps {
        key: "value".to_string(),
    };
    InertiaResponder::new("ExampleComponent", props).respond_to(&req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(VersionMiddleware::new("1.0".to_string()))
            .route("/example", web::get().to(example_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Contributing

Contributions are welcome! Please see the [contributing guidelines](CONTRIBUTING.md) for more details.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/jehadja/actix-inertia/blob/main/LICENSE) file for details.
 