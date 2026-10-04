// original: 0x008d4e20 crc32_of_bytes
/// CRC32 (same parameters as `crc32_of_string`) of a byte buffer with an
/// explicit length. Non-positive lengths read nothing and yield 0xFFFF_FFFF.
export!(cdecl, rw_008d4e20(s: u32, len: i32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0103_2368;
        if len <= 0 {
            return 0xFFFF_FFFF;
        }
        let table = global::<[u32; 256]>(TABLE);
        let mut crc: u32 = 0xFFFF_FFFF;
        let mut p = s as *const u8;
        let mut n = len;
        while n > 0 {
            let idx = ((crc ^ (*p) as u32) & 0xFF) as usize;
            crc = (*table)[idx] ^ (crc >> 8);
            p = p.add(1);
            n -= 1;
        }
        crc
    }
});
