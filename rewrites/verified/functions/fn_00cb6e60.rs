// original: 0x00cb6e60 linked_float_or_local
/// Linked float (+0x8 of the sub-object) or the local float at +0x6C.
#[allow(clippy::all)]
#[allow(non_snake_case)]
#[allow(unused_unsafe)]
export!(thiscall, rw_cb6e60(this_ptr: u32) -> f32 {
    unsafe {
        let holder_flag = ((this_ptr.wrapping_add(0xB4)) as *const u8).read();
        if holder_flag & 1 == 0 {
            ((this_ptr.wrapping_add(0x6C)) as *const f32).read()
        } else {
            let inner = ((this_ptr.wrapping_add(0x20)) as *const u32).read();
            let status_obj = ((inner.wrapping_add(0x50)) as *const u32).read();
            ((status_obj.wrapping_add(0x8)) as *const f32).read()
        }
    }
});
