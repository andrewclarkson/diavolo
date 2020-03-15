use diavolo_core::ADDRESS;

use tokio::prelude::*;
use std::str;
use std::error::Error;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(ADDRESS).await?;
    let _ = stream.write(&"Hello!!".as_bytes()).await?;
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).await?;
    let result = str::from_utf8(&buf[0..n])?;
    println!("Got '{}' from server", result);
    Ok(())
}

