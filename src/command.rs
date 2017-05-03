use std::borrow::Borrow;
use packet::MemPacket;

use cache::cache::Cache;
use cache::storage_structure::CacheStorageStructure;
use cache::replacement_policy::CacheReplacementPolicy;

use commands;

fn handle_command<T: CacheStorageStructure, R: CacheReplacementPolicy>(packet: MemPacket, cache: &mut Cache<T, R>) -> Option<MemPacket> {
    println!("handle_command");
    println!("{:?}", packet.header);
    println!("{:?}", packet);

    let mut response = MemPacket::new(false);

    if packet.header.magic != 0x80 {
        response.header.with_status(0x0084);
        return Some(response);
    }

    match packet.header.opcode {
        0x00 => commands::get::get_command(packet, cache),
        0x01 => commands::set::set_command(packet, cache),
        0x02 => commands::set::add_command(packet, cache),
        0x03 => commands::set::replace_command(packet, cache),
        0x04 => commands::delete::delete_command(packet, cache),
        _ => {
            response.header.with_status(0x0081);
            Some(response) 
        }
    }
}

// TODO: This will eventually be removed once a client is implemented, for now this exists for the purposes of telnet
pub fn parse_command<T: CacheStorageStructure, R: CacheReplacementPolicy>(command: &str, cache: &mut Cache<T, R>) -> Option<MemPacket> {
    let mut iter = command.split_whitespace();

    let mut extra_bytes: Vec<u8> = Vec::new();
    let mut key_bytes: Vec<u8> = Vec::new();
    let mut value_bytes: Vec<u8> = Vec::new();

    let code: u8;
    // TODO: Add param length validation? probably not if we implement a client
    match iter.next() {
        Some(cmd) => {
            match cmd.to_uppercase().borrow() {
                "GET" => {
                    code = 0x00;
                    key_bytes = Vec::from(iter.next().unwrap().as_bytes());
                },
                "SET" => {
                    code = 0x01;
                    key_bytes = Vec::from(iter.next().unwrap().as_bytes());
                    value_bytes = Vec::from(iter.next().unwrap().as_bytes());        
                },
                "ADD" => {
                    code = 0x02;
                    key_bytes = Vec::from(iter.next().unwrap().as_bytes());
                    value_bytes = Vec::from(iter.next().unwrap().as_bytes());           
                },
                "REPLACE" => {
                    code = 0x03;
                    key_bytes = Vec::from(iter.next().unwrap().as_bytes());
                    value_bytes = Vec::from(iter.next().unwrap().as_bytes());             
                },
                "DELETE" => {
                    code = 0x04;
                    key_bytes = Vec::from(iter.next().unwrap().as_bytes());          
                },
                "INCREMENT" => {
                    code = 0x05;
                    // TODO:            
                },
                "DECREMENT" => {
                    code = 0x06;
                    // TODO:            
                },
                "QUIT" => {
                    code = 0x07;
                    // TODO:            
                },
                "FLUSH" => {
                    code = 0x08;
                    // TODO:            
                },
                "GETQ" => {
                    code = 0x09;
                    // TODO:            
                },
                "NO-OP" => {
                    code = 0x0a;
                    // TODO:            
                },
                "VERSION" => {
                    code = 0x0b;
                    // TODO:            
                },
                "GETK" => {
                    code = 0x0c;
                    // TODO:            
                },
                "GETKQ" => {
                    code = 0x0d;
                    // TODO:            
                },
                "APPEND" => {
                    code = 0x0e;
                    // TODO:            
                },
                "PREPEND" => {
                    code = 0x0f;
                    // TODO:            
                },
                "STAT" => {
                    code = 0x10;
                    // TODO:            
                },
                "SETQ" => {
                    code = 0x11;
                    // TODO:            
                },
                "ADDQ" => {
                    code = 0x12;
                    // TODO:            
                },
                "REPLACEQ" => {
                    code = 0x13;
                    // TODO:            
                },
                "DELETEQ" => {
                    code = 0x14;
                    // TODO:            
                },
                "INCREMENTQ" => {
                    code = 0x15;
                    // TODO:            
                },
                "DECREMENTQ" => {
                    code = 0x16;
                    // TODO:            
                },
                "QUITQ" => {
                    code = 0x17;
                    // TODO:            
                },
                "FLUSHQ" => {
                    code = 0x18;
                    // TODO:            
                },
                "APPENDQ" => {
                    code = 0x19;
                    // TODO:            
                },
                "PREPENDQ" => {
                    code = 0x1a;
                    // TODO:            
                },
                _ => {
                    code = 0xFF;
                }
            }
        }
        None => {
            let mut response = MemPacket::new(false);
            response.header.with_status(0x0084);

            return Some(response);
        }
    }

    let mut request = MemPacket::new(true);
    request.header.with_opcode(code);
    request.with_key(String::from_utf8_lossy(key_bytes.as_slice()).into_owned());    
    request.with_extras(String::from_utf8_lossy(extra_bytes.as_slice()).into_owned());
    request.with_value(String::from_utf8_lossy(value_bytes.as_slice()).into_owned());

    handle_command(request, cache)
}