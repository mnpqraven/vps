use limbo::Builder;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: dir
    let db = Builder::new_local("sqlite.db").build().await?;
    let conn = db.connect()?;

    let res = conn.query("SELECT * FROM users", ()).await?;
    println!("Hello, world!");

    Ok(())
}
