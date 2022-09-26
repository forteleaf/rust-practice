// 열거형 정의하기
//#![allow(unused)]
//fn main() {
//    //   let four = IpAddrKind::V4;
//    //  let six = IpAddrKind::V6;
//    //    let home = IpAddr {
//    //        kind: IpAddrKind::V4,
//    //        address: String::from("127.0.0.1"),
//    //    };
//    //    let loopback = IpAddr {
//    //        kind: IpAddrKind::V6,
//    //        address: String::from("::1"),
//    //    };
//    //
//    let home = IpAddr::V4(String::from("127.0.0.1"));
//    let loopback = IpAddr::V6(String::from("::1"));
//}
//fn route(ip_type: IpAddrKind) {}

//enum IpAddrKind {
//    V4,
//    V6,
//}
//
//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

#![allow(unused)]
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    enum Message {
        Quit,                       //연관된 데이터 없음
        Move { x: i32, y: i32 },    // 익명 구조체
        Write(String),              // 하나의 String
        ChangeColor(i32, i32, i32), // 세개의 i32
    }

    impl Message {
        fn call(&self) {
            // method context
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
}
