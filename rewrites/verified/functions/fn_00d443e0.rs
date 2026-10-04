// original: 0x00d443e0 CTaskSimplePlayRandomAmbients::vf1

/// Emit this task's ambient effect through the pool object, if one is live.
///
/// `this` points to the task: float at `+0x30`, optional parameter block at
/// `+0x40`, one argument dword at `+0x58`, flag bytes at `+0x5c` and `+0x5d`.
/// Fetches the pool object via the pool helper (whose object comes from a
/// static slot); a null pool object returns 0 at once. Otherwise calls the
/// emitter with six arguments: the optional block (`this+0x40` when flag bit
/// 2 of `+0x5c` is set, else null), the float bits, bit 0 of `+0x5c`, bits 1
/// and 2 of `+0x5d`, and the dword at `+0x58`. Returns the emitter's answer.
///
/// The original pushes the float by overwriting a pushed scratch slot with a
/// vector store; the rewrite passes the same bits as a plain word.
///
/// Original: thiscall, no stack words. Both callees are intercepted.
lf_checker_rt::export!(thiscall, rw_00d443e0(this: u32) -> u32 {
    unsafe {
        const POOL_SLOT: u32 = 0x0167e2a0;
        const POOL_CALLEE: u32 = 1;
        const EMIT_CALLEE: u32 = 2;
        const FLOAT_OFF: u32 = 0x30;
        const OPT_OFF: u32 = 0x40;
        const ARG_OFF: u32 = 0x58;
        const FLAGS: u32 = 0x5c;
        const FLAGS2: u32 = 0x5d;
        const OPT_BIT: u8 = 2;
        let pool = (lf_checker_rt::global::<u32>(POOL_SLOT) as *const u32).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(POOL_CALLEE, u32, pool);
        if obj == 0 {
            return 0;
        }
        let dl = ((this + FLAGS) as *const u8).read();
        let opt = if dl & OPT_BIT != 0 { this.wrapping_add(OPT_OFF) } else { 0 };
        let cl = ((this + FLAGS2) as *const u8).read();
        let a = ((this + ARG_OFF) as *const u32).read_unaligned();
        let f = ((this + FLOAT_OFF) as *const u32).read_unaligned();
        let b2 = ((cl >> 2) & 1) as u32;
        let b1 = ((cl >> 1) & 1) as u32;
        let b0 = (dl & 1) as u32;
        lf_checker_rt::callee_thiscall!(EMIT_CALLEE, u32, obj, opt, f, b0, b1, b2, a)
    }
});
