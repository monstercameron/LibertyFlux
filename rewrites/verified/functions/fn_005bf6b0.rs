// original: 0x005BF6B0 get_viewport_component_pointer
/// Return the nested component pointer in a viewport object. A non-null
/// pointer at `this + 8` takes precedence and yields its field at `+0xb4`;
/// otherwise a non-null pointer at `this + 0xc` yields its first word. If
/// neither pointer is present, return zero.
///
/// The original is a no-argument thiscall function; pointers and fields are
/// 32-bit game-layout values.
lf_checker_rt::export!(thiscall, rw_005bf6b0(this: u32) -> u32 {
    unsafe {
        let first = (this.wrapping_add(8) as *const u32).read_unaligned();
        if first != 0 {
            return (first.wrapping_add(0xb4) as *const u32).read_unaligned();
        }
        let second = (this.wrapping_add(0xc) as *const u32).read_unaligned();
        if second != 0 {
            return (second as *const u32).read_unaligned();
        }
        0
    }
});
