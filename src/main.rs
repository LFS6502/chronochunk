#![allow(unused)]
use std::any::type_name_of_val;

use rcon::{AsyncStdStream, Connection, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let address = "localhost:25575";
    let mut conn = <Connection<AsyncStdStream>>::builder()
        .enable_minecraft_quirks(true)
        .connect(address, dotenv::var("password").unwrap().as_ref())
        .await?;

    //main check loop
    loop {
        let online = conn.cmd("list").await.unwrap();
        let mut online = online
            .trim()
            .split_whitespace()
            .filter_map(|item| item.parse::<i32>().ok());
        println!("{}", online.next().unwrap());
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        talk(&mut conn, "hello world :3").await;
    }
    Ok(())
}
async fn demo(conn: &mut Connection<AsyncStdStream>, cmd: &str) -> Result<(), Error> {
    let resp = conn.cmd(cmd).await?;
    println!("{}", resp);
    Ok(())
}

async fn talk(conn: &mut Connection<AsyncStdStream>, text: &str) {
    let cmd =
        format!(r#"tellraw @a ["",{{"text":"[Spitfire] ","color":"red"}},{{"text":"{text}"}}]"#);
    conn.cmd(&cmd).await.unwrap();
}
