// original: 0x00dda8a0 UIBasicClip::vf127

/// Tail-jump to a slot of the inner object's function table.
///
/// `this` is the clip object. The inner object at `this + PART2 (+0x1e8)`
/// is loaded, its table pointer is loaded from its first word, and control
/// jumps to the entry at `SLOT (+0x208)` with the inner object still in ECX
/// (a computed tail jump: the stack is untouched). The rewrite performs the
/// same load-and-call through the same fabricated table; both sides land on
/// the checker's recorder stub, whose answer in EAX is the result.
///
/// Original: thiscall, no stack arguments, tail jump, word result in EAX.
lf_checker_rt::export!(thiscall, rw_00dda8a0(this: u32) -> u32 {
    const PART2: u32 = 0x1e8;
    const SLOT: u32 = 0x208;
    unsafe {
        let inner = ((this + PART2) as *const u32).read_unaligned();
        let table = (inner as *const u32).read_unaligned();
        let target = ((table + SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(inner)
    }
});
