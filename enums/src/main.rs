enum IpAddrKind {
    V4,
    V6,
}

struct IpAddress {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let _localhost = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
}
