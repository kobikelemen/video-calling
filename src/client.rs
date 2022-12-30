

use crate::audio::{AudioPacket};
use crate::byte_traits::{ConvertBytes};
use std::net::{UdpSocket, IpAddr, Ipv4Addr};

pub struct ServerConnection {
    // server_ip : IpAddr,
    // server_stream : TcpStream,
}

impl ServerConnection {
    pub fn new() -> Self {
        
        /*
        let server_connect_port = 20001;
        server_ip_ = IPAddr::new(127, 0, 0, 1);
        if let Ok(server_str) = TcpStream::connect((server_ip_, server_connect_port)); {
            println!("Connected to server...");
        } else {
            println!("Connection to server FAILED");
        }
        ServerConnection {
            server_ip : server_ip_,
            server_strean : server_str,
        }
        */


        ServerConnection {

        }
    }

    pub fn http_req(&self, data : String) {

    }

    pub fn get_friend_addr(&self, friend_id : String) -> (IpAddr, u16) {
        // call http req to get ip addr & port num
        (IpAddr::V4(Ipv4Addr::new()), 1068)
    }
}




pub struct CallConnection {
    other_ip : IpAddr,
    other_port : u16,
    my_udp_port : u16,
    server_connection : ServerConnection,
    my_udp_socket : UdpSocket,
}

impl CallConnection {
    pub fn new(friend_id : String, server_conn : ServerConnection, port : u16) -> Self {
        let (oip, oport) = server_conn.get_friend_addr(friend_id);
        let udpport = port;
        let socket = UdpSocket::bind(("", udpport)).expect("Couldn't bind to address!");
        socket.set_nonblocking(true).expect("Couldn't set to non blocking");
        socket.connect((oip, oport)).expect("Connection failed");
        CallConnection {
            server_connection : server_conn,
            // need to do error checking here to ensure other person is online ->
            other_ip : oip,
            other_port : oport,
            my_udp_port : udpport,
            my_udp_socket : socket,
        }
        
    }

    pub fn send_data<T: ConvertBytes>(&self, packet_bytes : Vec<u8>) 
    {
        
        // println!("Packet sent: ");
        // for x in packet.samples {
        //     println!("{}", x);
        // }
        
        // let mut buf : Vec<u8> = Vec::new();
        // for i in 0..packet.samples.len() {
        //     let byte_arr : [u8; 4] = packet.samples[i].to_ne_bytes().try_into().expect("FAILED");
        //     for byte in byte_arr {
        //         buf.push(byte);
        //     }
        // }


        self.my_udp_socket.send(&packet_bytes);
        // self.my_udp_socket.send_to(&packet_bytes, (self.other_ip, self.other_port)).expect("Couldn't send data!");
        
    }

    pub fn recv_data<U: ConvertBytes>(&self) -> Option<Vec<u8>> {
        const packetsize : usize = 180;
        let mut buf = [0; packetsize];
        println!("before self.my_upd_socket.recv_from()");
        let res = self.my_udp_socket.recv(&mut buf);//.expect("Didn't recieve data");
        println!("after self.my_upd_socket.recv_from()");
        let data : Vec<u8> = Vec::from(buf);
        match res {
            // Ok((num_bytes, src_addr)) => return Some(U.from_ne_bytes::<U>(&buf[0..packetsize])),
            Ok(num_bytes) => {
                println!("Recieved data!");
                return Some(data)
            },
            Err(e) => {
                println!("Didn't receive");
                return None
            },
        }
    }
}