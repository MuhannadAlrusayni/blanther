use actix_web::web::{Data, Json};
use actix_web::{error, post, web, App, HttpResponse, HttpServer};
use sqlx::{Pool, Postgres};
use std::error::Error;

use database::conn::connect;
use database::user::{User, UserDB};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    envmnt::load_file(&format!("{}/.env", env!("CARGO_MANIFEST_DIR")))?;
    let pool = connect().await?;

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
async fn create_user(
    pool: web::Data<Pool<Postgres>>,
    user: Json<User>,
) -> Result<Json<User>, Box<dyn Error>> {
    let user = user
        .insert_user(pool.get_ref())
        .await
        .map_err(|_err| "Something went wrong!")?;

    Ok(Json(user))
}
