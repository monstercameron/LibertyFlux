// original: 0x00a931c0 stream_slot_dispatch

/// Dispatches a manager slot and its probe pointer.
///
/// Reads the manager at file VA 0x12FB258 (array base at `+0`, probe
/// offset at `+4`, stride at `+0xC`): when the probe byte at `p + offset`
/// has its top bit set the slot is 0, else `base + stride * p`. Sends the
/// slot (callee 1, thiscall) with the context at file VA 0x12FB260, then
/// sends `p` itself (callee 2, cdecl) and returns that answer. Two calls.
/// Original: 0x00A931C0 (cdecl, one stack word), 55 bytes.
lf_checker_rt::export!(cdecl, rw_00a931c0(p: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x12FB258;
        const CTX: u32 = 0x12FB260;
        const SEND_SLOT: u32 = 1;
        const SEND_PTR: u32 = 2;
        let mgr = lf_checker_rt::global::<u32>(MGR).read();
        let off = (mgr.wrapping_add(4) as *const u32).read_unaligned();
        let probe = (p.wrapping_add(off) as *const u8).read();
        let slot = if (probe & 0x80) != 0 {
            0
        } else {
            let stride = (mgr.wrapping_add(0xC) as *const u32).read_unaligned();
            let base = (mgr as *const u32).read_unaligned();
            base.wrapping_add(stride.wrapping_mul(p))
        };
        let ctx = lf_checker_rt::global::<u32>(CTX).read();
        let _: u32 = lf_checker_rt::callee_thiscall!(SEND_SLOT, u32, ctx, slot);
        let ans: u32 = lf_checker_rt::callee_cdecl!(SEND_PTR, u32, p);
        ans
    }
});
