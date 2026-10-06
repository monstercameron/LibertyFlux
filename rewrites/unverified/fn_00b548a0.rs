// original: 0x00B548A0 crmt_list_link_8c (proposed)

/// Insert `new` into a doubly linked list just after `this`.
///
/// Same shape as the list-link routine at +0x10/+0x14 but the link fields
/// are the next pointer at +0x8C and the previous pointer at +0x90: `new`
/// takes over `this`'s old next pointer, is itself pointed back at `this`,
/// the old next node's previous pointer is repointed at `new` when
/// non-null, and `this`'s next pointer becomes `new`. Both reads of the
/// old next pointer are kept, in order, for the overlapping case.
///
/// Original: 0x00B548A0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b548a0(this: u32, new: u32) -> u32 {
    unsafe {
        const NEXT: u32 = 0x8c;
        const PREV: u32 = 0x90;
        let old_next = ((this + NEXT) as *const u32).read_unaligned();
        ((new + NEXT) as *mut u32).write_unaligned(old_next);
        ((new + PREV) as *mut u32).write_unaligned(this);
        let check = ((this + NEXT) as *const u32).read_unaligned();
        if check != 0 {
            ((check + PREV) as *mut u32).write_unaligned(new);
        }
        ((this + NEXT) as *mut u32).write_unaligned(new);
        check
    }
});
