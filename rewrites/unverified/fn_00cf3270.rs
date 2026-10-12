// original: 0x00CF3270 split_material_name
/// Copy bytes from the source string to the destination until the source ends or a vertical bar is reached, then write a NUL terminator and return the destination. The source and destination are the first and second stdcall arguments.
lf_checker_rt::export!(stdcall, rw_00cf3270(source: u32, destination: u32) -> u32 {
    let mut copied = 0u32;
    loop {
        let ch = unsafe { (source.wrapping_add(copied) as *const u8).read_unaligned() };
        if ch == 0 || ch == b'|' { break; }
        unsafe { (destination.wrapping_add(copied) as *mut u8).write_unaligned(ch); }
        copied = copied.wrapping_add(1);
    }
    unsafe { (destination.wrapping_add(copied) as *mut u8).write_unaligned(0); }
    destination
});
