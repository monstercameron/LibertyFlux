// original: 0x008f0120 buffer_selector_c
/// Buffer selector C: same flag-then-probe protocol over its own slots.
export!(thiscall, rw_008f0120(this: u32) -> u32 {
    if unsafe { *global::<u32>(0x117E6DC) } != 0 {
        return this.wrapping_add(0x2C28);
    }
    let cand = this.wrapping_add(0x28B8);
    let r = callee_cdecl!(1, u32, cand);
    if (r as i32) < 0 {
        cand
    } else {
        this.wrapping_add(0x28C8)
    }
});
