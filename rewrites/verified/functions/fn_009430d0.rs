// original: 0x009430d0 streaming_budget_raise (proposed)

/// Raise the object's budget floor to at least the scaled request.
///
/// Multiplies the argument by 1000 (wrapping), adds the global base, and
/// stores the unsigned maximum of that and the current floor at
/// `this + 0x568`. Returns the previous floor in `eax`.
///
/// Original: 0x009430d0 (thiscall, one stack argument; callee pops 4).
lf_checker_rt::export!(thiscall, rw_009430d0(this: u32, count: u32) -> u32 {
    unsafe {
        const FLOOR: u32 = 0x568;
        const BASE: u32 = 0x011735B4;
        const SCALE: u32 = 1000;
        let slot = (this + FLOOR) as *mut u32;
        let old = slot.read_unaligned();
        let want = count
            .wrapping_mul(SCALE)
            .wrapping_add(lf_checker_rt::global::<u32>(BASE).read());
        slot.write_unaligned(if old > want { old } else { want });
        old
    }
});
