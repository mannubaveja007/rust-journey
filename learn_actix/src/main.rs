use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};


#[get("/")]
async fn hello() -> impl  Responder {
    HttpResponse::Ok().body("Kehh haal haiii 👀")
}
fn main() {
    println!("Hello, world!");
}
