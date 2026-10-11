// original: 0x008C5A00 proposed
/// Copy the low byte of bounded UTF-16 characters into byte storage. Null
/// input writes an empty destination and returns its pointer; otherwise the
/// return is the number of copied characters.
lf_checker_rt::export!(cdecl, rw_008c5a00(destination: u32, source: u32, capacity: u32) -> u32 {
    unsafe {
        if source == 0 { (destination as *mut u8).write(0); return destination; }
        let limit = capacity.wrapping_sub(1) as i32;
        let mut copied = 0u32;
        while (copied as i32) < limit {
            let character = (source.wrapping_add(copied.wrapping_mul(2)) as *const u16).read_unaligned();
            if character == 0 { break; }
            (destination.wrapping_add(copied) as *mut u8).write(character as u8);
            copied = copied.wrapping_add(1);
        }
        (destination.wrapping_add(copied) as *mut u8).write(0);
        copied
    }
});
