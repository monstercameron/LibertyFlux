// original: 0x00cb6ba0 status_bit5_or_true
/// Status bit 5 of the linked sub-object, or true when the holder flag is clear.
#[allow(clippy::all)]
#[allow(non_snake_case)]
#[allow(unused_unsafe)]
export!(thiscall, rw_cb6ba0(this_ptr: u32) -> u8 {
    unsafe {
        let holder_flag = ((this_ptr.wrapping_add(0xB4)) as *const u8).read();
        if holder_flag & 1 == 0 {
            1
        } else {
            let inner = ((this_ptr.wrapping_add(0x20)) as *const u32).read();
            let status_obj = ((inner.wrapping_add(0x50)) as *const u32).read();
            let status = ((status_obj.wrapping_add(0x14)) as *const u32).read();
            ((status >> 5) & 1) as u8
        }
    }
});
