use actix_files as fs;
use actix_web::{App, HttpServer};

use bacon_sci::integrate::integrate;
use nalgebra::{VectorN, U1};

fn fresnel_S(t: f64, error: f64) -> Result<f64, String> {
    integrate(0.0, t, |x: f64| x.powi(2).sin(), error)
}

fn fresnel_C(t: f64, error: f64) -> Result<f64, String> {
    integrate(0.0, t, |x: f64| {x.powi(2) / (x.powi(2) - 5f64 * x + 6f64)}, error)
}

fn matching(value: String)
{
    println!("{:?}", value.split_inclusive('^').collect::<Vec<_>>());
    // for (i, j) in value.chars().enumerate()
    // {
    //     if j == '^'
    //     {
    //         println!("{:?}", value.chars().collect::<Vec<char>>()[i + 1])
    //     }
    //     // println!("{:?}", j);
    // }
    // println!("{:?}", value.chars().collect::<Vec<char>>());
}

// #[actix_web::main]
fn main() {
    println!("{:?}", fresnel_C(5f64, 100000f64));
    matching("10^2^3".to_owned());
    // HttpServer::new(|| {
    //     App::new()
    //         .service(fs::Files::new("/", "./static"))
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await
}