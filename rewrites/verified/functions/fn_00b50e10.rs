// original: 0x00b50e10 remove_event_and_release (proposed)

/// Remove an event from a group list and release it.
///
/// `this` holds a count at `+0x08` and event pointers from `+0x0C`. The
/// list is scanned for `ev`; when absent the count is returned. When found
/// the slot is cleared and the event's release entry (virtual slot `+0x00`,
/// thiscall on the event with argument 1) runs tail-wise and its result is
/// returned. A null `ev` returns the entry residue; that path is excluded
/// from the proof (see narrowed) and the rewrite returns 0 there.
///
/// Original: 0x00b50e10 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b50e10(this: u32, ev: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x08;
        const ITEMS: u32 = 0x0c;
        const RELEASE_SLOT: u32 = 0x00;
        const RELEASE_ARG: u32 = 1;
        if ev == 0 {
            // Unreachable in the proof: the null path returns the entry
            // residue. Returning 0 keeps the rewrite total.
            return 0;
        }
        let count = ((this + COUNT) as *const u32).read_unaligned();
        let mut i: u32 = 0;
        while i < count {
            let slot = this + ITEMS + i.wrapping_mul(4);
            if (slot as *const u32).read_unaligned() == ev {
                (slot as *mut u32).write_unaligned(0);
                let vt = (ev as *const u32).read_unaligned();
                let release: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    ((vt + RELEASE_SLOT) as *const u32).read_unaligned() as usize,
                );
                return release(ev, RELEASE_ARG);
            }
            i = i.wrapping_add(1);
        }
        count
    }
});
