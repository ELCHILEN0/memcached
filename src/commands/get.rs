use packet::{MemPacket, MemHeader};
use command;

use cache::cache::Cache;
use cache::key::Key;
use cache::value::Value;
use cache::storage_structure::CacheStorageStructure;
use cache::replacement_policy::CacheReplacementPolicy;

pub fn get_command<T: CacheStorageStructure, R: CacheReplacementPolicy>(request: MemPacket, cache: &mut Cache<T, R>) -> Option<MemPacket> {
    println!("get_command");

    let mut response = MemPacket::new(false);
    response.header.with_opcode(request.header.opcode);

    if request.has_extras() || request.has_value() {
        response.header.with_status(0x0004);
        return Some(response) ;
    }

    let key_bytes = request.key.clone().into_bytes();
    let extra_bytes = vec![0; 0];
    
    
    response.with_key(String::from_utf8_lossy(key_bytes.as_slice()).into_owned());
    response.with_extras(String::from_utf8_lossy(extra_bytes.as_slice()).into_owned());

    match cache.get(Key::new(request.key)) {
        Some(value) => {
            response.with_value(value.value.item.clone());
        },
        None => {
            response.header.with_status(0x0001);
            response.with_value(String::from("Not found").clone());
        }
    };
    
    Some(response)
}