// original: 0x008f00f0 probe_buffer_b
/// Probe buffer B: same probe protocol as buffer A over its own slot pair.
export!(thiscall, rw_008f00f0(this: u32) -> u32 {
    let cand = this.wrapping_add(0x2778);
    let r = callee_cdecl!(1, u32, cand);
    if (r as i32) < 0 {
        cand
    } else {
        this.wrapping_add(0x2788)
    }
});
