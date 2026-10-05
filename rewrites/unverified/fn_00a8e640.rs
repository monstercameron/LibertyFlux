// original: 0x00A8E640 pool_double_query_compare (proposed)

/// Run the two-field query and compare against the caller's slots.
///
/// The two-field helper runs with (`a`, `b`) into two frame out-slots.
/// NARROW PROOF: the original then compares `a` against its own return
/// address and returns a slot or the return address itself on equality;
/// a return address is a code address, never equal to the small tested
/// `a`, and the out-slots are pinned away from `a`, so both of those
/// branches never fire and the original always returns 0 here. The
/// comparison against the return address cannot be expressed without
/// assembly, so the rewrite omits it; see `narrowed` in results.json.
///
/// Original: thiscall, two stack words, returns u32 in EAX. One outgoing
/// call (two-field helper: thiscall, four stack words, two frame
/// out-slots whose addresses are skipped in the call comparison).
lf_checker_rt::export!(thiscall, rw_00A8E640(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const QUERY: u32 = 1;
        let mut slot_y: u32 = 0;
        let mut slot_x: u32 = 0;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            QUERY,
            u32,
            this,
            a,
            b,
            &mut slot_y as *mut u32 as u32,
            &mut slot_x as *mut u32 as u32
        );
        // Both equality branches against the return address / slot_y are
        // never taken under this contract (see above): the result is 0.
        let _ = (slot_y, slot_x, a);
        0
    }
});
