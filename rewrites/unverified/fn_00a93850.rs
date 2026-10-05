// original: 0x00a93850 stream_flush_pending_queue (proposed)

/// Flush the pending queue: sweep each entry, then drop the count.
///
/// The slot sweep runs first (callee 1, cdecl/0). Then each queued pointer
/// in the global array while the index is below the global count is passed
/// to the disposer (callee 2, cdecl/1); null entries are skipped. The
/// global count is cleared at the end on every path.
///
/// Returns the last disposer answer, 0 when the last entry was null, or
/// the sweep answer when the queue was empty. Cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_00a93850() -> u32 {
    unsafe {
        const COUNT_GLOBAL: u32 = 0x012fb274;
        const QUEUE_GLOBAL: u32 = 0x012fb278;
        let mut ret = lf_checker_rt::callee_cdecl!(1, u32,);
        if (lf_checker_rt::global::<i32>(COUNT_GLOBAL).read_unaligned()) <= 0 {
            lf_checker_rt::global::<u32>(COUNT_GLOBAL).write_unaligned(0);
            return ret;
        }
        let mut i = 0u32;
        while (i as i32)
            < lf_checker_rt::global::<i32>(COUNT_GLOBAL).read_unaligned()
        {
            let v = ((lf_checker_rt::relocated(QUEUE_GLOBAL) + i.wrapping_mul(4))
                as *const u32)
                .read_unaligned();
            if v == 0 {
                ret = 0;
            } else {
                ret = lf_checker_rt::callee_cdecl!(2, u32, v);
            }
            i = i.wrapping_add(1);
        }
        lf_checker_rt::global::<u32>(COUNT_GLOBAL).write_unaligned(0);
        ret
    }
});
