

use crate::byte_traits::{ConvertBytes};
use std::net::{UdpSocket, IpAddr, Ipv4Addr, TcpStream, TcpListener};
use std::str::FromStr;
use std::io::{Write, Read};

pub struct ServerConnection {
    // server_ip : IpAddr,
    // server_stream : TcpStream,
}

impl ServerConnection {
    pub fn new() -> Self {
        
        // let server_connect_port = 20001;
        // server_ip_ = IPAddr::new(127, 0, 0, 1);
        // if let Ok(server_str) = TcpStream::connect((server_ip_, server_connect_port)); {
        //     println!("Connected to server...");
        // } else {
        //     println!("Connection to server FAILED");
        // }
        // ServerConnection {
        //     server_ip : server_ip_,
        //     server_strean : server_str,
        // }
        ServerConnection {

        }
    }

    pub fn http_req(&self, data : String) {

    }

    pub fn get_friend_addr(&self, friend_id : String) -> (IpAddr, u16) {
        // call http req to get ip addr & tcp listener port num
        (IpAddr::V4(Ipv4Addr::new(192,168,68,114)), 1068)
    }
}





pub struct CallConnectionUDP {
    other_ip : IpAddr,
    other_port : u16,
    my_udp_port : u16,
    server_connection : ServerConnection,
    my_udp_socket : UdpSocket,
}

impl CallConnectionUDP {
    /* Sends the UDP packets */
    pub fn new(myip : IpAddr, myport : u16, oip : IpAddr, oport : u16, server_conn : ServerConnection) -> Self {
        let udpport = myport;
        let socket = UdpSocket::bind((myip, udpport)).expect("Couldn't bind to address!");
        socket.set_nonblocking(true).expect("Couldn't set to non blocking");
        socket.connect((oip, oport)).expect("Connection failedG");
        
        CallConnectionUDP {
            server_connection : server_conn,
            // need to do error checking here to ensure other person is online ->
            other_ip : oip,
            other_port : oport,
            my_udp_port : udpport,
            my_udp_socket : socket,
        }
    }

    pub fn send_data<T: ConvertBytes>(&self, packet_bytes : Vec<u8>) {
        self.my_udp_socket.send(&packet_bytes);        
    }

    pub fn recv_data<U: ConvertBytes>(&self) -> Option<Vec<u8>> {
        const packetsize : usize = 180;
        let mut buf = [0; packetsize];
        let res = self.my_udp_socket.recv_from(&mut buf);//.expect("Didn't recieve data");
        let data : Vec<u8> = Vec::from(buf);
        match res {
            Ok((num_bytes, src_addr)) => {
                return Some(data);
            },
            Err(e) => {
                return None;
            },
        }
    }
}


pub struct ConnectionTCP() {
    // my_ip : IpAddr,
    // my_recv_port : u16,
    // other_ip : IpAddr,
    // other_recv_port : u16, // what port THIS COMP sends to
    send_stream : TcpStream,
    recv_stream : TcpStream,
}

impl ConnectionTCP {
    pub fn wait_for_connection(my_ip : IpAddr, other_ip : IpAddr, my_recv_port : u16, other_recv_port : u16) -> Self {
        // binds to socket and waits for connection
        // when connection comes, returns content of request
        // "192.168.68.109:1071"
        let listener = TcpListener::bind((my_ip, my_recv_port)).expect("F");
        let recv_stream = listener.incoming().next().expect("F").expect("f");
        // let peer_addr : std::net::SocketAddr = stream.peer_addr().expect("F");
        let send_stream = TcpStream::connect((other_ip, other_recv_port)).expect("F");
        Self {
            send_stream,
            recv_stream,
        }
    }

    pub fn connect_to(my_ip : IpAddr, other_ip : IpAddr, my_recv_port : u16, other_port : u16) -> Self {
        let mut send_stream = TcpStream::connect((other_ip, other_port)).expect("F");
        // let local_addr : std::net::SocketAddr = stream.local_addr().expect("F");
        let listener = TcpListener::bind((my_ip, my_recv_port)).expect("F");
        let recv_stream = listener.incoming().next().expect("F").expect("f");
        Self {
            send_stream,
            recv_stream,
        }
    }

    pub fn send(&self, data : String) {
        self.send_stream.write(data.as_bytes());
    }

    pub fn recv(&self) -> String {
        let mut data = String::new();
        self.recv_stream.read_to_string(&mut data);
        data
    }
}



pub fn parse_call_req(req : String) -> (u8, u16) {
    println!("req: {req}");
    let i = req.find(':').expect("F");
    let j = req.find(' ').expect("F");
    let end = String::from(&req[j..]);
    let k = end.find(':').expect("F");
    let other_type : u8 = u8::from_str(&req[(i+1)..j]).expect("F");
    let other_udp_port : u16 = u16::from_str(&end[(k+1)..]).expect("F");
    (other_type, other_udp_port)
}


pub fn write_call_req(my_sample_type : u8, my_udp_port : u16) -> String {
    let mut req = String::from("type:");
    req.push_str(&my_sample_type.to_string());
    req.push_str(" port:");
    req.push_str(&my_udp_port.to_string());
    req
}


pub fn start_call(my_sample_type : u8, other_ip : IpAddr, other_tcp_port : u16, my_ip : IpAddr, my_udp_port : u16, server_conn : ServerConnection) -> CallConnectionUDP {
    println!("Press any key to start call");
    std::io::stdin().read_line(&mut String::new());
    let mut stream = TcpStream::connect("192.168.68.114:1068").expect("failed to open tcp stream to other computer");
    println!("tcp connected!");
    let req = write_call_req(my_sample_type, my_udp_port);
    stream.write(req.as_bytes());
    println!("sent req {req}");
    stream.shutdown(std::net::Shutdown::Both).expect("F");
    let listenerTCP = TcpListener::bind("192.168.68.109:1101").expect("failed to bind to tcp");
    let mut stream = listenerTCP.incoming().next().expect("f").expect("F");
    let mut resp = String::new();
    stream.read_to_string(&mut resp).expect("failed to read rcp req");
    println!("response: {resp}");
    // loop {}
    let (other_type, other_udp_port) = parse_call_req(resp);
    // let other_type = 1;
    // let other_udp_port = 1;
    let call_connection = CallConnectionUDP::new(my_ip, my_udp_port, other_ip, other_udp_port, server_conn);
    return call_connection
}   


pub fn wait_for_call(other_ip : IpAddr, other_tcp_port : u16, my_ip : IpAddr, mut my_tcp_port : u16, my_udp_port : u16, server_conn : ServerConnection) -> CallConnectionUDP {
    println!("Waiting for incoming call");
    let mut stream : TcpStream;
    let listenerTCP = TcpListener::bind((my_ip, my_tcp_port)).expect("failed to bind to tcp port");
    stream = listenerTCP.incoming().next().expect("f").expect("F");
    stream.set_nonblocking(false).expect("f");

    // loop {
    //     if let Ok(listenerTCP) = TcpListener::bind((my_ip, my_tcp_port)) {
    //         stream = listenerTCP.incoming().next().expect("f").expect("F");       
    //         break;
    //     } 
    //     my_tcp_port += 1;
    // }
    let mut req = String::new();
    stream.read_to_string(&mut req).expect("f");
    let (other_sample_type, other_udp_port) = parse_call_req(req);
    let my_sample_type : u8 = 0;
    let call_connection = CallConnectionUDP::new(my_ip, my_udp_port, other_ip, other_udp_port, server_conn);
    call_connection
}