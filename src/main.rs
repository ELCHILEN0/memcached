use std::str;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::marker::PhantomData;

use packet::{MemPacket, MemHeader};
mod packet;

use cache::cache::Cache;
use cache::storage_structure::{CacheStorageStructure, HashStorageStructure};
use cache::replacement_policy::{CacheReplacementPolicy, LRU};
mod cache;

mod command;
mod commands;

fn handle_client(mut stream: TcpStream) {
    let mut cache: Cache<HashStorageStructure, LRU> = Cache {
            capacity: 360,
            item_lifetime: 60 * 1000,
            max_key_len: 256,
            max_val_len: 512,
            storage_structure: HashStorageStructure::new(),
            replacement_policy: LRU::new(),

    };

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

                stream.write(response.bytes().as_slice());
                stream.write(b"\r\n");
                stream.flush();
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