use actix_files as fs;
use actix_web::{App, HttpServer, web, HttpResponse, Responder, get, http::header::ContentType};

use bacon_sci::integrate::integrate;

fn fresnel_S(t: f64, error: f64) -> Result<f64, String> {
    integrate(0.0, t, |x: f64| x.sqrt(), error)
}

fn fresnel_C(t: f64, error: f64) -> Result<f64, String> {
    integrate(0.0, t, |x: f64| {x.powi(2) / (x.powi(2) - 5f64 * x + 6f64)}, error)
}

#[get("/math")]
async fn math() -> impl Responder {
    HttpResponse::Ok()
        .header("content-type", "text/html; charset=utf-8")
        .body(format!("
        <h1>Integration from server Actix</h1><br>
        $$\\i ntegration$$
        $$\\int{{ x \\over x^2 - 5x + 6}}dx$$
        <script src=\"https://polyfill.io/v3/polyfill.min.js?features=es6\"></script>
        <script id=\"MathJax-script\" async src=\"https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js\"></script>
        {:?}", fresnel_C(5f64, 100000f64).unwrap().to_string()))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{:?}", fresnel_S(2f64, 100f64));
    HttpServer::new(|| {
        App::new()
            .service(math)
            .service(fs::Files::new("/", "./static/index/dist").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}