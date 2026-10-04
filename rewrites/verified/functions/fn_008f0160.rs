// original: 0x008f0160 buffer_selector_d
/// Buffer selector D: same flag-then-probe protocol over its own slots.
export!(thiscall, rw_008f0160(this: u32) -> u32 {
    if unsafe { *global::<u32>(0x117E6E0) } != 0 {
        return this.wrapping_add(0x2C18);
    }
    let cand = this.wrapping_add(0x28D8);
    let r = callee_cdecl!(1, u32, cand);
    if (r as i32) < 0 {
        cand
    } else {
        this.wrapping_add(0x28E8)
    }
});
