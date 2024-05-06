use anyhow::Result;
use std::{io, net::SocketAddr};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("listener address: {:?}", addr);
    loop {
        let (socket, raddr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process_socket_conn(socket, raddr).await {
                warn!("Error processing conn with {}: {:?}", raddr, e);
            }
        });
    }
}
async fn process_socket_conn(mut socket: TcpStream, raddr: SocketAddr) -> Result<()> {
    info!("receive remote socket: {:?}", raddr);
    loop {
        socket.readable().await?;
        let mut buff = Vec::with_capacity(1024);
        match socket.try_read_buf(&mut buff) {
            // socket is closed
            Ok(0) => break,
            Ok(n) => {
                info!("read byte size:{}", n);
                let line = String::from_utf8_lossy(&buff);
                info!("line content:{:?}", line);
                socket.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                warn!("failed to read from socket; err = {:?}", e);
                continue;
            }
        };
    }
    warn!("connected to socket is already connected");
    Ok(())
}
