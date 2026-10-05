// original: 0x00cd24f0 task_flag_dispatch
/// Copy a 16-byte pose block to `dst`, or dispatch on the kind field.
///
/// When bits 6-9 of the word at `src+0x28` equal 0xc0, the dispatch helper
/// runs (thiscall on `src`, `dst` and the constant 0x4b2). Otherwise 16
/// bytes are copied to `dst` from the alternate block (`[src+0x20]+0x30`
/// when that pointer is non-null, else `src+0x10`). Returns `dst`. Cdecl.
export!(cdecl, rw_00cd24f0(dst: u32, src: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_DISPATCH: u32 = 0xc0;
        const ALT_OFF: u32 = 0x20;
        const ALT_BIAS: u32 = 0x30;
        const DIRECT_OFF: u32 = 0x10;
        const CODE: u32 = 0x4b2;
        let kind =
            (src.wrapping_add(KIND_OFF) as *const u32).read_unaligned() & KIND_MASK;
        if kind == KIND_DISPATCH {
            let _: u32 = callee_thiscall!(1, u32, src, dst, CODE);
        } else {
            let alt = (src.wrapping_add(ALT_OFF) as *const u32).read_unaligned();
            let from = if alt != 0 {
                alt.wrapping_add(ALT_BIAS)
            } else {
                src.wrapping_add(DIRECT_OFF)
            };
            let mut i = 0u32;
            while i < 4 {
                let w = (from.wrapping_add(i * 4) as *const u32).read_unaligned();
                (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
                i += 1;
            }
        }
        dst
    }
});
