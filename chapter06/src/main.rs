enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let ip1: String = get_loopback(IpAddrKind::V4);
    println!("{}", ip1);

    let ip2: IpAddr = get_ip(Some(IpAddrKind::V6));
    println!("{:?}", ip2);

    let ip3: IpAddr = get_ip(None);
    println!("{:?}", ip3);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("Error");
    }
}

fn get_loopback(kind: IpAddrKind) -> String {
    match kind {
        IpAddrKind::V4 => { String::from("127.0.0.1") }
        IpAddrKind::V6 => { String::from("::1") }
    }
}

fn get_ip(kind: Option<IpAddrKind>) -> IpAddr {
    match kind {
        None => { IpAddr::V4(127, 0, 0, 1) }
        Some(_) => { IpAddr::V6(String::from("::1")) }
    }
}
