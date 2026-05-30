use rcon::{AsyncStdStream, Connection, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let address = "localhost:25575";
    let mut conn = <Connection<AsyncStdStream>>::builder()
        .enable_minecraft_quirks(true)
        .connect(address, dotenv::var("password").unwrap().as_ref())
        .await?;

    demo(&mut conn, "list").await?;
    demo(&mut conn, "say Rust lang rocks! ;P").await?;
    demo(&mut conn, "save-all").await?;
    //demo(&mut conn, "stop");
    Ok(())
}

async fn demo(conn: &mut Connection<AsyncStdStream>, cmd: &str) -> Result<(), Error> {
    let resp = conn.cmd(cmd).await?;
    println!("{}", resp);
    Ok(())
}
