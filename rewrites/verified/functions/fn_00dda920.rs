// original: 0x00dda920 UIBasicClip::vf125
/// Forward to the shared worker after loading the inner object.
///
/// The real target lives elsewhere; this entry only replaces the incoming
/// object pointer with the pointer stored at +0x1E0 and jumps on, passing
/// the single stack argument through untouched. The rewrite performs the
/// same load and forwards through the intercepted callee.
export!(thiscall, rw_00dda920(outer: u32, arg: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x1E0;
        let inner = ((outer.wrapping_add(INNER_OFF)) as *const u32).read();
        callee_thiscall!(1, u32, inner, arg)
    }
});
