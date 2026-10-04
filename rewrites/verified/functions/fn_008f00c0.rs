// original: 0x008f00c0 probe_buffer_a
/// Probe buffer A: runs the shared probe over the candidate slot; a negative
/// answer keeps the candidate, otherwise the alternate slot is selected.
export!(thiscall, rw_008f00c0(this: u32) -> u32 {
    let cand = this.wrapping_add(0x2758);
    let r = callee_cdecl!(1, u32, cand);
    if (r as i32) < 0 {
        cand
    } else {
        this.wrapping_add(0x2768)
    }
});
