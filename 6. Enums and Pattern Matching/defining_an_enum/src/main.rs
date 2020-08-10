fn main() {
    enum IpAddreKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddreKind,
        address: String,
    }
    
    let home = IpAddr {
        kind: IpAddreKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddreKind::V6,
        address: String::from("::1"),
    };
}
