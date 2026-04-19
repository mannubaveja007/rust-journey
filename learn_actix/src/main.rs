use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Kehh haal haiii 👀")
}

#[post("/echo")]
async fn echo(request_string: String) -> impl Responder {
    HttpResponse::Ok().body(request_string)
}
async fn manual() -> impl Responder {
    HttpResponse::Ok().body("Hello, World from Actix🦀")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Welcome to the Actix Server!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
