use actix_web::{web, App, HttpResponse, HttpServer};

fn test(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("Deu certo a rota `/test`")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
        // web::scope("/scope")
        //     .configure(test)
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(test)
            .service(
                web::scope("/api")
                    .route("/", web::get().to(|| HttpResponse::Ok().body("/api")))
                    .configure(test)
                    .service(
                        web::scope("/route")
                            .route("/", web::get().to(|| HttpResponse::Ok().body("/route")))
                            .configure(test),
                    ),
            )
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
