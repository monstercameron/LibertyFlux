// original: 0x008f01e0 probe_buffer_c
/// Probe buffer C: same probe protocol over its own slot pair.
export!(thiscall, rw_008f01e0(this: u32) -> u32 {
    let cand = this.wrapping_add(0x2898);
    let r = callee_cdecl!(1, u32, cand);
    if (r as i32) < 0 {
        cand
    } else {
        this.wrapping_add(0x28A8)
    }
});
