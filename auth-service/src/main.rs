use std::error::Error;

use database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:password@localhost/blanther";

    let pool = database::connect(url).await?;

    println!("{:?}", pool);

    Ok(())
}
