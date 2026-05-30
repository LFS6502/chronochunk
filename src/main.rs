#![allow(unused)]
use parse_duration::parse;
use rcon::{AsyncStdStream, Connection, Error};
use std::{any::type_name_of_val, str::FromStr, time::Duration};
use tokio::time::sleep;

#[derive(Debug, Clone, Copy, PartialEq)]
struct ServerStatus {
    uptime: Duration,
    tps: f32,
    mem_use: i32,
    mem_alloc: i32,
    mem_max: i32,
    loaded_chunks: i32,
}

impl FromStr for ServerStatus {
    type Err = (); //TODO: MAKE IT HAVE ERROR

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.trim().split_once('|').unwrap();
        let uptime_str = left.split_once("Uptime:").unwrap().1;
        let duration = parse(uptime_str).unwrap();
        let mut numbers = parse_numbers(right);
        Ok(Self {
            uptime: duration,
            tps: numbers.next().unwrap(),
            mem_use: numbers.next().unwrap() as i32,
            mem_alloc: numbers.next().unwrap() as i32,
            mem_max: numbers.next().unwrap() as i32,
            loaded_chunks: numbers.next().unwrap() as i32,
        })
    }
}

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
        let output = conn.cmd("gc").await.unwrap();
        let status = output.parse::<ServerStatus>().unwrap();
        dbg!(status);
        sleep(Duration::from_hours(24)).await;
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

fn parse_numbers(input: &str) -> impl Iterator<Item = f32> {
    input
        .trim()
        .split(|c: char| c.is_whitespace() || c == '/')
        .map(|word| {
            word.chars()
                .filter(|c| c.is_digit(10) || c == &'.')
                .collect::<String>()
        })
        .filter_map(|item| item.parse::<f32>().ok())
}
