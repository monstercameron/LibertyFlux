// original: 0x00e5fa30 timing_list_push_fa30
/// Push this unit's node onto the shared timing list.
///
/// The original reads the list head, saves the old head into this unit's
/// link slot at `0x0110F23C`, points the head at this unit's node at
/// `0x0110F238` (relocated: its immediate carries a HIGHLOW fixup), and
/// returns the old head.
lf_checker_rt::export!(cdecl, rw_00e5fa30() -> u32 {
    unsafe {
        /// Shared list head (file VA).
        const HEAD: u32 = 0x017ACD24;
        /// This unit's node linked at the head (file VA, relocated).
        const NODE: u32 = 0x0110F238;
        /// Slot holding the previous head (file VA).
        const PREV: u32 = 0x0110F23C;
        let head = lf_checker_rt::global::<u32>(HEAD);
        let old = head.read();
        lf_checker_rt::global::<u32>(PREV).write(old);
        head.write(lf_checker_rt::relocated(NODE));
        old
    }
});
