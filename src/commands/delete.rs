use packet::{MemPacket, MemHeader};
use command;

use cache::cache::Cache;
use cache::key::Key;
use cache::value::Value;
use cache::storage_structure::CacheStorageStructure;
use cache::replacement_policy::CacheReplacementPolicy;

pub fn delete_command<T: CacheStorageStructure, R: CacheReplacementPolicy>(request: MemPacket, cache: &mut Cache<T, R>) -> Option<MemPacket> {
    println!("delete_command");

    let mut response = MemPacket::new(false);
    response.header.with_opcode(request.header.opcode);

    if request.has_extras() || request.has_value() {
        response.header.with_status(0x0004);
        return Some(response);
    }
    
    cache.remove(Key::new(request.key));
    response.header.with_status(0x0000);    
    Some(response)
}