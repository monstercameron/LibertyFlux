// original: 0x00c80d30 scenario_detail_query (proposed) — UNVERIFIED (deferred)

// NOTE: deferred with reason `frame_pointer_args` (plus encrypted bytes): two
// callees take pointers to the function's own aligned stack frame. Kept for a
// future checker or re-run; never passed, not verified.

/// Query scenario details for subject `a0`, returning a row count or 0.
///
/// The subject's virtual slot `+0x54` resolves a handle whose byte at `+0x14`
/// is the mode; `c1(a0)` must agree or the result is 0. `c2(mode)` tags the
/// mode: tags 2 and 3 run `c4(a5, a4, a1, a0, mode)` directly, other tags run
/// `c3` over two scratch words and then `c5` over nine arguments (both take
/// frame pointers). A positive count also stores the shared generation word
/// at `+0x4c` of the row `c6(mode)` addresses. Returns the count.
///
/// Original: cdecl, six stack words (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c80d30(a0: u32, a1: u32, a2f: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const VT_SLOT: u32 = 0x54;
        const MODE_OFF: u32 = 0x14;
        const ROW_OFF: u32 = 0x4c;
        const GEN: u32 = 0x11735b4;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const C3: u32 = 3;
        const C4: u32 = 4;
        const C5: u32 = 5;
        const C6: u32 = 6;
        type Hook = extern "thiscall" fn(u32) -> u32;
        let vt = (a0 as *const u32).read_unaligned();
        let resolve: Hook =
            core::mem::transmute(((vt + VT_SLOT) as *const u32).read_unaligned() as usize);
        let mode = ((resolve(a0) + MODE_OFF) as *const u8).read() as u32;
        let ok: u32 = lf_checker_rt::callee_cdecl!(C1, u32, a0);
        if (ok & 0xff) == 0 {
            return 0;
        }
        let tag: u32 = lf_checker_rt::callee_cdecl!(C2, u32, mode);
        let count: u32;
        if tag == 2 || tag == 3 {
            count = lf_checker_rt::callee_cdecl!(C4, u32, mode, a0, a1, a4, a5);
        } else {
            let mut out_a = [0u32; 4];
            let mut out_b = [0u32; 4];
            let mid: u32 = lf_checker_rt::callee_cdecl!(
                C3, u32, mode, out_b.as_mut_ptr() as u32, a0,
                out_a.as_mut_ptr() as u32, a1);
            if mid == 0 {
                return 0;
            }
            let mut tmp_c = [0u32; 4];
            let mut tmp_d = [0u32; 4];
            let r: u32 = lf_checker_rt::callee_cdecl!(
                C5, u32, mode, a0, mid, tmp_d.as_mut_ptr() as u32, a2f, a3,
                tmp_c.as_mut_ptr() as u32, a1, a5);
            count = r & 0xff;
        }
        if (count as i32) > 0 {
            let row: u32 = lf_checker_rt::callee_cdecl!(C6, u32, mode);
            let gen = lf_checker_rt::global::<u32>(GEN).read_unaligned();
            ((row + ROW_OFF) as *mut u32).write_unaligned(gen);
        }
        count
    }
});
