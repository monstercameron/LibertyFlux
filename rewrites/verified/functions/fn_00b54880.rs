// original: 0x00B54880 crmt_list_link_10 (proposed)

/// Insert `new` into a doubly linked list just after `this`.
///
/// The link fields live at fixed offsets: the next pointer at +0x10 and
/// the previous pointer at +0x14. `new` takes over `this`'s old next
/// pointer, its previous pointer is set to `this`, and when the old next
/// pointer was non-null its previous pointer is repointed at `new`.
/// Finally `this`'s next pointer is set to `new`.
///
/// The original reads the old next pointer twice (once for the copy and
/// once for the null test) with two stores in between, so when `new`
/// overlaps `this` at an offset of -4 the second read sees the value the
/// first store wrote; the rewrite keeps both reads in the same order.
///
/// Original: 0x00B54880 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b54880(this: u32, new: u32) -> u32 {
    unsafe {
        const NEXT: u32 = 0x10;
        const PREV: u32 = 0x14;
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
