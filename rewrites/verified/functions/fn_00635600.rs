// original: 0x00635600 reloc_node_links (proposed)

/// Relocate a node's key word, its +0x34 word and both child links.
///
/// The +0x30 word is relocated in place when non-zero (relocate callee
/// with the stack argument as allocator). The +0x34 word goes through the
/// range-check/relocate pair under the thread's allocator and is zeroed
/// when the allocator is null or the check answers -1; a null word skips
/// the relocate call without being rewritten. The addresses of both child links (+0x38,
/// +0x3C) are then handed to the node-link callee in turn.
///
/// Returns `this` always.
///
/// Original: 0x00635600 (thiscall, one stack word, three callees).
lf_checker_rt::export!(thiscall, rw_00635600(this: u32, alloc: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x30;
        const WORD: u32 = 0x34;
        const LEFT: u32 = 0x38;
        const RIGHT: u32 = 0x3c;
        const TALLOC: u32 = 0x04;
        const RELOCATE: u32 = 1;
        const RANGE_CHECK: u32 = 2;
        const NODE_LINK: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if rd32(this.wrapping_add(KEY)) != 0 {
            let d: u32 = lf_checker_rt::callee_thiscall!(
                RELOCATE, u32, alloc, rd32(this.wrapping_add(KEY)));
            wr32(this.wrapping_add(KEY), rd32(this.wrapping_add(KEY)).wrapping_add(d));
        }
        let slot0 = lf_checker_rt::tls_slot(0);
        let tbl = rd32(slot0.wrapping_add(TALLOC));
        let w = this.wrapping_add(WORD);
        if tbl == 0 {
            wr32(w, 0);
        } else {
            let rc: u32 = lf_checker_rt::callee_thiscall!(RANGE_CHECK, u32, rd32(tbl), w);
            if rc == 0xFFFF_FFFF {
                wr32(w, 0);
            } else if rd32(w) != 0 {
                let d: u32 = lf_checker_rt::callee_thiscall!(RELOCATE, u32, tbl, rd32(w));
                wr32(w, rd32(w).wrapping_add(d));
            }
        }
        // The callee takes the ADDRESS of each link word (lea), not its value.
        let _l: u32 =
            lf_checker_rt::callee_thiscall!(NODE_LINK, u32, this.wrapping_add(LEFT));
        let _r: u32 =
            lf_checker_rt::callee_thiscall!(NODE_LINK, u32, this.wrapping_add(RIGHT));
        this
    }
});
