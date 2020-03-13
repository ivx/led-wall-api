#[allow(clippy::all)]
extern crate rosc;

// use rand::{thread_rng, Rng};
use rand::prelude::*;

use rosc::OscPacket;
use std::env;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

use redis::Commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage {} BIND_IP:PORT", &args[0]);
    if args.len() < 2 {
        println!("{}", usage);
        ::std::process::exit(1)
    }
    let addr = match SocketAddrV4::from_str(&args[1]) {
        Ok(addr) => addr,
        _ => panic!(usage),
    };
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Listening to {}", addr);

    let mut buf = [0u8; rosc::decoder::MTU];

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("Received packet with size {} from: {}", size, addr);
                let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                let addr = handle_packet(packet);

                con.set::<&str, u8, ()>("background:color:red", 0).unwrap();
                con.set::<&str, u8, ()>("background:color:blue", 0).unwrap();
                con.set::<&str, u8, ()>("background:color:green", 0)
                    .unwrap();

                println!("packet");
                match addr.as_ref() {
                    "/area/0" => {
                        con.set::<&str, u8, ()>("area0:color:red", random())
                            .unwrap();
                        let asd: u8 = random();
                        println!("{}", asd);

                        con.set::<&str, u8, ()>("area0:color:green", random())
                            .unwrap();

                        con.set::<&str, u8, ()>("area0:color:blue", random())
                            .unwrap();
                    }
                    "/area/1" => {
                        con.set::<&str, u8, ()>("area1:color:red", random())
                            .unwrap();
                        let asd: u8 = random();
                        println!("{}", asd);

                        con.set::<&str, u8, ()>("area1:color:green", random())
                            .unwrap();

                        con.set::<&str, u8, ()>("area1:color:blue", random())
                            .unwrap();
                    }

                    "/color/blue" => {
                        con.set::<&str, u8, ()>("background:color:blue", 200)
                            .unwrap();
                    }
                    "/color/red" => {
                        con.set::<&str, u8, ()>("background:color:red", 200)
                            .unwrap();
                    }
                    "/color/green" => {
                        con.set::<&str, u8, ()>("background:color:green", 200)
                            .unwrap();
                    }
                    e => println!("{}", e),
                }
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }
}

fn handle_packet(packet: OscPacket) -> String {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            println!("OSC arguments: {:?}", msg.args);
            // let test = format!("{}", msg.addr);
            // let asd = String::from(&msg.args);
            // return test;

            println!("msg");
            msg.addr
        }
        OscPacket::Bundle(bundle) => {
            println!("OSC Bundle: {:?}", bundle);

            "".to_string()
        }
    }
}
