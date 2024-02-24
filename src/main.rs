use std::net::UdpSocket;

use clap::{Parser, Subcommand};
use rosc::{OscMessage, OscPacket, OscType};

#[derive(Debug, Clone, Subcommand)]
enum Payload {
    Int {
        value: i32,
    },
    Float {
        value: f32,
    },
    String {
        value: String,
    },
    Bool {
        #[arg(help = "0 for false, 1 for true")]
        value: u8,
    },
}

/// Send an OSC message to a remote address
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:9000")]
    remote: String,

    #[arg(short, long)]
    address: String,

    #[command(subcommand)]
    payload: Payload,
}

fn main() {
    let args = Args::parse();

    let payload = match args.payload {
        Payload::Int { value } => OscType::Int(value),
        Payload::Float { value } => OscType::Float(value),
        Payload::String { value } => OscType::String(value),
        Payload::Bool { value } => OscType::Bool(value != 0),
    };

    println!("Sending to {} with payload {:?}", &args.address, &payload);

    let message = OscMessage {
        addr: args.address,
        args: vec![payload],
    };
    let packet = OscPacket::Message(message);
    let bytes = rosc::encoder::encode(&packet).unwrap();

    let sent = UdpSocket::bind("0.0.0.0:0")
        .unwrap()
        .send_to(&bytes, &args.remote)
        .unwrap();

    println!("Sent {} bytes to {}", sent, &args.remote);
}
