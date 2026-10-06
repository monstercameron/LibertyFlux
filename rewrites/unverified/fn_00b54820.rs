// original: 0x00B54820 crmt_list_push_10 (proposed)

/// Append `new` to the list rooted at `this`, linking through +0x10/+0x14.
///
/// The root holds a head pointer at +0x0 and a tail pointer at +0x4. When
/// the tail is non-null the link helper (callee 1) splices `new` after the
/// old tail and the tail is updated to `new`; the return value is the
/// helper's answer. When the tail is null the list is empty and both head
/// and tail are set to `new`, which is also returned.
///
/// Original: 0x00B54820 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b54820(this: u32, new: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x0;
        const TAIL: u32 = 0x4;
        const LINK: u32 = 1;
        let old = ((this + TAIL) as *const u32).read_unaligned();
        if old != 0 {
            let out: u32 = lf_checker_rt::callee_thiscall!(LINK, u32, old, new);
            ((this + TAIL) as *mut u32).write_unaligned(new);
            out
        } else {
            ((this + HEAD) as *mut u32).write_unaligned(new);
            ((this + TAIL) as *mut u32).write_unaligned(new);
            new
        }
    }
});
