use std::net::TcpListener;
use std::net::TcpStream;

fn handle(_stream: TcpStream) {
    println!("Rustika connected");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream);
   }

}
