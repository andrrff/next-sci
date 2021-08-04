use actix_files as fs;
use actix_web::{App, HttpServer, web, HttpResponse, Responder, get};

use bacon_sci::integrate::integrate;

fn fresnel_S(t: f64, error: f64) -> Result<f64, String> {
    integrate(0.0, t, |x: f64| x.powi(2).sin(), error)
}

fn fresnel_C(t: f64, error: f64) -> Result<f64, String> {
    integrate(0.0, t, |x: f64| {x.powi(2) / (x.powi(2) - 5f64 * x + 6f64)}, error)
}

#[get("/math")]
async fn math() -> impl Responder {
    HttpResponse::Ok().body(fresnel_C(5f64, 100000f64).unwrap().to_string())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./static/index/dist").index_file("index.html"))
            // .service(math)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}