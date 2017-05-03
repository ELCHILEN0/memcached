use packet::MemPacket;

use cache::cache::Cache;
use cache::key::Key;
use cache::value::Value;
use cache::storage_structure::CacheStorageStructure;
use cache::replacement_policy::CacheReplacementPolicy;

fn set<T: CacheStorageStructure, R: CacheReplacementPolicy>(request: MemPacket, cache: &mut Cache<T, R>, response: &mut MemPacket) {
    // TODO: If the Data Version Check (CAS) is nonzero, the requested operation MUST only succeed if the item exists and has a CAS value identical to the provided value.
    
    match cache.set(Key::new(request.key), Value::new(request.value)) {
        Ok(val) => {
            response.header.with_status(0x0000);
            response.header.with_cas(0x0000000000000001);
        },
        Err(_) => {
            response.header.with_status(0x0084);
        }
    }
    
}

pub fn set_command<T: CacheStorageStructure, R: CacheReplacementPolicy>(request: MemPacket, cache: &mut Cache<T, R>) -> Option<MemPacket> {
    println!("set_command");

    let mut response = MemPacket::new(false);
    response.header.with_opcode(request.header.opcode);

    // TODO: Required
    // if !request.has_extras() || !request.has_key() {
    //     response.header.with_status(0x0004);
    //     return response;
    // }
    
    set(request, cache, &mut response);
    Some(response)
}

pub fn add_command<T: CacheStorageStructure, R: CacheReplacementPolicy>(request: MemPacket, cache: &mut Cache<T, R>) -> Option<MemPacket> {
    println!("add_command");

    let mut response = MemPacket::new(false);
    response.header.with_opcode(request.header.opcode);

    // TODO: Required
    // if !request.has_extras() || !request.has_key() {
    //     response.header.with_status(0x0004);
    //     return response;
    // }

    if cache.contains(Key::new(request.key.clone())) {
        response.header.with_status(0x0005);
        return Some(response);
    }

    set(request, cache, &mut response);
    Some(response)
}

pub fn replace_command<T: CacheStorageStructure, R: CacheReplacementPolicy>(request: MemPacket, cache: &mut Cache<T, R>) -> Option<MemPacket> {
    println!("replace_command");

    let mut response = MemPacket::new(false);
    response.header.with_opcode(request.header.opcode);

    // TODO: Required
    // if !request.has_extras() || !request.has_key() {
    //     response.header.with_status(0x0004);
    //     return response;
    // }

    if !cache.contains(Key::new(request.key.clone())) {
        response.header.with_status(0x0005);
        return Some(response);
    }

    set(request, cache, &mut response);
    Some(response)
}