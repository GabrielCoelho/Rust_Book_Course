enum IpAddrKind {
    V4(String),
    V6(String),
}

//struct IpAddress {
//  kind: IpAddrKind,
//address: String,
//}

fn main() {
    let _localhost = IpAddrKind::V4(String::from("127.0.0.1"));
}
