use actix_web::{
    body::BoxBody, get, http::header::ContentType, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};

use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/index")]
async fn index() -> impl Responder {
    MyObj { name: "user" }
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/")]
async fn static_index(_req: HttpRequest) -> impl Responder {
    web::Bytes::from_static(b"Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(static_index)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
