// original: 0x00c3acb0 train_lookup_and_apply (proposed)
/// Look up an entry for (kind, index) and apply helper id 1 to it.
///
/// `this` (ECX) is the car, `idx` a slot index. Returns 0 when idx is
/// negative, when idx reaches the count the count-table holds for the
/// kind byte at `this+0x14e7`, or when the entry-table word for
/// `kind*20 + idx` is null (two chained leas: kind*5, then idx+that*4).
/// Otherwise pushes the unaligned dword at
/// `entry+1` and calls helper id 1 (thiscall/1) on a global object,
/// returning its answer. The contract pins the kind to three values and
/// scripts the reachable table words; the rest stay pristine.
///
/// Original: 0x00c3acb0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c3acb0(this: u32, idx: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x14e7;
        const FAN: u32 = 20;
        const COUNTS: u32 = 0x16d0870;
        const ENTRIES: u32 = 0x16d00f0;
        const OBJECT: u32 = 0x115dc74;
        const APPLY: u32 = 1;
        if (idx as i32) <= -1 {
            return 0;
        }
        let kind = ((this + KIND) as *const u8).read() as u32;
        let n = lf_checker_rt::global::<u32>(COUNTS + kind * 4).read_unaligned();
        if (idx as i32) >= (n as i32) {
            return 0;
        }
        let e = lf_checker_rt::global::<u32>(ENTRIES + (kind * FAN + idx) * 4).read_unaligned();
        if e == 0 {
            return 0;
        }
        let w = ((e + 1) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(APPLY, u32, lf_checker_rt::relocated(OBJECT), w)
    }
});
