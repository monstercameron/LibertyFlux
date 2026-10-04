// original: 0x008d4df0 crc32_of_string
/// CRC32 (zlib table, initial 0xFFFF_FFFF, no final xor) of a NUL-terminated
/// byte string. An empty string yields 0xFFFF_FFFF.
export!(cdecl, rw_008d4df0(s: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0103_2368;
        let table = global::<[u32; 256]>(TABLE);
        let mut crc: u32 = 0xFFFF_FFFF;
        let mut p = s as *const u8;
        loop {
            let b = *p;
            if b == 0 {
                break;
            }
            let idx = ((crc ^ b as u32) & 0xFF) as usize;
            crc = (*table)[idx] ^ (crc >> 8);
            p = p.add(1);
        }
        crc
    }
});
