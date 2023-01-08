use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::tcp::{ReadHalf, WriteHalf},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    let (tx, rx) = broadcast::channel::<(String, SocketAddr)>(10);
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut buf_reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = buf_reader.read_line(&mut line) =>{
                        if result.unwrap() == 0{
                            break;
                        }

                        tx.send((line.clone(),addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() =>{
                        let (msg,other_addr) = result.unwrap();

                       if addr != other_addr{
                        writer.write_all(msg.as_bytes()).await.unwrap();
                       }
                    }
                }

                // let bytes_read = .await.unwrap();
                // if bytes_read == 0 {
                //     break;
                // }
                // tx.send(line.clone()).unwrap();
                // let msg = .await.unwrap();
            }
        });
    }

    // let socket = UdpSocket::bind("0.0.0.0:8000").await.unwrap();
    // let mut buf = [0u8; 1024];
    // loop {
    //     let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
    //     println!("{:?} bytes received from {:?}", len, addr);

    //     let len = socket.send_to(&mut buf, addr).await.unwrap();
    //     println!("{:?} bytes sent", len);
    // }
}
