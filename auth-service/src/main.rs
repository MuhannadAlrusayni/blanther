use actix_web::{post, web, App, HttpServer, error, HttpResponse};
use actix_web::web::Data;
use sqlx::{Pool, Postgres};
use std::error::Error;

use database::conn::connect;
use database::user::{User, UserDB};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:password@localhost/blanther";

    let pool = connect(url).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(web::scope("/api/v1").service(create_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

#[post("/user")]
async fn create_user(pool: web::Data<Pool<Postgres>>, user: web::Json<User>) -> Result<HttpResponse, Box<dyn Error>> {
    let user = user.insert_user(pool.get_ref()).await;

    match user {
        Ok(user) => {
            let json = serde_json::to_string(&user);

            match json {
                Ok(json) => Ok(HttpResponse::Ok().json(json)),
                Err(err) => {
                    eprintln!("could not parse json: {}", err);
                    Err(Box::new(error::ErrorBadRequest("something went wrong!")))
                },
            }
        },
        Err(err) => {
            eprintln!("could not insert user: {}", err);
            Err(Box::new(error::ErrorInternalServerError("something went wrong!")))
        },
    } 
}
