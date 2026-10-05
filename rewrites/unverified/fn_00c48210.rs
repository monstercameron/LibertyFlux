// original: 0x00c48210 CCamScripted::vf8 (symbols)
/// Copy the stored 4-word vector at `this + SRC` to `this + DST`.
///
/// Four words move from offsets `0x210..0x21c` to `0x50..0x5c` (the
/// original uses plain moves for the outer words and scalar float moves
/// for the middle two; all four are plain 32-bit copies). Returns the
/// last word copied, as the original leaves it in eax.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c48210(this: u32) -> u32 {
    const SRC: u32 = 0x210;
    const DST: u32 = 0x50;
    unsafe {
        let w0 = ((this + SRC) as *const u32).read_unaligned();
        let w1 = ((this + SRC + 4) as *const u32).read_unaligned();
        let w2 = ((this + SRC + 8) as *const u32).read_unaligned();
        let w3 = ((this + SRC + 12) as *const u32).read_unaligned();
        ((this + DST) as *mut u32).write_unaligned(w0);
        ((this + DST + 4) as *mut u32).write_unaligned(w1);
        ((this + DST + 8) as *mut u32).write_unaligned(w2);
        ((this + DST + 12) as *mut u32).write_unaligned(w3);
        w3
    }
});
