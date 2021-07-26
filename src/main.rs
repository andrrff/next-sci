use actix_web::{
    error, get, post, web, guard, middleware, App, Error, HttpRequest, HttpResponse,
    HttpServer, Result,Responder
};
use std::{env, io};
use std::sync::Mutex;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// async fn process_scope() -> impl Responder
// {
//     "Hello World"
// }

// struct AppState
// {
//     app_name: String
// }

// struct AppMutexData
// {
//     app_number: Mutex<i32>
// }

// #[get("/state")]
// async fn test_state(data: web::Data<AppState>) -> String
// {
//     let app_name = &data.app_name;
//     format!("Hello {} 🥰", app_name)
// }

// #[get("/stateMutex")]
// async fn test_state_mutex(data: web::Data<AppMutexData>) -> String
// {
//     let mut app_number = data.app_number.lock().unwrap();
//     *app_number += 1;
//     format!("App_number: {}", app_number)
// }

// #[actix_web::main]
// async fn main() -> io::Result<()> {
//     env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
//     env_logger::init();
//     let number = web::Data::new(AppMutexData{
//         app_number: Mutex::new(0)
//     });

//     HttpServer::new(move || {
//         App::new()
//             .data(
//                 AppState{
//                     app_name: "André".to_string()
//                 }
//             )
//             .service(test_state_mutex)
//             .app_data(number.clone())
//             .service(test_state)
//             .service(web::scope("/app")
//                         .route("/hello", web::get().to(process_scope))
//                     )
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test")))
            .route(web::get().to(|| HttpResponse::MethodNotAllowed()))
    );
}

struct Example
{
    value: Mutex<i32>
}

async fn counterExample(data: web::Data<Example>) -> String
{
    let mut counter = data.value.lock().unwrap();
    *counter += 1;

    format!("Counter: {}", counter)
}

#[get("/index")]
async fn index() -> impl Responder
{
    HttpResponse::Ok().body("/index")
}

#[post("/postman")]
async fn postIndex(req_body: String) -> impl Responder
{
    HttpResponse::Ok().body("req_body")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let counter = web::Data::new(
        Example
        {
            value: Mutex::new(0)
        }
    );

    HttpServer::new(move || {
        App::new()
            .route("/ola-mundo", web::to(|| HttpResponse::Ok().body("ola mundo")))
            // .service(
            //     web::resource("/tester")
            //         .route(web::get().to(|| HttpResponse::Ok().body("tester")))
            // )
            // .service(postIndex)
            .route("/post", web::post().to(|| HttpResponse::Ok().body("gravou")))
            .service(index)
            .app_data(counter.clone())
            .route("/", web::get().to(counterExample))
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
            .configure(config)
            .route("/", web::to(|| HttpResponse::Ok().body("/")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}