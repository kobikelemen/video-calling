// mod audio;

use crate::audio::AudioPacket;
use std::net::{UdpSocket, TcpStream, IpAddr, Ipv4Addr, Ipv6Addr};
use byteorder::ByteOrder;
use byteorder::LittleEndian;

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
        // call http req to get ip addr & port num
        (IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 1069)
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
        CallConnection {
            server_connection : server_conn,
            // need to do error checking here to ensure other person is online ->
            other_ip : oip,
            other_port : oport,
            my_udp_port : udpport,
            my_udp_socket : UdpSocket::bind(("127.0.0.1", udpport)).expect("Couldn't bind to address!"),
        }
        
    }

    pub fn send_data(&self, packet : AudioPacket) {
        
        // println!("Packet sent: ");
        // for x in packet.samples {
        //     println!("{}", x);
        // }
        
        let mut buf : Vec<u8> = Vec::new();
        for i in 0..data.len() {
            let byte_arr : [u8; 4] = packet.samples[i].to_ne_bytes();
            for byte in byte_arr {
                buf.push(byte);
            }
        }
        self.my_udp_socket.send_to(&buf, (self.other_ip, self.other_port)).expect("Couldn't send data!");
        
    }

    pub fn recv_data(&self) -> Option<AudioPacket> {

        
        let mut buf = [0; 300];
        let res = self.my_udp_socket.recv_from(&mut buf);//.expect("Didn't recieve data");
        match res {
            Ok((num_bytes, src_addr)) => return Some(AudioPacket::new_from_bytes(Vec::from(buf[0..num_bytes]))),
            Err(e) => return None,
        }
        

        // return Some(AudioPacket::new(69, 69, Vec::from([0.3;160])));
    }
}