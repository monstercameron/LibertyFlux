// original: 0x00B54850 crmt_list_push_8c (proposed)

/// Append `new` to the list rooted at `this`, linking through +0x8C/+0x90.
///
/// Same shape as the push routine at 0x00B54820: a non-null tail at +0x4
/// means `new` is spliced after it by the link helper (callee 1) and the
/// tail updated, returning the helper's answer; a null tail means an
/// empty list, so head (+0x0) and tail are both set to `new`.
///
/// Original: 0x00B54850 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b54850(this: u32, new: u32) -> u32 {
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
