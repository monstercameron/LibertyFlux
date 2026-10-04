// original: 0x008f0000 buffer_selector_a
/// Buffer selector A: while the mode flag is set, the ready buffer; otherwise
/// the shared probe picks between the candidate and alternate slots.
export!(thiscall, rw_008f0000(this: u32) -> u32 {
    if unsafe { *global::<u32>(0x117E6DC) } != 0 {
        return this.wrapping_add(0x2C28);
    }
    let cand = this.wrapping_add(0x2798);
    let r = callee_cdecl!(1, u32, cand);
    if (r as i32) < 0 {
        cand
    } else {
        this.wrapping_add(0x27A8)
    }
});
