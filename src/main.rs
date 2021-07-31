use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Responder};

// #[get("/")]
async fn index() -> impl Responder
{
    "Ola mundo".to_owned()
}

fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("/app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    );

    cfg.service(
        web::resource("/api")
            .route(web::get().to(|| HttpResponse::Ok().body("/api")))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(|| 
    {
        App::new()
            .service(
                web::scope("/")
                    .configure(config)
                    .route("/welcome", web::get().to(index))
            )
            // .configure(f: F)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}