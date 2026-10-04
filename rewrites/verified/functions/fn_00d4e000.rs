// original: 0x00D4E000 CTaskSimpleShakeFist::vf5

// Event handler (vf5): releases the held object on events 1 and 2, otherwise idles.
///
/// The second stack argument selects the path. On any other value the word at
/// `+0x18`, when non-null, goes in ECX with the float -4.0 to intercepted
/// callee 1 and the result is 0. On events 1 and 2 the same call happens,
/// then the re-read word goes in ECX with the object pointer to intercepted
/// callee 2, the slot is cleared, and the result is 1.
///
/// Original: 0x00D4E000 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d4e000(this: u32, _a0: u32, ev: u32, _a2: u32) -> u32 {
    unsafe {
        const NEG_FOUR_BITS: u32 = 0xC0800000; // -4.0f
        const C1: u32 = 1;
        const C2: u32 = 2;
        if ev == 1 || ev == 2 {
            let w = ((this + 0x18) as *const u32).read_unaligned();
            if w != 0 {
                lf_checker_rt::callee_thiscall!(C1, u32, w, NEG_FOUR_BITS);
            }
            let w2 = ((this + 0x18) as *const u32).read_unaligned();
            if w2 != 0 {
                lf_checker_rt::callee_thiscall!(C2, u32, w2, this);
                ((this + 0x18) as *mut u32).write_unaligned(0);
            }
            1
        } else {
            let w = ((this + 0x18) as *const u32).read_unaligned();
            if w != 0 {
                lf_checker_rt::callee_thiscall!(C1, u32, w, NEG_FOUR_BITS);
            }
            0
        }
    }
});
