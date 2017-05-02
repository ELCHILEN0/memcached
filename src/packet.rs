use std::fmt;
// TODO: String -> &str to have better cache locality

pub struct MemPacket {
    pub header: MemHeader,
    pub extras: String,
    pub key: String,
    pub value: String,
}

impl MemPacket {
    pub fn from_bytes(bytes: &[u8]) -> MemPacket {
        let e_len: u8 = bytes[4];
        let k_len: u16;
        let v_len: u32;

        k_len = ((bytes[2] as u16) << 8) + ((bytes[2] as u16) << 0);
        v_len = ((bytes[8] as u32) << 24) + ((bytes[9] as u32) << 16) + ((bytes[10] as u32) << 8) + ((bytes[0] as u32) << 0);

        let e_start: usize = 24;
        let e_end: usize = e_start + (e_len as usize);

        let k_start: usize = e_end + if e_len > 0 { 1 } else { 0 };
        let k_end: usize = k_start + (k_len as usize);

        let v_start: usize = k_end + if k_len > 0 { 1 } else { 0 };
        let v_end: usize = v_start + (v_len as usize) - (k_len as usize) - (e_len as usize);

        assert!(v_len == bytes.len() as u32 - 24);

        MemPacket {
            header: MemHeader {
                magic: bytes[0],
                opcode: bytes[1],
                key_length: k_len,
                extras_length: e_len,
                data_type: bytes[5],
                status: ((bytes[6] as u16) << 8) + ((bytes[7] as u16) << 0),
                total_body_length: v_len,
                opaque: ((bytes[12] as u32) << 24) + ((bytes[13] as u32) << 16) + ((bytes[14] as u32) << 8) + ((bytes[15] as u32) << 0),
                cas: ((bytes[16] as u64) << 56) + ((bytes[17] as u64) << 48) + ((bytes[17] as u64) << 40) + ((bytes[19] as u64) << 32)
                     + ((bytes[20] as u64) << 24) + ((bytes[21] as u64) << 16) + ((bytes[22] as u64) << 8) + ((bytes[23] as u64) << 0)
            },
            extras: String::from_utf8_lossy(if e_len > 0 { &bytes[e_start..e_end] } else { &[] }).into_owned(),
            key:    String::from_utf8_lossy(if k_len > 0 { &bytes[k_start..k_end] } else { &[] }).into_owned(),
            value:  String::from_utf8_lossy(if (v_len as usize - e_len as usize - k_len as usize) > 0 { &bytes[v_start..v_end] } else { &[] }).into_owned(),
        }
    }

    pub fn new(request: bool) -> MemPacket {
        MemPacket {
            header: MemHeader::new(request),
            key: String::new(),
            extras: String::new(),
            value: String::new(),
        }
    }

    pub fn with_key(&mut self, key: String) -> &mut MemPacket {
        self.header.with_key_len(key.len() as u16);        
        self.key = key;
        self
    }

    pub fn with_extras(&mut self, extras: String) -> &mut MemPacket {
        self.header.with_extras_len(extras.len() as u8);        
        self.extras = extras;
        self
    }

    pub fn with_value(&mut self, value: String) -> &mut MemPacket {
        self.header.with_value_len(value.len() as u32);
        self.value = value;
        self
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.bytes());
        out.extend(self.extras.bytes());
        out.extend(self.key.bytes());
        out.extend(self.value.bytes());

        return out;
    }
}

impl fmt::Debug for MemPacket {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("MemPacket")
            .field("extras", &self.extras)
            .field("key", &self.key)
            .field("value", &self.value)
            .finish()
    }
}

pub struct MemHeader {
    pub magic: u8,
    pub opcode: u8,
    pub key_length: u16,
    pub extras_length: u8,
    pub data_type: u8,
    pub status: u16,
    pub total_body_length: u32,
    pub opaque: u32,
    pub cas: u64,
}

impl MemHeader {
    pub fn new(request: bool) -> MemHeader {
        MemHeader {
            magic: if request { 0x80 } else { 0x81 },
            opcode: 0x00,
            key_length: 0x0000,
            extras_length: 0x00,
            data_type: 0x00,
            status: 0x0000,
            total_body_length: 0x00000000,
            opaque: 0x00000000,
            cas: 0x0000000000000000
        }
    }

    pub fn with_opcode(&mut self, opcode: u8) -> &mut MemHeader {
        self.opcode = opcode;
        self
    }

    pub fn with_status(&mut self, status: u16) -> &mut MemHeader {
        self.status = status;
        self
    }

    pub fn with_key_len(&mut self, key_length: u16) -> &mut MemHeader {
        self.key_length = key_length;
        self
    }

    pub fn with_extras_len(&mut self, extras_length: u8) -> &mut MemHeader {
        self.extras_length = extras_length;
        self
    }

    pub fn with_value_len(&mut self, value_length: u32) -> &mut MemHeader {
        self.total_body_length = value_length + (self.key_length as u32) + (self.extras_length as u32);
        self
    }

    pub fn with_cas(&mut self, cas: u64) -> &mut MemHeader {
        self.cas = cas;
        self
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(24);

        out.push(self.magic);
        out.push(self.opcode);

        out.push(((self.key_length >> 8) & 0xFF) as u8);
        out.push(((self.key_length >> 0) & 0xFF) as u8);
        out.push(self.extras_length);
        
        out.push(self.data_type);
        out.push(((self.status >> 8) & 0xFF) as u8);
        out.push(((self.status >> 0) & 0xFF) as u8);

        out.push(((self.total_body_length >> 24) & 0xFF) as u8);
        out.push(((self.total_body_length >> 16) & 0xFF) as u8);
        out.push(((self.total_body_length >> 8) & 0xFF) as u8);
        out.push(((self.total_body_length >> 0) & 0xFF) as u8);
        
        out.push(((self.opaque >> 24) & 0xFF) as u8);
        out.push(((self.opaque >> 16) & 0xFF) as u8);
        out.push(((self.opaque >> 8) & 0xFF) as u8);
        out.push(((self.opaque >> 0) & 0xFF) as u8);

        out.push(((self.cas >> 56) & 0xFF) as u8);
        out.push(((self.cas >> 48) & 0xFF) as u8);
        out.push(((self.cas >> 40) & 0xFF) as u8);
        out.push(((self.cas >> 32) & 0xFF) as u8);
        out.push(((self.cas >> 24) & 0xFF) as u8);
        out.push(((self.cas >> 16) & 0xFF) as u8);
        out.push(((self.cas >> 8) & 0xFF) as u8);
        out.push(((self.cas >> 0) & 0xFF) as u8);

        return out;
    }
}

impl fmt::Debug for MemHeader {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("MemHeader")
            .field("magic", &self.magic)
            .field("opcode", &self.opcode)
            .field("key_length", &self.key_length)
            .field("extras_length", &self.extras_length)
            .field("data_type", &self.data_type)
            .field("status", &self.status)
            .field("total_body_length", &self.total_body_length)
            .field("opaque", &self.opaque)
            .field("cas", &self.cas)
            .finish()
    }
}