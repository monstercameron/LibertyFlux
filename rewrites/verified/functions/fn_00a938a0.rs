// original: 0x00a938a0 stream_sweep_entries (proposed)

/// Sweep the sweep table: retire dead records, then mark newly ready ones.
///
/// The sweep global points at `{ base, flag_bytes, count, stride }`. The
/// first pass visits slots `1..count` (signed): slots whose flag byte has
/// `0x80` set are skipped, as are slots whose record pointer
/// (`base + stride * i`) is null or whose dword at `+0x50` is not -1; the
/// rest go to the retire callee. The second pass visits the same slots and
/// sends records whose byte at `+0x54` is 0 to the probe callee with the
/// global handle; when the byte at `+0x55` is then non-zero the `+0x54`
/// byte is set to 1. Returns the sweep global (the last value the original
/// leaves in `eax` on every path).
///
/// Original: cdecl, no stack arguments. Two callees (cdecl, 1/2 args).
lf_checker_rt::export!(cdecl, rw_00a938a0() -> u32 {
    unsafe {
        const SWEEP_G: u32 = 0x012fb258;
        const HANDLE_G: u32 = 0x0103e89c;
        const DEAD_MARK: u32 = 0xffff_ffff;
        const SKIP_FLAG: u8 = 0x80;
        const RETIRE: u32 = 0;
        const PROBE: u32 = 1;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        let sweep = rd32(lf_checker_rt::relocated(SWEEP_G));
        let base = rd32(sweep);
        let flag_bytes = rd32(sweep.wrapping_add(4));
        let count = rd32(sweep.wrapping_add(8)) as i32;
        let stride = rd32(sweep.wrapping_add(0x0c));
        let mut i: i32 = 1;
        while i < count {
            let rec = base.wrapping_add(stride.wrapping_mul(i as u32));
            if rd8(flag_bytes.wrapping_add(i as u32)) & SKIP_FLAG == 0
                && rec != 0
                && rd32(rec.wrapping_add(0x50)) == DEAD_MARK
            {
                let _: u32 = lf_checker_rt::callee_cdecl!(RETIRE, u32, i as u32);
            }
            i = i.wrapping_add(1);
        }
        let handle = rd32(lf_checker_rt::relocated(HANDLE_G));
        let mut j: i32 = 1;
        while j < count {
            let rec = base.wrapping_add(stride.wrapping_mul(j as u32));
            if rd8(flag_bytes.wrapping_add(j as u32)) & SKIP_FLAG == 0
                && rec != 0
                && rd8(rec.wrapping_add(0x54)) == 0
            {
                let _: u32 =
                    lf_checker_rt::callee_cdecl!(PROBE, u32, j as u32, handle);
                if rd8(rec.wrapping_add(0x55)) != 0 {
                    wr8(rec.wrapping_add(0x54), 1);
                }
            }
            j = j.wrapping_add(1);
        }
        sweep
    }
});
