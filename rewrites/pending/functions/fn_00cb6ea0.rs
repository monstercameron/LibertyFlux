// original: 0x00cb6ea0 linked_float_or_16
/// Linked float (+0x10 of the sub-object), or 16.0 when the holder flag is clear.
#[allow(clippy::all)]
#[allow(non_snake_case)]
#[allow(unused_unsafe)]
export!(thiscall, rw_cb6ea0(this_ptr: u32) -> f32 {
    unsafe {
        let holder_flag = ((this_ptr.wrapping_add(0xB4)) as *const u8).read();
        if holder_flag & 1 == 0 {
            16.0
        } else {
            let inner = ((this_ptr.wrapping_add(0x20)) as *const u32).read();
            let status_obj = ((inner.wrapping_add(0x50)) as *const u32).read();
            ((status_obj.wrapping_add(0x10)) as *const f32).read()
        }
    }
});
