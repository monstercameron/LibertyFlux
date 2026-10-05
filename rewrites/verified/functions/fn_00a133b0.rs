// original: 0x00a133b0 up_vector_or_default (proposed)
/// Copy a direction row from the linked object, or a default up vector.
///
/// `this + 0x20` points at a linked object. When it is non-null, the three
/// words at its offsets `0x20`, `0x24`, `0x28` are copied to `out`. When it
/// is null, `out` gets `(0, 0, 1.0)`. Returns `out`. Thiscall, one stack
/// argument, callee cleans it.
export!(thiscall, rw_00a133b0(this: u32, out: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0x20;
        const ROW_OFF: u32 = 0x20;
        const ONE_BITS: u32 = 0x3f800000;
        let link = ((this + LINK_OFF) as *const u32).read_unaligned();
        if link != 0 {
            for k in 0..3u32 {
                let w = ((link + ROW_OFF + 4 * k) as *const u32).read_unaligned();
                ((out + 4 * k) as *mut u32).write_unaligned(w);
            }
        } else {
            (out as *mut u32).write_unaligned(0);
            ((out + 4) as *mut u32).write_unaligned(0);
            ((out + 8) as *mut u32).write_unaligned(ONE_BITS);
        }
        out
    }
});
