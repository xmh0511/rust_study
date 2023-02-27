mod async_tcpstream;

use async_tcpstream::{AsyncTcp};
#[tokio::main]
async fn main() {
    // let mut conn = TcpStream::connect("127.0.0.1:8080").unwrap();
    // let write_content = b"GET / HTTP/1.1\r\n\r\n";
    // let mut vec = Vec::new();
    // vec.resize(1024 * 1024, b'\0');
    // conn.write(write_content).unwrap();
    // let read_size = conn.read(&mut vec[..]).unwrap();
    // println!("{read_size}");
    let conn = AsyncTcp::new("127.0.0.1:8080");
    let r = conn.write(b"GET / HTTP/1.1\r\n\r\n").await;
    println!("result: {r:?}");
    let r1 = conn.read().await;
    println!("{r1:?}");
}
