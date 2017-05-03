use std::str;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

extern crate linked_hash_map;

use packet::MemPacket;
mod packet;

use cache::cache::Cache;
use cache::storage_structure::{CacheStorageStructure, NaiveStorageStructure};
use cache::replacement_policy::{CacheReplacementPolicy, LRU, Clock, LFU};
mod cache;

mod command;
mod commands;

fn handle_client(mut stream: TcpStream) {
    let mut cache: Cache<_, _> = Cache::new(360, NaiveStorageStructure::new(), LFU::new());

    loop {
        let mut buffer = [0; 128];
        let len = stream.read(&mut buffer).unwrap();

        let string = match str::from_utf8(&buffer[0..len]) {
            Ok(s) => s,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
        };
        println!("{}", string);

        match command::parse_command(string, &mut cache) {
            Some(response) => {
                println!("{:?}", response.header);
                println!("{:?}", response);

                let _ = stream.write(response.bytes().as_slice());
                let _ = stream.write(b"\r\n");
                let _ = stream.flush();
            },
            None => {},
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4321").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Established connection!");
                handle_client(stream);
            }
            Err(e) => {
                panic!("Unable to establish connection: {}", e);
            }
        }
    }
}