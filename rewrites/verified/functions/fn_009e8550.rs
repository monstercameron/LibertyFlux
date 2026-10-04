// original: 0x009e8550 ped_slot_tag_changed
/// Compares the tag at `+4` with the live tag of the indexed slot
/// (`[this][this+8]`, word at its link `+0xA0` plus `+0x34`) and
/// reports mismatch (1) or match (0) to the shared sink, returning its
/// answer. (thiscall.)
lf_checker_rt::export!(thiscall, rw_009e8550(this_ptr: u32) -> u32 {
    unsafe {
        const SLOTS_OFF: u32 = 0;
        const TAG_OFF: u32 = 4;
        const INDEX_OFF: u32 = 8;
        const LINK_OFF: u32 = 0xA0;
        const LIVE_OFF: u32 = 0x34;
        let slots = (this_ptr.wrapping_add(SLOTS_OFF) as *const u32).read_unaligned();
        let index = (this_ptr.wrapping_add(INDEX_OFF) as *const u32).read_unaligned();
        let elem = (slots.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let tag = (this_ptr.wrapping_add(TAG_OFF) as *const u32).read_unaligned();
        let target = (elem.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        let live = (target.wrapping_add(LIVE_OFF) as *const u32).read_unaligned();
        let changed = if tag == live { 0u32 } else { 1u32 };
        lf_checker_rt::callee_thiscall!(1, u32, elem, changed)
    }
});
