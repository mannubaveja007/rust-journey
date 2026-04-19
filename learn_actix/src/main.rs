use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};


#[get("/")]
async fn hello() -> impl  Responder {
    HttpResponse::Ok().body("Kehh haal haiii 👀")
}

#[post("/echo")]
async fn echo(request_string : String) -> impl  Responder {
    HttpResponse::Ok().body(request_string)
}
fn main() {
    println!("Hello, world!");
}
