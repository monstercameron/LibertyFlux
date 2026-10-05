// original: 0x00A8E930 pool_slot_search (proposed)

/// Find the slot whose helper index matches, or fetch a slot directly.
///
/// Returns 0 when the enable byte at `this+0x76` is clear. With a zero key
/// the slot table at `this+0xEC` is indexed by `b` directly. Otherwise the
/// index helper maps (`a`, `b`) to a target, the count helper (with 0)
/// bounds the search, and slots are scanned from 0 for the first whose
/// helper index (with the slot as its second argument) equals the target;
/// the search returns that slot's table entry, or 0 when the bound runs
/// out first.
///
/// Original: thiscall, two stack words, returns u32 in EAX. Up to two
/// distinct callees (index helper: thiscall two args; count helper:
/// thiscall one arg) across four call sites.
lf_checker_rt::export!(thiscall, rw_00A8E930(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const ENABLE_OFF: u32 = 0x76;
        const SLOTS_OFF: u32 = 0xec;
        const INDEX_HELPER: u32 = 1;
        const COUNT_HELPER: u32 = 2;
        if ((this + ENABLE_OFF) as *const u8).read() == 0 {
            return 0;
        }
        let slots = ((this + SLOTS_OFF) as *const u32).read_unaligned();
        if a == 0 {
            return (slots.wrapping_add(b.wrapping_mul(4)) as *const u32).read_unaligned();
        }
        let target: u32 = lf_checker_rt::callee_thiscall!(INDEX_HELPER, u32, this, a, b);
        let bound: u32 = lf_checker_rt::callee_thiscall!(COUNT_HELPER, u32, this, 0);
        if bound == 0 {
            return 0;
        }
        let mut slot: u32 = 0;
        loop {
            let got: u32 =
                lf_checker_rt::callee_thiscall!(INDEX_HELPER, u32, this, 0, slot);
            if got == target {
                return (slots.wrapping_add(slot.wrapping_mul(4)) as *const u32)
                    .read_unaligned();
            }
            let n: u32 = lf_checker_rt::callee_thiscall!(COUNT_HELPER, u32, this, 0);
            slot = slot.wrapping_add(1);
            if slot >= n {
                return 0;
            }
        }
    }
});
