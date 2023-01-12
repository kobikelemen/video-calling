use std::net::{UdpSocket, IpAddr, Ipv4Addr, TcpListener, TcpStream};

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
    // println!("EQUAL!");
    
}


fn handle_client(stream: TcpStream) {
    println!("Connected!");
    println!("other address: {}", stream.peer_addr().expect("fail"));
}

fn recv_tcp() {
    let listener = TcpListener::bind("192.168.68.109:1071");
    println!("binded successfully");    
    for stream in listener.expect("FAIL1").incoming() {
        handle_client(stream.expect("Failed connection"));
    }
}

fn send_tcp() {
    // let stream = TcpStream::connect("192.168.68.114:1071").expect("failed");
    let stream = TcpStream::connect("192.168.68.114:1071").expect("failed");
    println!("connected!");

    // if let Ok(stream) = TcpStream::connect("192.168.68.114:1071") {
    //     println!("connected!");
    // } else {
    //     println!("Failed");
    // }
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



fn main()
{
    // send_tcp(); 
    send_udp();
}