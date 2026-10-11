// original: 0x008C59B0 proposed
/// Copy bounded bytes to UTF-16 storage, sign-extending each byte. Null
/// input writes an empty destination and returns its pointer; otherwise the
/// return is the number of copied characters.
lf_checker_rt::export!(cdecl, rw_008c59b0(destination: u32, source: u32, capacity: u32) -> u32 {
    unsafe {
        if source == 0 { (destination as *mut u16).write_unaligned(0); return destination; }
        let limit = capacity.wrapping_sub(1) as i32;
        let mut copied = 0u32;
        while (copied as i32) < limit {
            let byte = (source.wrapping_add(copied) as *const u8).read();
            if byte == 0 { break; }
            let value = (byte as i8 as i16) as u16;
            (destination.wrapping_add(copied.wrapping_mul(2)) as *mut u16).write_unaligned(value);
            copied = copied.wrapping_add(1);
        }
        (destination.wrapping_add(copied.wrapping_mul(2)) as *mut u16).write_unaligned(0);
        copied
    }
});
