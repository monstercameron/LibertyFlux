// original: 0x00a9e980 stream_route_by_limit (proposed)

/// Route `obj` by its limit word: notify nearby, tail away when at the limit.
///
/// The object's virtual slot `+0xa0` (object in ECX) resolves the limit
/// record; a null answer falls back to the word at `obj + 0x38`, and a null
/// there returns 0. A limit value of 0xffff at record `+8` returns the
/// record address itself. Otherwise the target at `[[record + 4] + 0xc]`
/// carries the threshold at `+8`: when `level` differs from it (ordered
/// float equality; NaN always differs) the notifier runs with the context
/// global (file address 0x012b9c78) in ECX and (`limit`, `level` bits, 0),
/// and its answer is the result. On equality the call tail-jumps to the
/// forwarder with the same context in ECX and (`limit`, 0), and its answer
/// is the result.
///
/// Original: 0x00a9e980 (stdcall, two stack words: object pointer, level
/// bits; equality path ends in a tail jump).
lf_checker_rt::export!(stdcall, rw_00a9e980(obj: u32, level_bits: u32) -> u32 {
    unsafe {
        const ASK_SLOT: u32 = 0xa0;
        const FALLBACK_OFF: u32 = 0x38;
        const LIMIT_OFF: u32 = 8;
        const NO_LIMIT: u32 = 0xffff;
        const CONTEXT_GLOBAL: u32 = 0x012b9c78;
        const ASK: u32 = 1;
        const NOTIFY: u32 = 2;
        const FORWARD: u32 = 3;
        let vtable = (obj as *const u32).read_unaligned();
        let at = ((vtable + ASK_SLOT) as *const u32).read_unaligned();
        let ask: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(at as usize);
        let got: u32 = ask(obj);
        let record = if got == 0 {
            let fb = ((obj + FALLBACK_OFF) as *const u32).read_unaligned();
            if fb == 0 {
                return 0;
            }
            fb
        } else {
            got
        };
        let limit = ((record + LIMIT_OFF) as *const u16).read_unaligned() as u32;
        if limit == NO_LIMIT {
            return record;
        }
        let mid = ((record + 4) as *const u32).read_unaligned();
        let target = ((mid + 0x0c) as *const u32).read_unaligned();
        let threshold = f32::from_bits(((target + 8) as *const u32).read_unaligned());
        let level = f32::from_bits(level_bits);
        let ctx = lf_checker_rt::global::<u32>(CONTEXT_GLOBAL).read_unaligned();
        // Ordered equality, exactly the original's ucomiss-then-parity-test:
        // equal (including +0 versus -0) tails away, everything else (NaN
        // included) notifies.
        if level == threshold {
            return lf_checker_rt::callee_thiscall!(FORWARD, u32, ctx, limit, 0);
        }
        lf_checker_rt::callee_thiscall!(NOTIFY, u32, ctx, limit, level_bits, 0)
    }
});
