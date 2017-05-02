use packet::{MemPacket, MemHeader};
use command;

use cache::cache::Cache;
use cache::key::Key;
use cache::value::Value;
use cache::storage_structure::CacheStorageStructure;
use cache::replacement_policy::CacheReplacementPolicy;

pub fn set_command<R: CacheReplacementPolicy, T: CacheStorageStructure<R>>(request: MemPacket, cache: &mut Cache<T, R>) -> Option<MemPacket> {
    println!("set_command");

    let mut response = MemPacket::new(false);
    response.header.with_opcode(request.header.opcode);

    // TODO: Required
    // if request.extras.len() <= 0 || request.key.len() <= 0 {
    //     response.header.with_status(0x0004);
    //     return response;
    // }

    // TODO: If the Data Version Check (CAS) is nonzero, the requested operation MUST only succeed if the item exists and has a CAS value identical to the provided value.
    // Add MUST fail if the item already exist.
    // Replace MUST fail if the item doesn't exist.
    // Set should store the data unconditionally if the item exists or not.

    cache.storage_structure.set(Key::new(request.key), Value::new(request.value));
    response.header.with_status(0x0000);
    response.header.with_cas(0x0000000000000001);
    
    Some(response)
}