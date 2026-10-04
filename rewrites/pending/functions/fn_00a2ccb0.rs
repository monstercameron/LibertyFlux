// original: 0x00a2ccb0 ped_value_refresh_7

/// Refresh the float at `+0x3B4` of the ped's inner block from worker 7.
/// Same shape as `rw_00a2cc60` with query argument 7, destination
/// `inner + 0x3B4`, and no flag clearing afterwards.
/// Original: 0x00a2ccb0 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00a2ccb0(this: u32) -> u32 {
    unsafe {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn wrf(a: u32, v: f32) {
        unsafe { wr32(a, v.to_bits()) }
    }
        const EXT: u32 = 0x228;
        const INNER: u32 = 0x70;
        const DST: u32 = 0x3b4;
        const QUERY: u32 = 1;
        const ARG: u32 = 7;
        let raw = rd32(this.wrapping_add(EXT));
        let dst = if raw == 0 { 0 } else { raw.wrapping_add(INNER) };
        let v: f32 = lf_checker_rt::callee_cdecl!(QUERY, f32, ARG);
        wrf(dst.wrapping_add(DST), v);
        0
    }
});
