// original: 0x00a94e90 stream_walk_masked_chain (proposed)

/// Walk the link chain from `[this+0x14]`, unlinking entries outside `mask`.
///
/// Entries link through the word at `+0x12` (`0xffff` means null, resolved
/// against the table base global); the walk stops at `[this+0x10]`. Each
/// visited entry whose flag word at `+0x0e` shares no bit with `mask` is
/// handed to the unlink callee with its slot number
/// (`(entry - [this]) / 24`, signed magic divide). A null start returns 0;
/// otherwise the last iteration's leftover is returned: the unlink
/// answer when it ran, else the entry's flag word. With no iterations the
/// leftover is the `lea`-scaled first link (`first * 3`), or `0xffff`
/// when the head link is null.
///
/// Original: thiscall, one stack argument (mask).
/// One callee (thiscall, 1 arg).
lf_checker_rt::export!(thiscall, rw_00a94e90(this: u32, mask: u32) -> u32 {
    unsafe {
        const CHAIN_HEAD: u32 = 0x14;
        const CHAIN_END: u32 = 0x10;
        const ENT_LINK: u32 = 0x12;
        const ENT_FLAGS: u32 = 0x0e;
        const TABLE_BASE_G: u32 = 0x012fb3a8;
        const ENTRY_STRIDE: u32 = 24;
        const NO_LINK: u32 = 0xffff;
        const UNLINK: u32 = 0;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        /// Signed divide by 24 as the original's magic sequence does it.
        fn div24(d: i32) -> i32 {
            let hi = ((d as i64 * 0x2aaaaaabi64) >> 32) as i32;
            let s = hi >> 2;
            s.wrapping_add(((s as u32) >> 31) as i32)
        }
        let start = rd32(this.wrapping_add(CHAIN_HEAD));
        if start == 0 {
            return 0;
        }
        let base = rd32(lf_checker_rt::relocated(TABLE_BASE_G));
        let end = rd32(this.wrapping_add(CHAIN_END));
        let first = rd16(start.wrapping_add(ENT_LINK));
        let mut cur = if first == NO_LINK {
            0
        } else {
            base.wrapping_add(first.wrapping_mul(ENTRY_STRIDE))
        };
        // No iterations yet: eax holds first*3 from the lea that formed
        // the entry offset (or first itself when the link was null).
        let mut seen = if first == NO_LINK {
            first
        } else {
            first.wrapping_mul(3)
        };
        let self_base = rd32(this);
        while cur != end {
            let slot = div24((cur as i32).wrapping_sub(self_base as i32));
            let next = rd16(cur.wrapping_add(ENT_LINK));
            let nxt = if next == NO_LINK {
                0
            } else {
                base.wrapping_add(next.wrapping_mul(ENTRY_STRIDE))
            };
            let flags = rd16(cur.wrapping_add(ENT_FLAGS));
            if mask & flags == 0 {
                seen = lf_checker_rt::callee_thiscall!(UNLINK, u32, this, slot as u32);
            } else {
                seen = flags;
            }
            if nxt == end {
                break;
            }
            cur = nxt;
        }
        seen
    }
});
