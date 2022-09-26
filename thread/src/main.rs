use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // bind함수는 new함수처럼 동작
    // TcpListener의 새 인스턴스를 반환
    // 네트워크 관련에서 포트를 수신대기하는 과정을 포트를 바인딩한다 라고 부른다.
    //

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Request {}", String::from_utf8_lossy(&buffer[..]));
}
