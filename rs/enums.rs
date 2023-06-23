use IpAddr::V4; // so we can use V4 directly without `IpAddr::` prefix

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // enums can contain values
    V6(String),
}

fn main() {
    let localhost = V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    route(localhost);
    route(loopback);
}

fn route(ip: IpAddr) {
    println!("route {:?}", ip)
}
