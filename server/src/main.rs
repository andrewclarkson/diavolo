use std::error::Error;

use tokio::prelude::*;
use tokio::net::TcpListener;

use diavolo_core::ADDRESS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut listener = TcpListener::bind(ADDRESS).await?;

    loop {
	let (mut socket, _) = listener.accept().await?;

	tokio::spawn(async move {
	    let mut buf = [0; 1024];
	    
	    let n = socket.read(&mut buf)
		.await
		.expect("bum bum bummmm!!");

	    if n > 0 {
		socket.write_all(&buf[0..n])
		    .await
		    .expect("bum bum bummm!! 2");
	    }
	    
	});
    }
}
