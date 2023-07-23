use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use env_logger::Env;
use routes::email::post_trigger_otp;
mod mail;
mod routes;
use crate::routes::email::trigger_otp;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let api_key: String = env::var("MAILSLURP_API_KEY").expect("$USER is not set");
    // println!("Got api key: {}", api_key);
    // middleware setup
    // mail::check(String::from("vawadil993@rc3s.com"));

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // info!("can log from the test too");
    HttpServer::new(|| {
        let cors = Cors::default().supports_credentials();
        App::new()
            .service(routes::health_check)
            .service(
                web::scope("/email")
                    .service(post_trigger_otp)
                    .service(trigger_otp),
            )
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed),
            )
            .wrap(cors)
            .wrap(Logger::new("%a %t '%r' %s %b '%{Referer}i'  %T"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// .wrap(Logger::new("%a %{User-Agent}i"));
