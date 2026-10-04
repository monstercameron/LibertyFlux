// original: 0x00a2cc60 ped_value_refresh_8

/// Refresh the float at `+0x3E0` of the ped's inner block from worker 8:
/// calls the worker (callee 1, cdecl with the single argument 8, float
/// result on ST0) and stores the answer at `inner + 0x3E0`, where `inner`
/// is `[this+0x228] + 0x70` (a null extension faults on the store, as in
/// the original). Afterwards the flag word at bit 0x80 is cleared: at
/// `raw + 0x440` for a live extension, or at address `0x3D0` when the
/// extension pointer re-read after the call is null.
/// Original: 0x00a2cc60 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00a2cc60(this: u32) -> u32 {
    unsafe {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn wrf(a: u32, v: f32) {
        unsafe { wr32(a, v.to_bits()) }
    }
        const EXT: u32 = 0x228;
        const INNER: u32 = 0x70;
        const DST: u32 = 0x3e0;
        const CLR: u32 = 0x440;
        const FALLBACK: u32 = 0x3d0;
        const KEEP: u32 = 0xffffff7f;
        const QUERY: u32 = 1;
        const ARG: u32 = 8;
        let raw = rd32(this.wrapping_add(EXT));
        let dst = if raw == 0 { 0 } else { raw.wrapping_add(INNER) };
        let v: f32 = lf_checker_rt::callee_cdecl!(QUERY, f32, ARG);
        wrf(dst.wrapping_add(DST), v);
        let raw2 = rd32(this.wrapping_add(EXT));
        if raw2 == 0 {
            wr32(FALLBACK, rd32(FALLBACK) & KEEP);
        } else {
            let p = raw2.wrapping_add(CLR);
            wr32(p, rd32(p) & KEEP);
        }
        0
    }
});
