mod client;
mod byte_traits;

use std::net::{UdpSocket, IpAddr, Ipv4Addr, TcpListener, TcpStream};
use std::io::{Write,Read};


fn u32_to_binary(mut i : u32) -> [u8; 32] {
    let mut res : [u8; 32] = [0; 32];
    let mut x : u32 = 31;
    while x >= 0 {
        if 2_u32.pow(x) <= i {
            res[(31-x) as usize] = 1;
            i -= 2_u32.pow(x);
        }
        if x == 0 {
            break;
        }
        x -= 1;
    }
    return res;
}


fn float_to_binary()
{
    let f = 12.5f32;    
    let x = f.to_ne_bytes();
    let y = u32_to_binary(f.to_bits());
    print!("y: ");
    for i in 0..32 {
        print!("{}", y[i])
    }

    /* MUST CONSIDER IF PLATFORM IS BIG OR SMALL ENDIAN!! */
    let mut z : [u8; 4] = [0; 4];
    let mut j = y.len()-1;
    for i in 0..4 {
        let mut s = 0;
        for p in 0..8 {
            if y[j] == 1 {
                s += 2_u8.pow(p);
            }
            if j == 0 {
                break;
            }
            j -= 1;
        }
        z[i] = s;
    }
    println!("x");
    for n in 0..4 {
        println!("{}", x[n]);
    }
    println!("z");
    for n in 0..4 {
        println!("{}", z[n]);
    }
}


fn handle_client(mut stream: TcpStream) {
    println!("Connected!");
    println!("other address: {}", stream.peer_addr().expect("fail"));
    let mut s = String::new();
    stream.read_to_string(&mut s);
    println!("recved: {s}");
}


fn recv_tcp() {
    let mut listener = TcpListener::bind("192.168.68.109:1071");
    println!("binded successfully");    
    for stream in listener.expect("FAIL1").incoming() {
        handle_client(stream.expect("Failed connection"));
    }
}


fn send_tcp() {
    // let stream = TcpStream::connect("192.168.68.114:1071").expect("failed");
    let mut stream = TcpStream::connect("192.168.68.114:1071").expect("failed");
    println!("connected!");
    let s = String::from("hello bish");
    stream.write(s.as_bytes());
    println!("finished writing");
}


fn recv_udp() {
    let socket = UdpSocket::bind("192.168.68.109:1071").expect("couldn't bind");
    let mut buf = [0;10];
    match socket.recv(&mut buf) {
        Ok(received) => println!("received {received} bytes {:?}", &buf[..received]),
        Err(e) => println!("recv function failed: {e:?}"),
    }
}


fn send_udp() {
    let socket = UdpSocket::bind("192.168.68.109:1071").expect("couldn't bind");
    socket.send_to(&[0;10], "192.168.68.114:1071").expect("Coudn't send data");
}


fn test_req() {
    let bytes = client::write_call_req(69,420);
    println!("{:?}", bytes);
    let (typ, port) = client::parse_call_req(bytes);
    println!("{typ} {port}");
}


fn h(mut stream : TcpStream) {
    println!("incomong connection from: {}", stream.peer_addr().expect("F"));
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).expect("f");
        println!("receieved");
        stream.write(&buf[..bytes_read]).expect("f");
        println!("sent");
    }
}


fn server() {
    let listener = TcpListener::bind("192.168.68.109:1071").expect("F");
    for stream in listener.incoming() {
        h(stream.expect("F"));
    }
}


fn server_ConnectionTCP() {
    let my_ip = IpAddr::V4(Ipv4Addr::new(192,168,68,109));
    let other_ip = IpAddr::V4(Ipv4Addr::new(192,168,68,114));
    let my_recv_port = 1070;
    let other_recv_port = 1070;
    let mut connection = client::ConnectionTCP::wait_for_connection(my_ip, my_recv_port);
    let data = connection.recv();
    println!("recieved: {data}");
    connection.send(String::from("hello bish"));
    println!("data sent");
}


fn client_ConnectionTCP() {
    let my_ip = IpAddr::V4(Ipv4Addr::new(192,168,68,114));
    let other_ip = IpAddr::V4(Ipv4Addr::new(192,168,68,109));
    let my_recv_port = 1070;
    let other_recv_port = 1070;
    let mut connection = client::ConnectionTCP::connect_to(other_ip, other_recv_port);
    connection.send(String::from("yoooo"));
    println!("data sent");
    let data = connection.recv();
    println!("recieved: {data}");
}


fn main()
{
    server_ConnectionTCP();
    loop{}
}