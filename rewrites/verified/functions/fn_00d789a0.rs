// original: 0x00D789A0 match_resolved_or_tagged_pair (proposed)

/// Test whether two objects match, by handle or by tag pair.
///
/// Returns true in `al` when the tag byte at `a0 + 0xe6e` is one of
/// 0xa-0xd (unsigned window after subtracting 0xa) and either the
/// resolved handle of `a0 + 0xe48` equals `a1`, or `a1` is non-null,
/// both mode words at `+0x1304` equal 1, and the tag byte at `a1 + 0xe6e`
/// is also one of 0xa-0xd. Only `al` is set: the upper 24 bits are the
/// selector answer's upper bytes, except on the first gate's fail path,
/// where they are the caller's incoming `eax` (the contract fixes it to
/// 0, so the rewrite returns 0 there). Cdecl, two stack words.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

const RESOLVE: u32 = 1;
const SELECT: u32 = 2;

export!(cdecl, rw_00d789a0(a0: u32, a1: u32) -> u32 {
    unsafe {
        const TAG_OFF: u32 = 0xe6e;
        const TAG_LO: u8 = 0x0a;
        const TAG_SPAN: u8 = 3;
        const INNER_OFF: u32 = 0xe48;
        const MODE_OFF: u32 = 0x1304;
        let tag = ((a0 + TAG_OFF) as *const u8).read();
        if tag.wrapping_sub(TAG_LO) > TAG_SPAN {
            return 0;
        }
        let t = callee_thiscall!(RESOLVE, u32, a0 + INNER_OFF);
        let r = callee_cdecl!(SELECT, u32, t);
        let base = r & 0xffff_ff00;
        if r == a1 {
            return base | 1;
        }
        if a1 == 0 {
            return base;
        }
        if ((a0 + MODE_OFF) as *const u32).read_unaligned() != 1 {
            return base;
        }
        if ((a1 + MODE_OFF) as *const u32).read_unaligned() != 1 {
            return base;
        }
        let tag2 = ((a1 + TAG_OFF) as *const u8).read();
        if tag2.wrapping_sub(TAG_LO) <= TAG_SPAN {
            base | 1
        } else {
            base
        }
    }
});
